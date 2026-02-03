use std::collections::HashMap;

use darling::{FromMeta, ast::NestedMeta};

use crate::i18n::{ParsedI18nMap, ParsedI18nMapTrait};

// 参数信息结构体，用于存储参数的完整信息
#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: String,          // 参数名
    pub r#type: String,        // 参数类型
    pub attributes: ParamAttr, // 属性配置
}

impl ParamInfo {
    pub fn new(name: String, r#type: String, attributes: ParamAttr) -> Self {
        Self {
            name,
            r#type,
            attributes,
        }
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
pub struct ParamAttr {
    #[darling(default)]
    pub action: Option<ParsedI18nMap>,
    #[darling(default, rename = "name")]
    pub name: Option<ParsedI18nMap>,
    #[darling(default, rename = "description")]
    pub description: Option<ParsedI18nMap>,
    #[darling(default, rename = "label")]
    pub label: Option<ParsedI18nMap>,
    #[darling(default)]
    pub default: Option<String>,
}

impl ParamAttr {
    /// 获取所有名称配置
    pub fn get_all_names(&self) -> HashMap<String, String> {
        self.name
            .clone()
            .unwrap_or_else(HashMap::new)
            .to_filter_value()
    }

    /// 获取所有描述配置
    pub fn get_all_descriptions(&self) -> HashMap<String, String> {
        self.description
            .clone()
            .unwrap_or_else(HashMap::new)
            .to_filter_value()
    }
}

// 新的解析函数，使用 Darling 自动解析
pub fn parse_param_attributes(attrs: &[NestedMeta]) -> darling::Result<ParamAttr> {
    ParamAttr::from_list(attrs)
}
