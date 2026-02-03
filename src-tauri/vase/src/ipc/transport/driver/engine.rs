use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use bytes::Bytes;
use serde_json::Value;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::ipc::{
    Error, Result,
    envelope::TransportMode,
    layers::connection::context::ConnectionContext,
    transport::{
        driver::{generic::GenericTransport, session::Session},
        msg::TransportMessage,
        traits::{TransportAdapter, TransportForClient, TransportForServer},
    },
};

#[allow(dead_code)]
pub(crate) struct TransportEngine<A: TransportAdapter> {
    pub(crate) sessions: RwLock<HashMap<Uuid, Arc<Session>>>,
    pub(crate) routes: Arc<RwLock<HashMap<String, Uuid>>>, // Server: Package -> Session
    pub(crate) _marker: std::marker::PhantomData<A>,
    pub(crate) mode: OnceLock<TransportMode>,
}

#[allow(dead_code)]
impl<A: TransportAdapter> TransportEngine<A> {
    pub fn new() -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            routes: Arc::new(RwLock::new(HashMap::new())),
            _marker: std::marker::PhantomData,
            mode: OnceLock::new(),
        }
    }
    pub(crate) fn set_mode(&self, mode: TransportMode) -> Result<()> {
        Ok(self
            .mode
            .set(mode)
            .map_err(|_| Error::TransportModeAlreadySet)?)
    }
    pub(crate) async fn handle_incoming_bytes(&self, buf: Bytes, session: Arc<Session>) {
        let mut ctx = ConnectionContext::default();
        ctx.inflight = Some(session.inflight.clone());
        ctx.package_routes = Some(self.routes.clone());
        ctx.session_id = session.id;

        // [关键修改] 根据模式调用不同的 inbound pipeline
        let inbound_res: Result<TransportMessage<Value>> = match self.mode.get() {
            Some(TransportMode::Server) => {
                // 服务端收到请求：调用 Server Pipeline (查找 #[handle])
                <Arc<GenericTransport<A>> as TransportForServer>::inbound(buf, &mut ctx).await
            }
            Some(TransportMode::Client) => {
                // 客户端收到请求：调用 Client Pipeline (查找 #[export])
                <Arc<GenericTransport<A>> as TransportForClient>::inbound(buf, &mut ctx).await
            }
            None => Err(Error::ErrorEntry),
        };

        match inbound_res {
            Ok(TransportMessage::Done) => (),
            Ok(resp_msg) => {
                let outbound_res = match self.mode.get() {
                    Some(TransportMode::Server) => {
                        <Arc<GenericTransport<A>> as TransportForServer>::outbound(
                            resp_msg, &mut ctx,
                        )
                        .await
                    }
                    Some(TransportMode::Client) => {
                        <Arc<GenericTransport<A>> as TransportForClient>::outbound(
                            resp_msg, &mut ctx,
                        )
                        .await
                    }
                    None => Err(Error::ErrorEntry),
                };

                if let Ok(bytes) = outbound_res {
                    let _ = session.tx.send(bytes);
                }
            }
            Err(e) => {
                eprintln!("handle_incoming_bytes error: {}", e);
            }
        }
    }
    pub(super) async fn cleanup_session(&self, session: Uuid) {
        self.sessions.write().await.remove(&session);
    }
}
