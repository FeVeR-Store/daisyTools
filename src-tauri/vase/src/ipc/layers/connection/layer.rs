use bytes::Bytes;

use crate::ipc::{
    envelope::Envelope,
    error::Result,
    layers::{connection::context::ConnectionContext, traits::Layer},
};

pub struct ConnectionLayer;
/// Connection layer for handling connection-related operations.
///
impl Layer for ConnectionLayer {
    type In = Bytes;
    type Out = Envelope;
    async fn inbound(
        &mut self,
        input: Self::In,
        context: &mut ConnectionContext,
    ) -> Result<Self::Out> {
        let env = Envelope::from_bytes(input)?;
        context.corr = env.corr.clone();
        context.flags = env.flags.clone();
        context.meta = env.meta.clone();
        context.msg_kind = env.kind.clone();
        Ok(env)
    }
    async fn outbound(&mut self, input: Self::Out, _: &mut ConnectionContext) -> Result<Self::In> {
        let bytes = input.to_bytes();
        Ok(bytes)
    }
    fn new() -> Self {
        ConnectionLayer
    }
}
