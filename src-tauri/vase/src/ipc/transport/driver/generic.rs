use std::{collections::HashMap, fmt::Debug, sync::Arc};

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tokio::sync::{Mutex, mpsc, oneshot};
use uuid::Uuid;

use crate::ipc::{
    Error, Result,
    envelope::TransportMode,
    layers::connection::context::ConnectionContext,
    transport::{
        driver::{engine::TransportEngine, session::Session},
        msg::TransportMessage,
        traits::{TransportAdapter, TransportForClient, TransportForServer},
    },
};

// 驱动器实例
#[allow(unused)]
pub struct GenericTransport<A: TransportAdapter> {
    pub(crate) adapter: Arc<A>,
    pub(crate) engine: Arc<TransportEngine<A>>,
}

impl<A: TransportAdapter> GenericTransport<A> {
    async fn get_active_session(&self) -> Result<Arc<Session>> {
        let mut session = None;
        for _ in 0..20 {
            // 尝试 20 次，每次 50ms，共 1秒
            let guard = self.engine.sessions.read().await;
            if let Some(s) = guard.values().next() {
                session = Some(s.clone());
                break;
            }
            drop(guard); // 重要：释放锁
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        session.ok_or(Error::SessionNotFound)
    }
}

#[async_trait]
impl<A: TransportAdapter> TransportForServer for Arc<GenericTransport<A>> {
    async fn setup(&self) -> Result<()> {
        self.engine.set_mode(TransportMode::Server)?;
        self.adapter.bind().await?; // 绑定端口
        let adapter = self.adapter.clone(); // 假设 Adapter 内部是 Arc 实现 Clone
        let engine = self.engine.clone();

        #[cfg(test)]
        println!("Starting server...");

        tokio::spawn(async move {
            while let Some(stream) = adapter.next_incoming().await {
                let (tx, rx) = mpsc::unbounded_channel();
                let session = Arc::new(Session {
                    id: Uuid::new_v4(),
                    tx,
                    inflight: Arc::new(Mutex::new(HashMap::new())),
                });

                engine
                    .sessions
                    .write()
                    .await
                    .insert(session.id, session.clone());

                let eng_weak = Arc::downgrade(&engine);
                tokio::spawn(async move {
                    session
                        .drive(
                            stream,
                            eng_weak,
                            rx,
                            #[cfg(test)]
                            "Server",
                        )
                        .await;
                });
            }
        });
        Ok(())
    }
    async fn unicast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        package: String,
        msg: TransportMessage<Pa>,
    ) -> Result<Value> {
        // 1. 查找路由
        let uuid = self
            .engine
            .routes
            .read()
            .await
            .get(&package)
            .cloned()
            .ok_or(Error::UnknownPackage(package))?;
        let session = self
            .engine
            .sessions
            .read()
            .await
            .get(&uuid)
            .cloned()
            .ok_or(Error::SessionNotFound)?;

        // 2. Pipeline
        let mut ctx = ConnectionContext::default();
        let bytes = <Self as TransportForServer>::outbound(msg, &mut ctx).await?;

        // 3. 发送 Request
        let (tx, rx) = oneshot::channel();
        session.inflight.lock().await.insert(ctx.corr, tx);
        session.tx.send(bytes).ok();

        let resp = rx
            .await
            .map_err(|_| Error::Transport("Recv failed".into()))?;

        tokio::spawn(async move {
            session.inflight.lock().await.remove(&ctx.corr);
        });

        Ok(resp)
    }

    async fn broadcast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        msg: TransportMessage<Pa>,
    ) -> Result<()> {
        // Pipeline
        let mut ctx = ConnectionContext::default();
        let bytes = <Self as TransportForServer>::outbound(msg, &mut ctx).await?;

        // 广播给所有 session
        let sessions = self.engine.sessions.read().await;
        for session in sessions.values() {
            session.tx.send(bytes.clone()).ok();
        }
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl<A: TransportAdapter> TransportForClient for Arc<GenericTransport<A>> {
    async fn setup(&self) -> Result<()> {
        self.engine.set_mode(TransportMode::Client)?;
        let adapter = self.adapter.clone(); // Adapter需支持Clone或Arc
        let engine = self.engine.clone();

        #[cfg(test)]
        println!("Starting client...");

        tokio::spawn(async move {
            loop {
                // 自动重连逻辑
                if let Ok(stream) = adapter.connect().await {
                    let (tx, rx) = mpsc::unbounded_channel();
                    let session = Arc::new(Session {
                        id: Uuid::new_v4(),
                        tx,
                        inflight: Arc::new(Mutex::new(HashMap::new())),
                    });

                    engine
                        .sessions
                        .write()
                        .await
                        .insert(session.id, session.clone());

                    // 阻塞直到连接断开
                    session
                        .drive(
                            stream,
                            Arc::downgrade(&engine),
                            rx,
                            #[cfg(test)]
                            "Client",
                        )
                        .await;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });
        Ok(())
    }

    async fn send<Pa, Resp>(
        &self,
        msg: TransportMessage<Pa>,
        mut ctx: ConnectionContext,
    ) -> Result<Resp>
    where
        Pa: Serialize + DeserializeOwned + Send + Debug,
        Resp: Serialize + DeserializeOwned + Send,
    {
        let bytes = <Self as TransportForClient>::outbound(msg, &mut ctx).await?;

        let session = self.get_active_session().await?;

        let (tx, rx) = oneshot::channel();
        session.inflight.lock().await.insert(ctx.corr, tx);
        session
            .tx
            .send(bytes)
            .map_err(|_| Error::Transport("Send failed".into()))?;

        let resp = rx
            .await
            .map_err(|_| Error::Transport("Recv failed".into()))?;

        tokio::spawn(async move {
            session.inflight.lock().await.remove(&ctx.corr);
        });

        Ok(serde_json::from_value(resp)?)
    }
    async fn shutdown(&self) -> Result<()> {
        todo!()
    }
}
