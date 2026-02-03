use anyhow::anyhow;
use bytes::Bytes;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use time::OffsetDateTime;

use crate::ipc::{
    IPC_VERSION, codec,
    envelope::{Envelope, MsgKind},
    error::Error,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response<T: Serialize = Value> {
    Success {
        version: String,
        timestamp: String,
        data: T,
    },
    Error {
        version: String,
        timestamp: String,
        error: String,
    },
}

impl<'a, T: Serialize + Deserialize<'a>> Response<T> {
    pub fn payload(self) -> anyhow::Result<T> {
        match self {
            Response::Success { data, .. } => Ok(data),
            Response::Error { error, .. } => Err(anyhow!(error)),
        }
    }
    pub fn to_bytes(&self) -> Result<Bytes, Error> {
        Ok(Bytes::from(codec::encode(&self)?))
    }
}

impl Response {
    pub fn error(error: String) -> Response {
        Response::Error {
            version: IPC_VERSION.to_string(),
            timestamp: OffsetDateTime::now_utc().to_string(),
            error,
        }
    }
    pub fn success<P: Serialize>(payload: P) -> Response<P> {
        Response::Success {
            version: IPC_VERSION.to_string(),
            timestamp: OffsetDateTime::now_utc().to_string(),
            data: payload,
        }
    }
    pub fn from_bytes<Pa: Serialize + DeserializeOwned>(
        bytes: &[u8],
    ) -> Result<Response<Pa>, Error> {
        codec::decode(bytes)
    }
}

impl<T: Serialize + DeserializeOwned> TryInto<Response<T>> for Envelope {
    type Error = Error;
    fn try_into(self) -> Result<Response<T>, Self::Error> {
        match self.kind {
            MsgKind::Error => {
                let err = codec::decode(&self.payload)?;
                Err(Error::Transport(err))
            }
            MsgKind::RpcResponse => Response::from_bytes::<T>(&self.payload),
            _ => Err(Error::MismatchedTypes),
        }
    }
}
