use std::collections::HashMap;
use std::fmt::Debug;
use std::panic;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use futures_util::SinkExt;
use interprocess::local_socket::tokio::Stream as LocalSocketStream;
use interprocess::local_socket::traits::tokio::Stream;
use interprocess::local_socket::{
    GenericNamespaced, ListenerOptions, ToNsName, traits::tokio::Listener,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::sync::oneshot::Sender;
use tokio::sync::{Mutex, oneshot};
use tokio::time;
use tokio::{select, sync::mpsc, task::JoinSet};
use tokio_stream::StreamExt;
use tokio_util::codec::LengthDelimitedCodec;
use tokio_util::{codec::Framed, sync::CancellationToken};
use uuid::Uuid;

use crate::ipc::envelope::MsgKind;
use crate::ipc::layers::connection::context::ConnectionContext;
use crate::ipc::transport::msg::TransportMessage;
use crate::ipc::{
    self, IPC_ENDPOINT,
    envelope::{Envelope, response::Response},
    error::{Error, Result},
    transport::{
        traits::{RequestTx, Transport, TransportForClient, TransportForServer},
        utils::RequestHandler,
    },
};

enum Message {
    NoResponse(Bytes),
    WithResponse(Bytes, oneshot::Sender<Bytes>),
}

type Sessions = Arc<Mutex<HashMap<Uuid, mpsc::UnboundedSender<Message>>>>;

type Tx = mpsc::UnboundedSender<(Uuid, Bytes, oneshot::Sender<Bytes>)>;
type Rx = mpsc::UnboundedReceiver<(Uuid, Bytes, oneshot::Sender<Bytes>)>;

#[allow(dead_code)]
pub enum LocalSocketTransport {
    Server(Arc<LocalSocketServer>),
    Client(Arc<LocalSocketClient>),
    None,
}

#[allow(dead_code)]
pub struct LocalSocketServer {
    sessions: Sessions,
    route_table: Arc<Mutex<HashMap<String, Uuid>>>,
    started: AtomicBool,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: Mutex<mpsc::Receiver<()>>,
}
#[allow(dead_code)]
pub struct LocalSocketClient {
    tx: Tx,
    rx: Mutex<Rx>,
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: Mutex<mpsc::Receiver<()>>,
}

impl Transport for LocalSocketTransport {
    fn new() -> Self {
        LocalSocketTransport::None
    }
    fn client_entry(&mut self) -> impl TransportForClient {
        match self {
            LocalSocketTransport::None => {
                let (tx, rx) = mpsc::unbounded_channel();
                let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
                let client = Arc::new(LocalSocketClient {
                    tx,
                    rx: Mutex::new(rx),
                    shutdown_tx: shutdown_tx,
                    shutdown_rx: Mutex::new(shutdown_rx),
                });
                *self = LocalSocketTransport::Client(client.clone());
                client.clone()
            }
            LocalSocketTransport::Client(client) => client.clone(),
            LocalSocketTransport::Server(_) => {
                panic!("Need to call client_entry at DeviceRef")
            }
        }
    }
    fn server_entry(&mut self) -> impl TransportForServer {
        match self {
            LocalSocketTransport::None => {
                let (tx, rx) = mpsc::channel(1);
                let server = Arc::new(LocalSocketServer {
                    route_table: Arc::new(Mutex::new(HashMap::new())),
                    sessions: Arc::new(Mutex::new(HashMap::new())),
                    started: AtomicBool::new(false),
                    shutdown_tx: tx,
                    shutdown_rx: Mutex::new(rx),
                });
                *self = LocalSocketTransport::Server(server.clone());
                server.clone()
            }
            LocalSocketTransport::Server(server) => server.clone(),
            LocalSocketTransport::Client(_) => {
                panic!("Need to call server_entry at Device")
            }
        }
    }
    fn as_server(&self) -> ipc::Result<impl TransportForServer> {
        match self {
            LocalSocketTransport::Server(server) => Ok(server.clone()),
            LocalSocketTransport::None => {
                Err(ipc::Error::Transport("Call server_entry first".to_string()))
            }
            LocalSocketTransport::Client(_) => Err(ipc::Error::Transport(
                "Need to call server_entry at Device".to_string(),
            )),
        }
    }
    fn as_client(&self) -> ipc::Result<impl TransportForClient> {
        match self {
            LocalSocketTransport::Client(client) => Ok(client.clone()),
            LocalSocketTransport::Server(_) => Err(ipc::Error::Transport(
                "Need to call client_entry at DeviceRef".to_string(),
            )),
            LocalSocketTransport::None => {
                Err(ipc::Error::Transport("Call client_entry first".to_string()))
            }
        }
    }
}
#[async_trait]
impl TransportForClient for Arc<LocalSocketClient> {
    async fn setup(&self) -> crate::ipc::error::Result<()> {
        // 计算命名端点
        let ns = IPC_ENDPOINT
            .to_ns_name::<GenericNamespaced>()
            .expect("invalid IPC endpoint");
        // 后台任务：唯一拥有者，负责重连 + I/O

        let this = Arc::clone(self);
        tokio::spawn(async move {
            loop {
                // 1) 连接
                let stream = match LocalSocketStream::connect(ns.clone()).await {
                    Ok(s) => s,
                    Err(_e) => {
                        #[cfg(test)]
                        println!("failed to connect to local socket: {}", _e);
                        time::sleep(Duration::from_secs(1)).await;
                        continue;
                    }
                };
                #[cfg(test)]
                println!("socket connected");
                let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

                // 2) 会话内的在途请求表
                let mut inflight: HashMap<Uuid, oneshot::Sender<Bytes>> = HashMap::new();

                // 3) 单任务事件循环：同时处理“写（rx.recv）/读（framed.next）”
                let mut rx = this.rx.lock().await;
                let mut shutdown_rx = this.shutdown_rx.lock().await;
                let mut shutdown = false;
                loop {
                    select! {
                        _ = shutdown_rx.recv() => {
                            // 处理关闭信号
                            shutdown = true;
                            break;
                        }
                        // 写：从唯一的 rx 取待发请求
                        Some((id, req_bytes, tx_reply)) = rx.recv() => {
                            #[cfg(test)]
                            println!("sending request: id {:?}, body: {:?}", id, req_bytes);
                            // 记录在途
                            inflight.insert(id, tx_reply);

                            if let Err(_e) = framed.send(req_bytes.into()).await {
                                #[cfg(test)]
                                println!("write error: {:?}", _e);
                                // 写端断开，结束本次会话以触发重连
                                break;
                            }
                            println!("sent");
                        }
                        // 读：从 socket 收响应
                        maybe = framed.next() => {
                            println!("received");
                            match maybe {
                                Some(Ok(bytes)) => {
                                    println!("Received response");
                                    let mut bytes = bytes.freeze();
                                    let corr = Envelope::get_corr_id_from_bytes(&mut bytes);

                                    if corr.is_nil() {
                                        let mut ctx = ConnectionContext::new_broadcast();
                                        let _: Result<()> = Self::inbound(bytes, &mut ctx).await;
                                        continue;
                                    }

                                    println!("Correlation ID: {}", corr);
                                    // 根据响应里的 request_id 匹配在途请求
                                    if let Some(tx) = inflight.remove(&corr) {
                                        let _ = tx.send(bytes);
                                    } else {
                                        panic!("Unexpected response")
                                    }
                                }
                                Some(Err(err)) => {
                                    println!("Read error: {}", err);
                                    // 读端断开
                                    break;
                                }
                                None => {
                                    println!("Read error");
                                    // 读端断开
                                    break;
                                }
                            }
                        }
                    }
                }

                // 4) 会话结束：把尚未完成的请求全部报错（避免调用方永远等待）
                for (_, tx_reply) in inflight.drain() {
                    // 这里可构造一个“连接断开”的 Response/错误，示例里简单丢弃
                    if let Ok(close_resp) =
                        Envelope::from_response(Response::error("Server close".to_string()))
                    {
                        let _ = tx_reply.send(close_resp.to_bytes());
                    }
                }
                if shutdown {
                    break;
                } else {
                    time::sleep(Duration::from_millis(300)).await
                }
            }
        });
        Ok(())
    }
    async fn send<
        Pa: Serialize + DeserializeOwned + Send + Debug,
        Resp: Serialize + DeserializeOwned + Send,
    >(
        &self,
        msg: TransportMessage<Pa>,
        mut ctx: ConnectionContext,
    ) -> crate::ipc::error::Result<Resp> {
        let (tx, rx) = oneshot::channel();
        let corr = ctx.corr;
        let data = Self::outbound(msg, &mut ctx).await?;
        self.tx
            .send((corr, data, tx))
            .map_err(|e| ipc::Error::ChannelError(e.to_string()))?;
        let resp = rx
            .await
            .map_err(|_| Error::Transport("transport closed".to_string()))?;
        let resp = Self::inbound(resp, &mut ctx).await?;
        Ok(resp)
    }
    async fn shutdown(&self) -> ipc::Result<()> {
        self.shutdown_tx
            .send(())
            .await
            .map_err(|e| ipc::Error::ChannelError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl TransportForServer for Arc<LocalSocketServer> {
    async fn setup(&self) -> ipc::Result<()> {
        if self.started.swap(true, Ordering::AcqRel) {
            return Err(Error::Transport("transport already started".to_string()));
        }
        // 1) 创建监听器、通道（这些都 move 进任务）
        let listener = ListenerOptions::new()
            .name(IPC_ENDPOINT.to_ns_name::<GenericNamespaced>()?)
            .create_tokio()?;
        let (tx, rx) = mpsc::unbounded_channel::<(Bytes, oneshot::Sender<Bytes>)>();
        let shutdown = CancellationToken::new();

        // 2) 拿一个拥有所有权的 Arc<Self> 进入任务
        let this = Arc::clone(&self);

        let handle = tokio::spawn(async move {
            let mut request_handler = RequestHandler::new(rx);
            #[cfg(test)]
            println!("Starting request handler");
            request_handler
                .handle(async move |req, ctx| {
                    #[cfg(test)]
                    println!("Start handle");
                    let res = Self::inbound(req, ctx).await?;
                    let msg = TransportMessage::Response { payload: res };
                    let res = Self::outbound(msg, ctx).await?;
                    Ok(res)
                })
                .await;
        });

        tokio::spawn(async move {
            let mut conns = JoinSet::new();
            let mut shutdown_rx = this.shutdown_rx.lock().await;
            loop {
                tokio::select! {
                    _ = shutdown.cancelled() => break,
                    _ = shutdown_rx.recv() => {
                        handle.abort();
                        break;
                    },
                    res = listener.accept() => {
                        let Ok(stream) = res else {
                            if let Err(e) = res { eprintln!("accept error: {e}"); }
                            break;
                        };
                        let tx = tx.clone();
                        let sessions = this.sessions.clone();
                        let this2 = Arc::clone(&this);

                        // 这里的 handle_conn 里解码成 Bytes 后 `tx.send(bytes)` 推给上层
                        conns.spawn(async move {
                            let _ = this2.handle_conn(stream, tx, sessions).await;
                        });
                    }
                }
            }
        });

        Ok(())
    }
    async fn broadcast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        msg: TransportMessage<Pa>,
    ) -> ipc::Result<()> {
        let mut ctx = ConnectionContext::new_broadcast();
        let msg = Self::outbound(msg, &mut ctx).await?;
        let sessions = self.sessions.lock().await;
        for session in sessions.values() {
            let _ = session.send(Message::NoResponse(msg.clone()));
        }
        Ok(())
    }
    async fn unicast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        package: String,
        msg: TransportMessage<Pa>,
    ) -> ipc::Result<Value> {
        let sessions = self.sessions.lock().await;
        let route_table = &self.route_table.lock().await;
        let Some(route) = route_table.get(&package) else {
            return Err(ipc::Error::UnknownPackage(package));
        };
        let Some(session) = sessions.get(route) else {
            return Err(ipc::Error::SessionNotFound);
        };
        let mut ctx = ConnectionContext::default();
        let req = Self::outbound(msg, &mut ctx).await?;
        let (tx, rx) = oneshot::channel();
        session
            .send(Message::WithResponse(req, tx))
            .map_err(|e| ipc::Error::SendError(e.to_string()))?;
        let resp = rx
            .await
            .map_err(|e| ipc::Error::ReceiveError(e.to_string()))?;
        let resp = Self::inbound(resp, &mut ctx).await?;
        Ok(resp)
    }
    async fn shutdown(&self) -> ipc::Result<()> {
        self.shutdown_tx
            .send(())
            .await
            .map_err(|e| ipc::Error::ChannelError(e.to_string()))?;
        Ok(())
    }
}

#[allow(dead_code)]
impl LocalSocketServer {
    async fn handle_conn(
        &self,
        stream: LocalSocketStream,
        tx: RequestTx, // 发送给上层 RequestHandler 的通道 (处理来自客户端的请求)
        sessions: Sessions,
    ) -> Result<()> {
        let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

        // 1. 生成物理连接 ID
        let id = Uuid::new_v4();

        // 2. 注册会话发送端 (Server -> Client 的通道)
        let (tx_out, mut rx_out) = mpsc::unbounded_channel::<Message>();
        sessions.lock().await.insert(id, tx_out.clone());

        // 3. 定义在途请求表 (CorrelationId -> 回调通道)
        // 用于 Unicast: Server 发起请求 -> Client 响应 -> Server 收到 -> 触发回调
        let mut inflight: HashMap<Uuid, oneshot::Sender<Bytes>> = HashMap::new();

        // [Optional] 这里可以预留一个位置记录当前连接绑定的 package_name，用于断开时清理 route_table
        let mut bound_package: Option<String> = None;

        loop {
            select! {
                // A. 读取来自 Socket 的数据 (Client -> Server)
                msg = framed.next() => {
                    match msg {
                        Some(Ok(bytes)) => {
                            #[cfg(test)]
                            println!("Received raw bytes from client: {:?}", bytes);

                            let buf = bytes.clone().freeze();

                            // 尝试解析 Correlation ID (用于判断是 请求 还是 响应)
                            // 注意：Envelope::get_corr_id_from_bytes 需要你自己实现或从 envelope 模块引入
                            // 它应该从字节流头部读取 UUID，且不消耗字节流（或者在此处 clone）
                            let corr = Envelope::get_corr_id_from_bytes(&buf);
                            let msg_kind = Envelope::get_msg_kind_from_bytes(&buf);

                            // 检查这是否是我们发出的 Unicast 的响应
                            if let Some(reply_tx) = inflight.remove(&corr) {
                                // CASE 1: 这是一个响应 (Response)
                                // 找到对应的 Sender，将结果回传给 unicast 的调用者
                                let _ = reply_tx.send(buf);
                            } else if matches!(msg_kind, MsgKind::Ping) {
                                // CASE 2: 这是一个握手包
                                if bound_package.is_none() {
                                    let env = Envelope::from_bytes(buf)?;
                                    let package = env.meta.PackageName;
                                    if let Some(pkg) = package {
                                        // 绑定路由
                                        self.route_table.lock().await.insert(pkg.clone(), id);
                                        println!("Client bound package: {}", pkg);
                                        bound_package = Some(pkg);
                                    }
                                }
                            } else {
                                // CASE 3: 这是一个新请求 (Request)
                                // 发送给上层 RequestHandler 处理
                                let (tx_once, rx_once) = oneshot::channel::<Bytes>();

                                let tx_out = tx_out.clone();

                                // 这里我们传递原始 bytes 给上层
                                if let Err(_) = tx.send((buf, tx_once)) {
                                    break; // 上层 Handler 挂了
                                }

                                // 由于 Framed split 比较麻烦，简单的做法是在这里由 loop 管理写，
                                // 或者为了简单起见，我们暂且假设处理很快，或者把 rx_once 的等待放入 futures 集合管理。
                                // 但为了匹配你的代码结构，最简单的非阻塞写法是：
                                tokio::spawn(async move {
                                     if let Ok(resp_bytes) = rx_once.await {
                                         // 这里有个问题：并发写 framed 需要锁。
                                         // 更好的方案是：Request Handler 处理完后，通过 sessions 里的 tx_out 发送 Message::NoResponse 回来
                                         // 这样所有的写操作都在下面的 rx_out 分支处理，保证线程安全。

                                         // 既然 RequestHandler 的设计是 callback 形式，
                                         // 我们需要修改 RequestHandler 的逻辑，让它通过 session.send 发回响应，
                                         // 而不是通过这里的 oneshot 等待。

                                         // 但基于你目前的 RequestHandler 代码：
                                         // handle_conn -> tx -> RequestHandler -> processing -> return Ok(res) -> rx_once yields
                                         // 我们必须把这个 res 发回给 client。

                                         // 修正方案：将 rx_once 的结果发送到本函数的 rx_out 队列中
                                         let _ = tx_out.send(Message::NoResponse(resp_bytes));
                                     }
                                });

                                // [Handshake Logic Stub]
                                // 如果这是握手包，你需要解析 buf 内容，提取 package name
                                // 然后: self.route_table.lock().await.insert(pkg_name, id);
                                // bound_package = Some(pkg_name);
                            }
                        }
                        Some(Err(e)) => {
                            println!("Socket read error: {}", e);
                            break;
                        }
                        None => {
                            break; // 客户端断开
                        }
                    }
                }

                // B. 处理待发送消息 (Server -> Client)
                Some(out) = rx_out.recv() => {
                    let data;
                    match out {
                        // 1. 普通消息 / 对 Client 请求的响应
                        Message::NoResponse(bytes) => {
                            data = bytes;
                        }
                        // 2. Unicast 请求 (Server 主动调用 Client)
                        Message::WithResponse(bytes, sender) => {
                            data = bytes;
                            // 提取 Correlation ID 并记录到 inflight 表
                            // 确保 bytes 里的 header 已经包含了这个 ID (在 unicast_outbound 中生成的)
                            let mut buf = data.clone();
                            let corr = Envelope::get_corr_id_from_bytes(&mut buf);

                            if !corr.is_nil() {
                                inflight.insert(corr, sender);
                            } else {
                                // 如果没有 ID，无法追踪响应，按 NoResponse 处理或报错
                                eprintln!("Warning: Unicast message missing Correlation ID");
                            }
                        }
                    }

                    // 执行物理发送
                    if let Err(e) = framed.send(data.into()).await {
                        eprintln!("Socket write error: {}", e);
                        break;
                    }
                }
            }
        }

        // 清理工作
        {
            let mut sess = sessions.lock().await;
            sess.remove(&id);
        }

        // 清理路由表
        if let Some(pkg) = bound_package {
            let mut rt = self.route_table.lock().await;
            // 只有当路由表里的 ID 确实是当前连接时才移除（防止同名新连接被误删）
            if let Some(current_id) = rt.get(&pkg) {
                if *current_id == id {
                    rt.remove(&pkg);
                }
            }
        }

        // 终止所有在途请求
        for (_, tx) in inflight.drain() {
            // 忽略发送错误（调用方可能已经超时放弃）
            let _ = tx.send(Bytes::new()); // 或者发送特定的 Error Bytes
        }

        Ok(())
    }
}
