use std::fmt::Debug;

use async_trait::async_trait;
use bytes::Bytes;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot,
};
use vase_macro::pipeline;

use crate::MacroInit;
use crate::ipc::{
    Result,
    layers::{
        ConnectionLayer, Layer, MiddlewareLayer, RouterLayer, StrategyLayer,
        connection::context::ConnectionContext,
    },
    transport::msg::TransportMessage,
};
#[allow(dead_code)]
pub type RequestRx = UnboundedReceiver<(Bytes, oneshot::Sender<Bytes>)>;
#[allow(dead_code)]
pub type RequestTx = UnboundedSender<(Bytes, oneshot::Sender<Bytes>)>;

pub trait IoStream: AsyncRead + AsyncWrite + Send + Unpin + 'static {}

impl<T: AsyncRead + AsyncWrite + Send + Unpin + 'static> IoStream for T {}

#[async_trait]
pub trait TransportAdapter: Send + Sync + 'static {
    type Stream: IoStream;

    async fn connect(&self) -> Result<Self::Stream>;
    async fn next_incoming(&self) -> Option<Self::Stream>;
    async fn bind(&self) -> Result<()>;
}

#[allow(dead_code)]
pub trait Transport {
    fn new() -> Self;
    async fn client_entry(&mut self) -> Result<()>;
    async fn server_entry(&mut self) -> Result<()>;
    fn as_client(&self) -> Result<impl TransportForClient + Send + Sync>;
    fn as_server(&self) -> Result<impl TransportForServer + Send + Sync>;
}

impl<T: TransportForClient + TransportForServer + MacroInit + Clone> Transport for T {
    fn new() -> Self {
        MacroInit::new()
    }
    async fn client_entry(&mut self) -> Result<()> {
        TransportForClient::setup(self).await?;
        Ok(())
    }
    async fn server_entry(&mut self) -> Result<()> {
        TransportForServer::setup(self).await?;
        Ok(())
    }
    fn as_client(&self) -> Result<impl TransportForClient + Send + Sync> {
        Ok(self.clone())
    }
    fn as_server(&self) -> Result<impl TransportForServer + Send + Sync> {
        Ok(self.clone())
    }
}

#[allow(dead_code)]
#[async_trait]
pub trait TransportForServer: Send + Sync + 'static {
    async fn setup(&self) -> Result<()>;
    async fn broadcast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        data: TransportMessage<Pa>,
    ) -> Result<()>;
    async fn unicast<Pa: Serialize + DeserializeOwned + Send + Debug>(
        &self,
        package: String,
        data: TransportMessage<Pa>,
    ) -> Result<Value>;
    async fn shutdown(&self) -> Result<()>;

    async fn inbound<Resp: Serialize + DeserializeOwned + Send + Debug>(
        buf: Bytes,
        ctx: &mut ConnectionContext,
    ) -> Result<TransportMessage<Resp>> {
        Ok(
            pipeline!(buf -> [ConnectionLayer::new(), StrategyLayer::new(), MiddlewareLayer::new(), RouterLayer::server()] as mut layer {
                #[cfg(test)]
                println!("{:?} inbound:", ctx.msg_kind);

                dbg!(layer.inbound(buf, ctx).await?)
            }),
        )
    }
    async fn outbound<Pa: Serialize + DeserializeOwned + Send + Debug>(
        msg: TransportMessage<Pa>,
        ctx: &mut ConnectionContext,
    ) -> Result<Bytes> {
        Ok(
            pipeline!(msg -> [RouterLayer::server(), MiddlewareLayer::new(), StrategyLayer::new(), ConnectionLayer::new()] as mut layer {
                #[cfg(test)]
                println!("{:?} outbound: ", ctx.msg_kind);
                dbg!(layer.outbound(msg, ctx).await?)
            }),
        )
    }
}

#[allow(dead_code)]
#[async_trait]
pub trait TransportForClient: Send + Sync + 'static {
    async fn setup(&self) -> Result<()>;
    async fn send<
        Pa: Serialize + DeserializeOwned + Send + Debug,
        Resp: Serialize + DeserializeOwned + Send,
    >(
        &self,
        data: TransportMessage<Pa>,
        ctx: ConnectionContext,
    ) -> Result<Resp>;
    async fn inbound<Resp: Serialize + DeserializeOwned + Send + Debug>(
        buf: Bytes,
        ctx: &mut ConnectionContext,
    ) -> Result<TransportMessage<Resp>> {
        Ok(
            pipeline!(buf -> [ConnectionLayer::new(), StrategyLayer::new(), MiddlewareLayer::new(), RouterLayer::client()] as mut layer {
                #[cfg(test)]
                println!("{:?} inbound: ", ctx.msg_kind);
                dbg!(layer.inbound(buf, ctx).await?)
            }),
        )
    }
    async fn outbound<Pa: Serialize + DeserializeOwned + Send + Debug>(
        msg: TransportMessage<Pa>,
        ctx: &mut ConnectionContext,
    ) -> Result<Bytes> {
        Ok(
            pipeline!(msg -> [RouterLayer::client(), MiddlewareLayer::new(), StrategyLayer::new(), ConnectionLayer::new()] as mut layer {
                #[cfg(test)]
                println!("{:?} outbound:", ctx.msg_kind);
                dbg!(layer.outbound(msg, ctx).await?)
            }),
        )
    }
    async fn shutdown(&self) -> Result<()>;
}
