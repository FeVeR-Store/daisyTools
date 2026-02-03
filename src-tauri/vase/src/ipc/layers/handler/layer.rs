use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::ipc::{
    self, Error, Result,
    envelope::{
        request::{Request, RequestSource},
        response::Response,
    },
    layers::connection::context::ConnectionContext,
};

pub struct HandlerLayer;

impl HandlerLayer {
    pub fn new() -> Self {
        Self
    }
    pub async fn inbound<N: Serialize + DeserializeOwned>(
        &mut self,
        x: serde_json::Value,
        ctx: &mut ConnectionContext,
    ) -> Result<N> {
        let resp: Response<N> = serde_json::from_value(x)?;
        if let Some(inflight) = &ctx.inflight {
            if let Some(tx) = inflight.lock().await.remove(&ctx.corr) {
                tx.send(serde_json::to_value(resp.payload()?)?)
                    .map_err(|e| Error::SendError(e.to_string()))?;
            }
            Err(Error::TakenOver)
        } else {
            Ok(resp.payload()?)
        }
    }
    pub async fn outbound<S: Serialize + DeserializeOwned>(
        &mut self,
        x: S,
        _: &mut ConnectionContext,
    ) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(x)?)
    }
}
