use thiserror::Error;

#[derive(Debug, Error)]
pub enum ElevationError {
    #[error("Failed to request elevation, details: {0}")]
    RequestElevationError(String),
    #[error("Failed to check elevation, details: {0}")]
    CheckElevationError(String),
}
