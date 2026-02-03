use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Failed to start up service")]
    StartupServiceError(#[from] windows_service::Error),
    #[error("Failed to install service: {0}")]
    InstallServiceError(String),
    #[error("Failed to uninstall service: {0}")]
    UninstallServiceError(String),
    #[error("Filed to query service status: {0}")]
    QueryServiceStatusError(String),
    #[error("An error occurred while the service was running: {0}")]
    ServiceInternalError(String),
    #[error("Failed to run task {0}: {1}")]
    RunTaskError(String, String),
    #[error("Failed to notify service state: {0}")]
    NotifyServiceStateError(String),
}
