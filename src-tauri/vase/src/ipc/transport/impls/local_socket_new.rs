use std::sync::Arc;

use async_trait::async_trait;
use interprocess::local_socket::traits::tokio::{Listener, Stream};
use interprocess::local_socket::{GenericNamespaced, ListenerOptions, ToNsName};
use tokio::sync::Mutex;
use vase_macro::transport;

use crate::ipc::{Result, transport::traits::TransportAdapter};
use interprocess::local_socket::tokio::{
    Listener as LocalSocketListener, Stream as LocalSocketStream,
};

transport!(LocalSocketTransport<LocalSocketAdapter>(address: String) {
    address,
    listener: Arc<Mutex<Option<LocalSocketListener>>> = Arc::new(Mutex::new(None))
});

#[async_trait]
impl TransportAdapter for LocalSocketAdapter {
    type Stream = LocalSocketStream;

    async fn connect(&self) -> Result<Self::Stream> {
        let ns = self
            .address
            .clone()
            .to_ns_name::<GenericNamespaced>()
            .expect("invalid IPC endpoint");
        Ok(LocalSocketStream::connect(ns).await?)
    }

    async fn bind(&self) -> Result<()> {
        let ns = self
            .address
            .clone()
            .to_ns_name::<GenericNamespaced>()
            .expect("invalid IPC endpoint");
        let listener = ListenerOptions::new().name(ns).create_tokio()?;
        let mut lock = self.listener.lock().await;
        *lock = Some(listener);
        Ok(())
    }

    async fn next_incoming(&self) -> Option<Self::Stream> {
        let mut lock = self.listener.lock().await;

        if let Some(listener) = &mut *lock {
            match listener.accept().await {
                Ok(stream) => Some(stream),
                Err(err) => {
                    eprintln!("LocalSocket accept error: {}", err);
                    // 如果是致命错误，返回 None 停止循环；如果是临时错误，可以内部递归或返回 None
                    // 这里简化处理：出错则停止
                    None
                }
            }
        } else {
            None
        }
    }
}
