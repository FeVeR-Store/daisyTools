use bytes::Bytes;
use vase_macro::pipeline;

use crate::ipc::{
    self,
    layers::{
        connection::context::ConnectionContext,
        middleware::{serde::SerdeMiddleware, traits::Middleware},
        traits::Layer,
    },
};

pub struct MiddlewareLayer;

impl Layer for MiddlewareLayer {
    type In = Bytes;
    type Out = serde_json::Value;
    fn new() -> Self {
        Self
    }
    async fn inbound(
        &mut self,
        x: Self::In,
        _: &mut ConnectionContext,
    ) -> ipc::error::Result<Self::Out> {
        let result = pipeline!(x -> [SerdeMiddleware] as mid {
            mid.inbound(x)?
        });
        Ok(result)
    }
    async fn outbound(
        &mut self,
        x: Self::Out,
        _: &mut ConnectionContext,
    ) -> ipc::error::Result<Self::In> {
        let result = pipeline!(x -> [SerdeMiddleware] as mid {
            mid.outbound(x)?
        });
        Ok(result)
    }
}
