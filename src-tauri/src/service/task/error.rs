use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Failed to setup task {0}: {1}")]
    SetupTaskError(String, String),
    #[error("Failed to create task {0}: {1}")]
    CreateTaskError(String, String),
    #[error("Task {0} not found")]
    TaskNotFoundError(String),
    #[error("Failed to run action {0}: {1}")]
    RunActionError(String, String),
    #[error("Failed to get task file {0}: {1}")]
    ReadTaskFileError(PathBuf, String),
    #[error("Failed to parse task file {0}: {1}")]
    ParseTaskFileError(PathBuf, String),
    #[error("Failed to update task list {0}")]
    UpdateTaskListError(String),
}
