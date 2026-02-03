use std::collections::HashMap;

use aster_common::
    i18n::I18nValue
;

use crate::FormType;

pub trait FromType {
    fn from_type(r#type: String) -> Self;
}

impl FromType for FormType {
    fn from_type(t: String) -> Self {
        let mut t = t.replace(" ", "");
        let optional = if let Some(inner) = FormType::get_inner_type(&t) {
            t = inner;
            true
        } else {
            false
        };
        if t == "String" {
            FormType::String(optional)
        } else if t == "Code" {
            FormType::Code(optional)
        } else if t == "Text" {
            FormType::TextArea(optional)
        } else if t == "bool" {
            FormType::Switch(optional)
        } else if is_u_number(&t) {
            FormType::Number {
                optional,
                step: 1.0,
                min: Some(0.0),
                max: None,
            }
        } else if is_i_number(&t) {
            FormType::Number {
                optional,
                step: 1.0,
                min: None,
                max: None,
            }
        } else if is_f_number(&t) {
            FormType::Number {
                optional,
                step: 0.01,
                min: None,
                max: None,
            }
        } else if t == "File" {
            FormType::File(optional)
        } else {
            FormType::Unknown(optional)
        }
    }
}

fn is_u_number(t: &str) -> bool {
    if !t.starts_with("u") {
        false
    } else {
        ["u8", "u16", "u32", "u64", "u128", "usize"].contains(&t)
    }
}
fn is_i_number(t: &str) -> bool {
    if !t.starts_with("i") {
        false
    } else {
        ["i8", "i16", "i32", "i64", "i128", "isize"].contains(&t)
    }
}
fn is_f_number(t: &str) -> bool {
    if !t.starts_with("f") {
        false
    } else {
        ["f32", "f64"].contains(&t)
    }
}

pub trait IntoI18nValueList<'a> {
    fn insert_into<'b: 'a>(self, i18n: &'b mut HashMap<String, Vec<(String, I18nValue)>>);
}

impl<'a> IntoI18nValueList<'a> for (&'a str, &'a HashMap<String, String>) {
    /// 转化(key, { zh_cn: <value>, en: <value> }) 形式
    fn insert_into<'b: 'a>(self, i18n: &'b mut HashMap<String, Vec<(String, I18nValue)>>) {
        let (key, map) = self;
        map.iter().for_each(|(lang, value)| {
            i18n.entry(lang.to_string())
                .or_insert_with(Vec::new)
                .push((key.to_string(), I18nValue::Single(value.clone())))
        });
    }
}

impl<'a> IntoI18nValueList<'a>
    for (
        &'a str,
        &'a HashMap<String, String>,
        &'a HashMap<String, String>,
    )
{
    /// 转化({ zh_cn: <value>, en: <value> }, { zh_cn: <description>, en: <description> }) 形式
    fn insert_into<'b: 'a>(self, i18n: &'b mut HashMap<String, Vec<(String, I18nValue)>>) {
        let (key, name, description) = self;
        name.iter().for_each(|(lang, title)| {
            let default_desc = String::new();
            let desc = description.get(lang).unwrap_or(&default_desc);
            i18n.entry(lang.clone()).or_insert_with(Vec::new).push((
                key.to_string(),
                I18nValue::WithDescription {
                    title: title.clone(),
                    description: desc.clone(),
                },
            ));
        });
    }
}

pub fn insert_i18n_value(
    i18n: &mut HashMap<String, Vec<(String, I18nValue)>>,
    i18n_map: HashMap<String, String>,
) {
    i18n_map.into_iter().for_each(|(lang, value)| {
        i18n.entry(lang)
            .or_insert_with(Vec::new)
            .push((String::new(), I18nValue::Single(value)));
    });
}
