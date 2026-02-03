// 放在 tests/transport_test.rs 或 crate/ipc/transport/driver/tests.rs 中

use crate::ipc::Result;
use crate::ipc::layers::connection::context::ConnectionContext;
use crate::ipc::transport::driver::generic::GenericTransport;
use crate::ipc::transport::msg::TransportMessage;
use crate::ipc::transport::traits::{TransportAdapter, TransportForClient, TransportForServer};
use async_channel::{Receiver, Sender, unbounded};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::io::{DuplexStream, duplex};

// --- Mock Adapter ---

#[derive(Clone)]
struct MockAdapter {
    // 用于模拟 "网络"：Client connect 时把一端塞进去，Server next_incoming 时取出来
    // 使用 async_channel 支持多对多，且 Clone 开销小
    network: Sender<DuplexStream>,
    incoming: Receiver<DuplexStream>,
}

impl MockAdapter {
    fn pair() -> (Self, Self) {
        let (tx, rx) = unbounded();
        // Server 只有 rx (用于 accept), Client 只有 tx (用于 connect)
        // 但为了 TransportAdapter 接口统一，我们让结构体同时持有 tx/rx，
        // 只是 Server 用 rx，Client 用 tx。
        let adapter = MockAdapter {
            network: tx,
            incoming: rx,
        };
        (adapter.clone(), adapter)
    }
}
#[async_trait]
impl TransportAdapter for MockAdapter {
    type Stream = DuplexStream;

    async fn connect(&self) -> Result<Self::Stream> {
        // 模拟 connect: 创建一对流
        let (client_end, server_end) = duplex(1024 * 64); // 64KB buffer
        // 把 server 端扔进 "网络"
        self.network.send(server_end).await.expect("Network down");
        // 返回 client 端
        Ok(client_end)
    }

    async fn next_incoming(&self) -> Option<Self::Stream> {
        // 模拟 accept: 从 "网络" 获取连接请求
        self.incoming.recv().await.ok()
    }

    async fn bind(&self) -> Result<()> {
        Ok(())
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::ipc::envelope::meta::{Meta, Metadata};

//     use super::*;
//     use serde_json::json;
//     use std::time::Duration;

//     // 辅助函数：创建 Server 和 Client
//     async fn create_pair() -> (GenericTransport<MockAdapter>, GenericTransport<MockAdapter>) {
//         let (server_adapter, client_adapter) = MockAdapter::pair();

//         let server = GenericTransport::new_server(server_adapter);
//         let client = GenericTransport::new_client(client_adapter);

//         (server, client)
//     }

//     #[tokio::test]
//     async fn test_handshake_and_connection() {
//         let (server, client) = create_pair().await;

//         // 1. 启动
//         TransportForServer::setup(&server).await.unwrap();
//         TransportForClient::setup(&client).await.unwrap();

//         // 等待连接建立和握手完成 (内存操作很快，100ms足够)
//         tokio::time::sleep(Duration::from_millis(100)).await;

//         // 2. 创建一个带有Package的ctx，并发送Ping请求
//         let mut ctx = ConnectionContext::default();
//         ctx.meta = Metadata::default() + Meta::PackageName("com.test.client".to_string());
//         let _: () = TransportForClient::send(&client, TransportMessage::Ping::<()>, ctx)
//             .await
//             .unwrap();

//         // 3. 验证 Server 端的路由表
//         let routes = server.engine.routes.read().await;
//         assert!(
//             routes.contains_key("com.test.client"),
//             "Server should have registered the client package"
//         );
//     }

//     #[tokio::test]
//     async fn test_client_call_server_rpc() {
//         let (server, client) = create_pair().await;
//         TransportForServer::setup(&server).await.unwrap();
//         TransportForClient::setup(&client).await.unwrap();

//         tokio::time::sleep(Duration::from_millis(50)).await;

//         // 构造请求
//         let req_msg = TransportMessage::Request {
//             method: "echo".to_string(),
//             payload: json!({ "msg": "hello" }),
//         };

//         let ctx = ConnectionContext::default();

//         // 发送并等待响应
//         // 注意：我们在 mock_inbound 里写了简单的 echo 逻辑
//         let resp: serde_json::Value = client.send(req_msg, ctx).await.expect("RPC call failed");

//         assert_eq!(resp["msg"], "hello");
//     }

//     #[tokio::test]
//     async fn test_server_unicast_client() {
//         let (server, client) = create_pair().await;

//         TransportForServer::setup(&server).await.unwrap();
//         TransportForClient::setup(&client).await.unwrap();

//         // 等待握手
//         tokio::time::sleep(Duration::from_millis(100)).await;

//         let mut ctx = ConnectionContext::default();
//         ctx.meta = Metadata::default() + Meta::PackageName("com.test.client".to_string());
//         let _: () = TransportForClient::send(&client, TransportMessage::Ping::<()>, ctx)
//             .await
//             .unwrap();

//         // Server 反向调用 Client
//         let msg = TransportMessage::Request {
//             method: "do_job".to_string(),
//             payload: json!({ "job_id": 101 }),
//         };

//         // 我们的 mock_inbound 同样会对这个请求做 Echo 返回
//         let resp = server
//             .unicast("worker.1".to_string(), msg)
//             .await
//             .expect("Unicast failed");

//         assert_eq!(resp["job_id"], 101);
//     }

//     #[tokio::test]
//     async fn test_concurrency_stress() {
//         // 并发压力测试：验证 Inflight Map 和 Session 锁是否正常工作
//         let (server, client) = create_pair().await;
//         TransportForServer::setup(&server).await.unwrap();
//         TransportForClient::setup(&client).await.unwrap();
//         tokio::time::sleep(Duration::from_millis(50)).await;

//         let client = Arc::new(client);
//         let mut handles = vec![];

//         for i in 0..100 {
//             let c = client.clone();
//             handles.push(tokio::spawn(async move {
//                 let req = TransportMessage::Request {
//                     method: "ping".to_string(),
//                     payload: json!(i),
//                 };
//                 let resp: serde_json::Value =
//                     c.send(req, ConnectionContext::default()).await.unwrap();
//                 assert_eq!(resp.as_i64(), Some(i));
//             }));
//         }

//         for h in handles {
//             h.await.unwrap();
//         }
//     }
// }
