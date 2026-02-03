use crate::ipc::transport::driver::engine::TransportEngine;
use crate::ipc::transport::traits::TransportAdapter;
use bytes::Bytes;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use tokio::sync::{Mutex, mpsc, oneshot};
use uuid::Uuid;

// 单个会话
pub(crate) struct Session {
    pub(crate) id: Uuid,
    // 发送通道：接收 TransportMessage，序列化后写入 Socket
    // 我们在这里使用 Bytes，意味着 Pipeline 已经在上层完成
    pub(crate) tx: mpsc::UnboundedSender<Bytes>,

    // 在途请求：RequestID -> 回调
    // 回调接收的是 Bytes (Socket 读到的原始响应)，需要再过 Inbound Pipeline
    pub(crate) inflight: Arc<Mutex<HashMap<Uuid, oneshot::Sender<Value>>>>,
}

#[allow(unused)]
impl Session {
    pub(super) async fn drive<A: TransportAdapter>(
        self: Arc<Self>,
        stream: A::Stream,
        engine: Weak<TransportEngine<A>>,
        mut rx_out: mpsc::UnboundedReceiver<Bytes>,
        #[cfg(test)] _marker: &str,
    ) {
        use futures_util::{SinkExt, StreamExt};
        use tokio_util::codec::{Framed, LengthDelimitedCodec};

        let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
        let session_id = self.id;

        loop {
            tokio::select! {
                // A. 写
                Some(bytes) = rx_out.recv() => {
                    #[cfg(test)]
                    println!("[{}]Send Bytes: {:?}", _marker, bytes);
                    if framed.send(bytes.into()).await.is_err() { break; }
                }
                // B. 读
                next = framed.next() => {
                    match next {
                        Some(Ok(bytes)) => {
                            #[cfg(test)]
                            println!("[{}]Read Bytes: {:?}", _marker, bytes);
                            let buf = bytes.freeze();
                            // 调用 Engine 处理逻辑
                            if let Some(engine) = engine.upgrade() {
                                let session = self.clone();
                                tokio::spawn(async move {
                                    engine.handle_incoming_bytes(buf, session).await;
                                });
                            }
                        }
                        _ => break,
                    }
                }
            }
        }

        // 清理
        if let Some(engine) = engine.upgrade() {
            engine.cleanup_session(session_id).await;
        }
    }
}
