pub mod error;
pub mod type_convert;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Data {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Json(Map<String, Value>),
    Vec(Vec<Value>),
    Any(Value),
    Null,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct ErrorWrap {
    pub inner: Box<dyn std::fmt::Debug>,
    pub branch: String,
}

impl std::fmt::Display for ErrorWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}
impl std::error::Error for ErrorWrap {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardResult {
    pub variant: &'static str,
    pub data: Data,
}
