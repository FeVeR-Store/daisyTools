use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum TransportMessage<Pa: Serialize + DeserializeOwned + Send> {
    Request { method: String, payload: Pa },
    Response { payload: Pa },
    Event { event: String, payload: Pa },
    Ping,
    Pong,
    Done,
}
