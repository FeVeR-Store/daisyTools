pub mod algorithm;
mod codec;
mod device;
// pub mod auth;
pub mod envelope;
pub mod error;
mod layers;
mod transport;
// pub mod southbound;
// pub mod tarits;

pub(self) const IPC_VERSION: &str = "1.0";
#[allow(dead_code)]
pub(self) const IPC_ENDPOINT: &str = "vase";

pub use error::Error;
use error::Result;

#[allow(unused)]
use layers::router::router::ExposedHandlerRegistration;
use layers::router::router::HandlerRegistration;
use layers::router::router::ListenerRegistration;

#[allow(unused)]
use envelope::TransportMode;
use transport::driver::generic::GenericTransport;
