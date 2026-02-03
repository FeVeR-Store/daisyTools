use bytes::Bytes;

use crate::ipc::{self, layers::middleware::traits::Middleware};

pub struct SerdeMiddleware;

impl Middleware for SerdeMiddleware {
    type Input = Bytes;
    type Output = serde_json::Value;
    fn inbound(&self, input: Self::Input) -> ipc::error::Result<Self::Output> {
        Ok(serde_json::from_slice(&input)?)
    }
    fn outbound(&self, output: Self::Output) -> ipc::error::Result<Self::Input> {
        Ok(Bytes::from(serde_json::to_vec(&output)?))
    }
}
