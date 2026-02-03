use thiserror::Error;

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Message format error: {0}")]
    MessageFormat(String),

    #[error("Custom error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, IpcError>;

// 便捷方法用于创建错误
impl IpcError {
    pub fn connection_error(msg: impl Into<String>) -> Self {
        Self::Connection(msg.into())
    }

    pub fn protocol_error(msg: impl Into<String>) -> Self {
        Self::Protocol(msg.into())
    }

    pub fn timeout_error(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }

    pub fn message_format_error(msg: impl Into<String>) -> Self {
        Self::MessageFormat(msg.into())
    }

    pub fn custom_error(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}