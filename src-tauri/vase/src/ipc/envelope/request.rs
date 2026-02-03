use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::ipc::{self, codec, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestSource {
    Vase,
    Bud,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMetadata {
    vase_version: String,
    additional: HashMap<String, Option<String>>,
}

impl IndexMut<String> for RequestMetadata {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        if !self.additional.contains_key(&index) {
            let value = None;
            self.additional.insert(index.clone(), value);
        }
        self.additional.get_mut(&index).unwrap()
    }
}
impl<'a> Index<String> for RequestMetadata {
    type Output = Option<String>;
    fn index(&'_ self, index: String) -> &'_ Self::Output {
        self.additional.get(&index).unwrap_or_else(|| &None)
    }
}

impl RequestMetadata {
    fn new() -> Self {
        Self {
            vase_version: env!("CARGO_PKG_VERSION").to_string(),
            additional: HashMap::new(),
        }
    }
    pub fn metadata(&mut self) -> &mut Self {
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request<T: Serialize = Value> {
    pub version: String,
    pub device_id: Option<Uuid>,
    pub timestamp: String,
    pub source: RequestSource,
    pub method: String,
    pub data: T,
    pub metadata: RequestMetadata,
}

impl<'a, T: Serialize> Request<T> {
    pub fn new(data: T, source: RequestSource, method: impl Into<String>) -> Self {
        Request {
            version: "1.0".to_string(),
            device_id: None,
            data,
            timestamp: OffsetDateTime::now_utc().to_string(),
            source,
            metadata: RequestMetadata::new(),
            method: method.into(),
        }
    }
    pub fn event(payload: T, event: String) -> Self {
        Self::new(payload, RequestSource::Vase, event)
    }
    pub fn to_bytes(&self) -> ipc::Result<Bytes> {
        Ok(Bytes::from(codec::encode(&self)?))
    }
}

impl Request {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        codec::decode(&bytes)
    }
}

pub struct CoreRequest<T: Serialize = Value> {
    pub data: T,
    pub metadata: RequestMetadata,
}

impl<T: Serialize> Into<CoreRequest<T>> for Request<T> {
    fn into(self) -> CoreRequest<T> {
        CoreRequest {
            data: self.data,
            metadata: self.metadata,
        }
    }
}
