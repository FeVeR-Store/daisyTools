use std::collections::HashMap;

use darling::FromMeta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromMeta, Serialize, Deserialize)]
pub struct I18n {
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
    pub en: Option<String>,
    pub ja: Option<String>,
    pub fr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParamI18n {
    pub description: HashMap<String, String>,
    pub key: String,
    pub name: HashMap<String, String>,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionI18n {
    pub action_type: String,
    pub title: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub params: Vec<ParamI18n>,
    pub entries: Vec<String>,
    pub keys: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub enum I18nValue {
    Single(String),
    WithDescription { title: String, description: String },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
    pub label: Option<ParsedI18nMap>,
    pub value: String,
}

impl Label {
    pub fn get_i18n(&self, ns: &str) -> HashMap<String, (String, I18nValue)> {
        if let Some(i18n) = &self.label {
            i18n.to_filter_value()
                .iter()
                .map(|(lang, value)| {
                    (
                        lang.clone(),
                        (
                            format!("{}_{}", ns, self.value.clone()),
                            I18nValue::Single(value.clone()),
                        ),
                    )
                })
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        }
    }
}

pub type ParsedI18nMap = HashMap<String, Option<String>>;

pub trait ParsedI18nMapTrait {
    fn to_filter_value(&self) -> HashMap<String, String>;
}
impl ParsedI18nMapTrait for ParsedI18nMap {
    fn to_filter_value(&self) -> HashMap<String, String> {
        self.iter()
            .filter_map(|(lang, value)| value.as_ref().map(|v| (lang.clone(), v.clone())))
            .collect()
    }
}
