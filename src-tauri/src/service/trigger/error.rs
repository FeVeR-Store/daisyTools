use thiserror::Error;

#[derive(Debug, Error)]
pub enum TriggerError {
    #[error("Failed to register trigger: {0}")]
    RegisrterTriggerError(String),
    #[error("Failed to setup trigger: {0}")]
    SetupTriggerError(String),
    #[error("Failed to run action {0}: {1}")]
    RunActionError(String, String),
    #[error("Failed to find trigger {0}")]
    FindTriggerError(String),
    #[error("Failed to remove trigger {0}: {1}")]
    RemoveTriggerError(String, String),
    #[error("Failed to run task {0}: {1}")]
    RunTaskError(String, String),
}
