use std::collections::HashMap;

use crate::{
    i18n::{I18nValue, Label, ParsedI18nMap, ParsedI18nMapTrait},
    typescript::DefaultValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormData {
    Option(Vec<Label>),
    AutoComplete(Vec<Label>),
    Range { min: f64, max: f64, step: f64 },
    Input { placeholder: ParsedI18nMap },
}

impl FormData {
    pub fn get_i18n(&self, ns: &str) -> HashMap<String, Vec<(String, I18nValue)>> {
        let mut i18n: HashMap<String, Vec<(String, I18nValue)>> = HashMap::new();
        match self {
            Self::AutoComplete(labels) => {
                labels.iter().for_each(|l| {
                    let i = l.get_i18n(ns);
                    i.iter().for_each(|(lang, (ns_value, value))| {
                        i18n.entry(lang.clone())
                            .or_insert_with(Vec::new)
                            .push((ns_value.clone(), value.clone()));
                    });
                });
            }
            Self::Input { placeholder } => {
                placeholder
                    .to_filter_value()
                    .iter()
                    .for_each(|(lang, value)| {
                        i18n.entry(lang.clone()).or_insert_with(Vec::new).push((
                            format!("{}_placeholder", ns),
                            I18nValue::Single(value.clone()),
                        ));
                    });
            }
            Self::Option(labels) => {
                labels.iter().for_each(|label| {
                    let i = label.get_i18n(ns);
                    i.iter().for_each(|(lang, (ns_value, value))| {
                        i18n.entry(lang.clone())
                            .or_insert_with(Vec::new)
                            .push((ns_value.clone(), value.clone()));
                    });
                });
            }
            _ => (),
        };
        i18n
    }
}

type Optional = bool;
#[derive(Debug, Clone, Deserialize)]
pub enum FormType {
    String(Optional),
    Number {
        optional: Optional,
        min: Option<f64>,
        max: Option<f64>,
        step: f64,
    },
    Option(Optional),
    Date(Optional),
    TextArea(Optional),
    File(Optional),
    AutoComplete(Optional),
    Range(Optional),
    Code(Optional),
    Switch(Optional),
    Unknown(Optional),
}

impl FormType {
    pub fn get_args_type(&self) -> String {
        match self {
            FormType::String(_) => "String".to_string(),
            FormType::Number { step, .. } => {
                if step == &1.0 {
                    "Int".to_string()
                } else {
                    "Float".to_string()
                }
            }
            FormType::Option(_) => "String".to_string(),
            FormType::Date(_) => "Int".to_string(),
            FormType::TextArea(_) => "Text".to_string(),
            FormType::File(_) => "File".to_string(),
            FormType::AutoComplete(_) => "String".to_string(),
            FormType::Range(_) => "Int".to_string(),
            FormType::Code(_) => "Code".to_string(),
            FormType::Switch(_) => "Bool".to_string(),
            // ? Unknown会被fix，并且目前这种情况只在传入Option出现
            // ! 但后续可能会出现改变
            FormType::Unknown(_) => "String".to_string(),
        }
    }
    pub fn get_width(&self) -> Option<i8> {
        match self {
            FormType::Code(_) => Some(2),
            FormType::TextArea(_) => Some(2),
            _ => None,
        }
    }
    pub fn fix_with(&mut self, form_data: &FormData) {
        // 要修复的情况只有type为Unknown时
        if let FormType::Unknown(optional) = self {
            match form_data {
                FormData::AutoComplete(_) => *self = FormType::AutoComplete(*optional),
                FormData::Option(_) => *self = FormType::Option(*optional),
                _ => (),
            }
        }
    }
    pub fn is_optional(&self) -> bool {
        match self {
            FormType::String(optional) => *optional,
            FormType::Number { optional, .. } => *optional,
            FormType::Option(optional) => *optional,
            FormType::Date(optional) => *optional,
            FormType::TextArea(optional) => *optional,
            FormType::File(optional) => *optional,
            FormType::AutoComplete(optional) => *optional,
            FormType::Range(optional) => *optional,
            FormType::Code(optional) => *optional,
            FormType::Switch(optional) => *optional,
            FormType::Unknown(optional) => *optional,
        }
    }
    pub fn get_form_type(&self) -> String {
        match self {
            FormType::String(_) => "String".to_string(),
            FormType::Number { .. } => "Number".to_string(),
            FormType::Option(_) => "Option".to_string(),
            FormType::Date(_) => "Date".to_string(),
            FormType::TextArea(_) => "TextArea".to_string(),
            FormType::File(_) => "File".to_string(),
            FormType::AutoComplete(_) => "AutoComplete".to_string(),
            FormType::Range(_) => "Range".to_string(),
            FormType::Code(_) => "Code".to_string(),
            FormType::Switch(_) => "Switch".to_string(),
            FormType::Unknown(_) => "Unknown".to_string(),
        }
    }
    pub fn get_inner_type(t: &str) -> Option<String> {
        if t.starts_with("Option<") && t.ends_with(">") {
            let inner_type = &t[7..t.len() - 1];
            Some(inner_type.to_string())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormItem {
    pub args_type: String,
    pub name: String,
    pub r#type: FormType,
    pub default: Option<DefaultValue>,
    pub data: Option<FormData>,
    pub optional: bool,
}
