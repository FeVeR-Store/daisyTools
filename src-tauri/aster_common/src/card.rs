use std::collections::HashMap;

use crate::i18n::{ParsedI18nMap};

#[derive(Debug, Clone, Default)]
pub struct CardAttr {
    pub title: Option<ParsedI18nMap>,
    pub description: Option<ParsedI18nMap>,
    pub entries: Vec<String>,
    pub keys: Vec<String>,
}

impl CardAttr {
    /// 获取所有标题配置
    pub fn get_all_titles(&self) -> ParsedI18nMap {
        self.title
            .clone()
            .unwrap_or_else(HashMap::new)
    }

    /// 获取所有描述配置
    pub fn get_all_descriptions(&self) -> ParsedI18nMap {
        self.description
            .clone()
            .unwrap_or_else(HashMap::new)
    }
}
