use std::borrow::Cow;

use thiserror::Error;

use crate::ty::Data;

#[derive(Debug, Error)]
pub enum TypeConvertError {
    #[error("Failed to convert {0} to {1}")]
    ConvertError(Data, Cow<'static, str>),
    #[error("Invalid value: {0}")]
    InvalidValueError(String),
    #[error("Failed to parse plug: {0}")]
    ParsePlugError(Cow<'static, str>),
}