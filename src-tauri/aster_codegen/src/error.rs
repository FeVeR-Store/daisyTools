use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Path not found: {0}")]
    PathNotExist(PathBuf),
    #[error("Failed to parse template file: {0}")]
    ParseTemplateError(String),
    #[error("Failed to write to file {0} : {1}")]
    WriteFileError(PathBuf, String),
    #[error("Failed to emit TypeScript code: {0}")]
    EmitTsCodeError(String),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
