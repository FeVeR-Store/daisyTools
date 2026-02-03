use std::{env, io};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O Error")]
    IoError(#[from] io::Error),
    #[error("I/O Error")]
    WalkDirError(#[from] walkdir::Error),
    #[error("Failed to parse vase scan index")]
    ParseIndexError(#[from] csv::Error),
    #[error("Failed to find env var")]
    EnvVarNotFoundError(#[from] env::VarError),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
