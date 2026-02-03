use crate::ipc::{error::Result, layers::connection::context::ConnectionContext};

pub trait Layer {
    type In;
    type Out;

    async fn inbound(&mut self, x: Self::In, ctx: &mut ConnectionContext) -> Result<Self::Out>;
    async fn outbound(&mut self, x: Self::Out, ctx: &mut ConnectionContext) -> Result<Self::In>;

    fn new() -> Self;
}
