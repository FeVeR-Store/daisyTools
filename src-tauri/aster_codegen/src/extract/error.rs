use std::io;

use thiserror::Error;
use toml;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read file")]
    IOError(#[from] io::Error),
    #[error("failed to parse toml")]
    ParseTomlError(#[from] toml::de::Error),
}

pub(in crate::extract) type Result<R> = std::result::Result<R, Error>;
