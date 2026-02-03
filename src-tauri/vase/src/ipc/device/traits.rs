use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::ipc::{
    self,
    envelope::{MsgKind, flags::Flags, meta::Metadata},
    layers::connection::context::ConnectionContext,
    transport::{
        msg::TransportMessage,
        traits::{TransportForClient, TransportForServer},
    },
};

pub trait DeviceConfig: Sized {
    fn meta(&self) -> Metadata;
    fn flags(&self) -> Flags;
    fn corr(&self) -> Uuid {
        Uuid::new_v4()
    }
    fn package(&self) -> Option<&'static str>;
}

#[allow(dead_code)]
#[async_trait]
pub trait Device: DeviceConfig {
    fn transport(&self) -> ipc::Result<impl TransportForServer>;
    async fn setup(&mut self) -> ipc::Result<()>;
    async fn shutdown(self) -> ipc::Result<()>;
    async fn broadcast<Payload: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        event: String,
        payload: Payload,
    ) -> ipc::Result<()> {
        let transport = self.transport()?;
        let msg = TransportMessage::Event { event, payload };
        transport.broadcast(msg).await
    }
    async fn unicast(&self, package: &str, method: &str, payload: Value) -> ipc::Result<Value> {
        let transport = self.transport()?;
        let msg = TransportMessage::Request {
            method: method.to_string(),
            payload,
        };
        transport.unicast(package.to_string(), msg).await
    }
    async fn current() -> ipc::Result<Arc<Mutex<Self>>>;
}

#[allow(dead_code)]
#[async_trait]
pub trait DeviceRef: DeviceConfig + Send + Sync {
    fn transport(&self) -> ipc::Result<impl TransportForClient>;
    async fn setup(&mut self) -> ipc::Result<()>;
    async fn _setup(&mut self) -> ipc::Result<()> {
        if self.package().is_some() {
            self.ping().await
        } else {
            Ok(())
        }
    }
    async fn send<
        Pa: Serialize + DeserializeOwned + Send + Debug,
        Resp: Serialize + DeserializeOwned + Send + Debug,
    >(
        &self,
        msg: TransportMessage<Pa>,
        ctx: ConnectionContext,
    ) -> crate::ipc::Result<Resp> {
        use crate::ipc::transport::traits::TransportForClient;
        let transport = self.transport()?;
        Ok(TransportForClient::send(&transport, msg, ctx).await?)
    }
    async fn call<
        Pa: Serialize + DeserializeOwned + Send + Debug,
        Resp: Serialize + DeserializeOwned + Send + Debug,
    >(
        &self,
        method: String,
        payload: Pa,
    ) -> ipc::Result<Resp> {
        let mut ctx = ConnectionContext::from_device_config(self);
        ctx.msg_kind = MsgKind::RpcRequest;
        let msg = TransportMessage::Request { method, payload };
        let resp = self.send(msg, ctx).await?;
        #[cfg(test)]
        println!("Received response!!! : {:?}", resp);
        Ok(resp)
    }
    async fn ping(&self) -> ipc::Result<()> {
        let mut ctx = ConnectionContext::from_device_config(self);
        ctx.msg_kind = MsgKind::Ping;
        let msg: TransportMessage<()> = TransportMessage::Ping;
        self.send(msg, ctx).await
    }
    async fn shutdown(self) -> ipc::Result<()>;
    async fn current() -> ipc::Result<Arc<Mutex<Self>>>;
}
