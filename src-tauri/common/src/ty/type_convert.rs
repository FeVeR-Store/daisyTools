use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value, json};

use crate::ty::{Data, error::TypeConvertError};

impl Data {
    pub fn value(self) -> Value {
        match self {
            Data::Any(val) => val,
            Data::Bool(val) => Value::Bool(val),
            Data::Float(val) => json!(val),
            Data::Int(val) => Value::Number(val.into()),
            Data::Json(val) => Value::Object(val),
            Data::Null => Value::Null,
            Data::String(val) => Value::String(val),
            Data::Vec(val) => Value::Array(val),
        }
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Self::Json(map) => map.get(key),
            Self::Any(val) => val.as_object().and_then(|m| m.get(key)),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Result<i64, TypeConvertError> {
        if let Self::Int(value) = self {
            Ok(*value)
        } else {
            Err(TypeConvertError::ConvertError(self.clone(), "int".into()))
        }
    }
    pub fn as_float(&self) -> Result<f64, TypeConvertError> {
        if let Self::Float(value) = self {
            Ok(*value)
        } else {
            Err(TypeConvertError::ConvertError(self.clone(), "float".into()))
        }
    }
    pub fn as_string(&self) -> Result<String, TypeConvertError> {
        if let Self::String(value) = self {
            Ok(value.clone())
        } else {
            Err(TypeConvertError::ConvertError(
                self.clone(),
                "string".into(),
            ))
        }
    }
    pub fn as_bool(&self) -> Result<bool, TypeConvertError> {
        if let Self::Bool(value) = self {
            Ok(*value)
        } else {
            Err(TypeConvertError::ConvertError(self.clone(), "bool".into()))
        }
    }
    pub fn as_vec(&self) -> Result<Vec<Value>, TypeConvertError> {
        if let Self::Vec(value) = self {
            Ok(value.clone())
        } else {
            Err(TypeConvertError::ConvertError(self.clone(), "vec".into()))
        }
    }
    pub fn to_value(&self) -> Value {
        match self {
            Self::Bool(bool) => json!(bool),
            Self::Any(value) => value.clone(),
            Self::Float(f) => json!(f),
            Self::Int(i) => json!(i),
            Self::Json(map) => json!(map),
            Self::Null => Value::Null,
            Self::String(str) => json!(str),
            Self::Vec(vec) => json!(vec),
        }
    }
    pub fn as_json(&self) -> Result<Map<String, Value>, TypeConvertError> {
        if let Self::Json(value) = self {
            Ok(value.clone())
        } else {
            Err(TypeConvertError::ConvertError(self.clone(), "json".into()))
        }
    }
    pub fn r#as<T>(&self) -> Result<T, TypeConvertError>
    where
        T: DeserializeOwned,
    {
        match self {
            Data::Any(value) => serde_json::from_value(value.clone()).map_err(|_| {
                TypeConvertError::ConvertError(self.clone(), "the type you want".into())
            }),
            Data::Json(value) => {
                serde_json::from_value(Value::Object(value.clone())).map_err(|_| {
                    TypeConvertError::ConvertError(self.clone(), "the type you want".into())
                })
            }
            _ => Err(TypeConvertError::ConvertError(
                self.clone(),
                "the type you want".into(),
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plug {
    pub r#type: String,
    pub value: Vec<String>,
}

pub trait FromString {
    fn from_str(string: &str) -> Result<Self, TypeConvertError>
    where
        Self: Sized;
}

pub fn parse_data(
    context: &HashMap<String, Data>,
    card_data: Data,
) -> Result<Data, TypeConvertError> {
    let Ok(data) = card_data.r#as::<serde_json::Map<String, Value>>() else {
        return Ok(card_data);
    };
    for ref mut val in data.values() {
        let res = serde_json::from_value::<Plug>(val.clone());
        let Ok(plug) = res else {
            continue;
        };
        let mut iter = plug.value.iter();
        let Some(ctx) =
            context.get(iter.next().ok_or_else(|| {
                TypeConvertError::ParsePlugError(Cow::Borrowed("Plug id is empty"))
            })?)
        else {
            return Err(TypeConvertError::ParsePlugError(Cow::Borrowed(
                "The value for plug was not found in the context",
            )));
        };
        let mut current_value = &ctx.clone().value();
        while let Some(key) = iter.next() {
            current_value = current_value.get(key).ok_or_else(|| {
                TypeConvertError::ParsePlugError(
                    format!(
                    "The key {} does not exist in the context, and the current value read is {:?}",
                    &key, &current_value
                )
                    .into(),
                )
            })?;
        }
        *val = &current_value;
    }
    Ok(Data::Json(data))
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for PathBuf {
    fn to_string(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
impl ToString for Path {
    fn to_string(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl ToString for Map<String, Value> {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(windows)]
impl ToString for windows_service::service::ServiceState {
    fn to_string(&self) -> String {
        let str = match *self {
            windows_service::service::ServiceState::Stopped => "Stopped",
            windows_service::service::ServiceState::StartPending => "StartPending",
            windows_service::service::ServiceState::StopPending => "StopPending",
            windows_service::service::ServiceState::Running => "Running",
            windows_service::service::ServiceState::ContinuePending => "ContinuePending",
            windows_service::service::ServiceState::PausePending => "PausePending",
            windows_service::service::ServiceState::Paused => "Paused",
        };
        str.to_string()
    }
}

/* use tauri_plugin_http::reqwest::Method;

pub trait FromString {
    fn from_str(string: &str) -> Result<Self, TypeConvertError>
    where
        Self: Sized;
}

impl FromString for Method {
    fn from_str(method_str: &str) -> Result<Self, TypeConvertError> {
        let method_str: &str = &method_str.to_uppercase();
        let method = match method_str {
            "CONNECT" => Method::CONNECT,
            "DELETE" => Method::DELETE,
            "POST" => Method::POST,
            "GET" => Method::GET,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "PATCH" => Method::PATCH,
            "PUT" => Method::PUT,
            "TRACE" => Method::TRACE,
            _ => return Err(TypeConvertError::InvalidValueError(method_str.to_string())),
        };
        Ok(method)
    }
}
 */
