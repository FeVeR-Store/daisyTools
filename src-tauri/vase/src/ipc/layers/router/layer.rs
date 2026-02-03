use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::ipc::{
    Error, Result,
    envelope::{
        MsgKind, TransportMode,
        request::{Request, RequestSource},
        response::Response,
    },
    layers::{
        connection::context::ConnectionContext,
        router::router::{EXPOSED_ROUTES, HANDLE_ROUTES, LISTENERS},
        traits::Layer,
    },
    transport::msg::TransportMessage,
};

pub struct RouterLayer<Resp: Serialize + DeserializeOwned + Send>(PhantomData<Resp>, TransportMode);

impl<Resp: Serialize + DeserializeOwned + Send> RouterLayer<Resp> {
    pub fn server() -> Self {
        Self(PhantomData, TransportMode::Server)
    }
    pub fn client() -> Self {
        Self(PhantomData, TransportMode::Client)
    }
}

impl<Resp: Serialize + DeserializeOwned + Send> Layer for RouterLayer<Resp> {
    type In = serde_json::Value;
    type Out = TransportMessage<Resp>;

    fn new() -> Self {
        Self::server()
    }

    async fn inbound(&mut self, x: Self::In, ctx: &mut ConnectionContext) -> Result<Self::Out> {
        #[cfg(test)]
        println!("router layer");

        match ctx.msg_kind {
            kind @ (MsgKind::Event | MsgKind::RpcRequest) => {
                let req: Request = serde_json::from_value(x)?;
                let method = req.method;
                let payload = req.data;

                #[cfg(test)]
                println!("method: {}", method);
                #[cfg(test)]
                println!("payload: {:?}", payload);

                match (kind, self.1) {
                    (
                        MsgKind::RpcRequest,
                        mode @ (TransportMode::Client | TransportMode::Server),
                    ) => {
                        let routes = match mode {
                            TransportMode::Client => &EXPOSED_ROUTES,
                            TransportMode::Server => &HANDLE_ROUTES,
                        };
                        let handle = routes
                            .get(method.as_str())
                            .ok_or(Error::RouteNotFound(method.to_string()))?;
                        let res = handle(payload).await?;
                        Ok(TransportMessage::Response {
                            payload: serde_json::from_value(res)?,
                        })
                    }
                    (MsgKind::Event, _) => {
                        let listeners = &LISTENERS
                            .get(method.as_str())
                            .ok_or(Error::RouteNotFound(method.to_string()))?;
                        for listener in listeners.iter() {
                            let _ = listener(payload.clone()).await;
                        }
                        Ok(TransportMessage::Done)
                    }
                    _ => panic!("Invalid message kind"),
                }
            }

            MsgKind::RpcResponse => {
                let resp: Response<Resp> = serde_json::from_value(x)?;
                let inflight = &ctx.inflight.clone().ok_or(Error::SessionNotFound)?;

                if let Some(tx) = inflight.lock().await.remove(&ctx.corr) {
                    tx.send(serde_json::to_value(resp.payload()?)?)
                        .map_err(|e| Error::SendError(e.to_string()))?;
                }
                Ok(TransportMessage::Done)
            }
            MsgKind::Ping if matches!(self.1, TransportMode::Server) => {
                let package_name =
                    ctx.meta
                        .PackageName
                        .clone()
                        .ok_or(Error::RegisterPackageError(
                            "No package name in metadata".to_string(),
                        ))?;
                let package_routes =
                    ctx.package_routes
                        .clone()
                        .ok_or(Error::RegisterPackageError(
                            "No package routes in connection context".to_string(),
                        ))?;
                package_routes
                    .write()
                    .await
                    .insert(package_name, ctx.session_id);
                Ok(TransportMessage::Pong)
            }
            MsgKind::Pong if matches!(self.1, TransportMode::Client) => {
                let inflight = &ctx.inflight.clone().ok_or(Error::SessionNotFound)?;

                if let Some(tx) = inflight.lock().await.remove(&ctx.corr) {
                    tx.send(Value::Null)
                        .map_err(|e| Error::SendError(e.to_string()))?;
                }
                Ok(TransportMessage::Done)
            }
            MsgKind::ChangeServer => todo!("ChangeServer not impl"),
            _ => Err(Error::UnsupportedMessageType),
        }
    }
    async fn outbound(&mut self, msg: Self::Out, ctx: &mut ConnectionContext) -> Result<Self::In> {
        let send_val = match msg {
            TransportMessage::Request { method, payload } => {
                ctx.msg_kind = MsgKind::RpcRequest;
                serde_json::to_value(Request::new(
                    payload,
                    match self.1 {
                        TransportMode::Client => RequestSource::Bud,
                        TransportMode::Server => RequestSource::Vase,
                    },
                    method,
                ))?
            }
            TransportMessage::Response { payload } => {
                ctx.msg_kind = MsgKind::RpcResponse;
                serde_json::to_value(Response::success(payload))?
            }
            TransportMessage::Event { event, payload } => {
                ctx.msg_kind = MsgKind::Event;
                serde_json::to_value(Request::event(payload, event))?
            }
            TransportMessage::Ping => {
                ctx.msg_kind = MsgKind::Ping;
                Value::Null
            }
            TransportMessage::Pong => {
                ctx.msg_kind = MsgKind::Pong;
                Value::Null
            }
            _ => return Err(Error::UnsupportedMessageType),
        };
        Ok(send_val)
    }
}
