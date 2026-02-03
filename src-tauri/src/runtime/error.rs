use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to create script file: {0}")]
    CreateScriptError(String),
    #[error("Failed to read script file: {0}")]
    ReadScriptError(String),
}
