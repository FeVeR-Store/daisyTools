use crate::ipc::{
    self, layers::connection::context::ConnectionContext, transport::traits::RequestRx,
};
use bytes::Bytes;

#[allow(dead_code)]
pub struct RequestHandler {
    rx: RequestRx,
}

#[allow(dead_code)]
impl RequestHandler {
    pub fn new(rx: RequestRx) -> Self {
        Self { rx }
    }
    pub async fn handle<F>(&mut self, handler: F)
    where
        F: AsyncFn(Bytes, &mut ConnectionContext) -> ipc::error::Result<Bytes>,
    {
        while let Some((msg, tx)) = self.rx.recv().await {
            let mut connect_context = ConnectionContext::default();
            if let Ok(res) = handler(msg, &mut connect_context).await {
                let _ = tx.send(res);
            };
        }
    }
}
