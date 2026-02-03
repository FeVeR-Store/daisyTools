use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Failed to light up action card: {0}")]
    LitActionCardError(String),
    #[error("Failed to run action: {0}")]
    RunActionCardError(String),
    #[error("Failed to remove action {0}: {1}")]
    RemoveActionError(String,String),
}
