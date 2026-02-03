use tokio::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("tokio I/O error")]
    IOError(#[from] io::Error),
    #[error("unauthorized")]
    Unauthorized,
    #[error("route not found: {0}")]
    RouteNotFound(String),
    #[error("codec: {0}")]
    Encode(#[from] bincode::error::EncodeError),
    #[error("codec: {0}")]
    Decode(#[from] bincode::error::DecodeError),
    #[error("transport: {0}")]
    Transport(String),
    #[error("crypto: {0}")]
    Crypto(String),
    #[error("internal: {0}")]
    Internal(String),
    #[error("timeout")]
    Timeout,
    #[error("aborted")]
    Aborted,
    #[error("failed to send message: {0}")]
    SendError(String),
    #[error("failed to receive message: {0}")]
    ReceiveError(String),
    #[error("mismatched types")]
    MismatchedTypes,
    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("router error: {0}")]
    RouterError(#[from] anyhow::Error),
    #[error("error entry")]
    ErrorEntry,
    #[error("channel error")]
    ChannelError(String),
    #[error("unknown package: {0}")]
    UnknownPackage(String),
    #[error("session not found")]
    SessionNotFound,
    #[error("unsupported message type")]
    UnsupportedMessageType,
    #[error("transport mode is already initialized")]
    TransportModeAlreadySet,
    #[error("register package error: {0}")]
    RegisterPackageError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
