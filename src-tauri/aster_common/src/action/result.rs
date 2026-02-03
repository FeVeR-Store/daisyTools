use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::i18n::{ParsedI18nMap, ParsedI18nMapTrait};

#[derive(Debug, Serialize, Deserialize)]
pub enum PlugType {
    Unknown,
    None,
    Error,
    Base(String),
    Value(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultBranchType {
    /// 分支名称
    pub branch: String,
    /// 分支类型id
    pub id: String,
    /// 分支所属的类型，目前有 primary | error
    pub r#type: String,
    /// handle位置
    pub position: String,
    /// 插头类型
    pub plug: PlugType,
    /// i18n
    pub i18n: Option<ParsedI18nMap>,
}

impl ResultBranchType {
    pub fn get_i18n(&self) -> HashMap<String, String> {
        self.i18n
            .clone()
            .unwrap_or_else(HashMap::new)
            .to_filter_value()
    }
}
