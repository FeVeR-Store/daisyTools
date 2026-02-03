use std::collections::HashMap;

use super::base::FromBase;
use serde_json::Value as SerdeValue;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
    ArrayLit, Bool, Expr, ExprOrSpread, KeyValueProp, Lit, Null, ObjectLit, Prop, PropName,
    PropOrSpread, SpreadElement,
};

use crate::{
    action::{
        form::{FormData, FormItem, FormType},
        result::{PlugType, ResultBranchType},
        stat::Stat,
    },
    i18n::Label,
    typescript::expr::base::ToExpr,
};

#[derive(Debug, Clone)]
pub enum ExprValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, ExprValue>),
    Expr(Expr),
    Entries(ObjectEntry),
    SpreadObject(Vec<(String, ExprValue)>),
}

type KeyValueEntry = Vec<(String, ExprValue)>;
type ArrayEntry = Vec<Expr>;

#[derive(Debug, Clone)]
pub enum ObjectEntry {
    KeyValue(KeyValueEntry),
    Array(Vec<Expr>),
    Empty,
}

pub fn create_array_expr(element: Vec<Expr>) -> Expr {
    Expr::Array(ArrayLit {
        elems: element
            .into_iter()
            .map(|element| {
                Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(element),
                })
            })
            .collect::<Vec<_>>(),
        span: DUMMY_SP,
    })
}

pub fn create_object_expr(entries: ObjectEntry) -> Expr {
    let mut props = Vec::new();

    let entries = match entries {
        ObjectEntry::Array(array) => return create_array_expr(array),
        ObjectEntry::Empty => {
            return Expr::Object(ObjectLit {
                props: Vec::new(),
                span: DUMMY_SP,
            });
        }
        ObjectEntry::KeyValue(entries) => entries,
    };

    for (key, value) in entries {
        match value {
            ExprValue::Entries(entries) => {
                let object = create_object_expr(entries);
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(object),
                }))));
            }
            ExprValue::String(value) => {
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(Expr::from_str(&value)),
                }))));
            }
            ExprValue::Boolean(value) => {
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(Expr::from_bool(value)),
                }))));
            }
            ExprValue::Number(value) => {
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(Expr::from_f64(value)),
                }))));
            }
            ExprValue::Object(object) => {
                let entries = object
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<Vec<_>>();
                let member = create_object_expr(ObjectEntry::KeyValue(entries));
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(member),
                }))));
            }
            ExprValue::Expr(expr) => {
                props.push(PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                    key: PropName::from_str(&key),
                    value: Box::new(expr),
                }))));
            }
            ExprValue::SpreadObject(entries) => {
                props.push(PropOrSpread::Spread(SpreadElement {
                    dot3_token: DUMMY_SP,
                    expr: Box::new(create_object_expr(ObjectEntry::KeyValue(entries))),
                }));
            }
        }
    }
    Expr::Object(ObjectLit {
        props,
        span: DUMMY_SP,
    })
}

pub trait ToObjectExpr {
    fn to_object_entry(&self) -> ObjectEntry;
    fn to_object_expr(&self) -> Expr {
        create_object_expr(self.to_object_entry())
    }
}

impl ToObjectExpr for serde_json::Map<String, SerdeValue> {
    fn to_object_entry(&self) -> ObjectEntry {
        self.iter()
            .map(|(k, v)| (k.as_str(), ExprValue::Expr(v.to_expr())))
            .collect::<Vec<_>>()
            .to_object_entry()
    }
}

impl<T: ToObjectExpr> ToObjectExpr for Vec<T> {
    fn to_object_entry(&self) -> ObjectEntry {
        ObjectEntry::Array(
            self.iter()
                .map(|item| item.to_object_expr())
                .collect::<Vec<_>>(),
        )
    }
}

impl ToObjectExpr for FormData {
    fn to_object_entry(&self) -> ObjectEntry {
        match self {
            Self::AutoComplete(labels) => {
                let entries = labels
                    .iter()
                    .map(|Label { value, .. }| {
                        create_object_expr(
                            vec![
                                ("label", ExprValue::String(value.clone())),
                                ("value", ExprValue::String(value.clone())),
                            ]
                            .to_object_entry(),
                        )
                    })
                    .collect::<Vec<_>>();

                entries.to_object_entry()
            }
            Self::Input { .. } => ObjectEntry::Empty,
            Self::Option(labels) => {
                let entries = labels
                    .iter()
                    .map(|Label { value, .. }| {
                        create_object_expr(
                            vec![
                                ("label", ExprValue::String(value.clone())),
                                ("value", ExprValue::String(value.clone())),
                            ]
                            .to_object_entry(),
                        )
                    })
                    .collect::<Vec<_>>();
                entries.to_object_entry()
            }

            Self::Range { min, max, step } => vec![
                ("min", ExprValue::Number(*min)),
                ("max", ExprValue::Number(*max)),
                ("step", ExprValue::Number(*step)),
            ]
            .to_object_entry(),
        }
    }
}

impl ToObjectExpr for HashMap<String, String> {
    fn to_object_entry(&self) -> ObjectEntry {
        self.iter()
            .map(|(k, v)| (k.clone(), ExprValue::String(v.clone())))
            .collect::<Vec<_>>()
            .to_object_entry()
    }
}

impl ToObjectExpr for FormItem {
    fn to_object_entry(&self) -> ObjectEntry {
        let r#type = self.r#type.get_form_type();
        let mut entries = vec![
            ("name", ExprValue::String(self.name.clone())),
            ("type", ExprValue::String(r#type.clone())),
            ("optional", ExprValue::Boolean(self.optional)),
        ];

        let meta = if r#type == "Number" {
            if let FormType::Number { min, max, step, .. } = &self.r#type {
                let mut entries = vec![(String::from("step"), ExprValue::Number(*step))];
                if let Some(min) = min {
                    entries.push((String::from("min"), ExprValue::Number(*min)));
                }
                if let Some(max) = max {
                    entries.push((String::from("max"), ExprValue::Number(*max)));
                }
                entries
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        let data_entries = if let Some(data) = &self.data {
            // 向前解构注入默认数据
            // 目前该情况只包括数字时，根据数字的类型推断最大最小值与步长
            match data.to_object_entry() {
                ObjectEntry::KeyValue(mut entries) => {
                    if !meta.is_empty() {
                        entries.insert(0, (String::new(), ExprValue::SpreadObject(meta)));
                    }
                    entries.to_object_entry()
                }
                ObjectEntry::Array(array) => array.to_object_entry(),
                _ => ObjectEntry::Empty,
            }
        } else {
            ObjectEntry::Empty
        };
        entries.push(("data", ExprValue::Entries(data_entries)));
        if let Some(default) = &self.default {
            entries.push(("defaultValue", ExprValue::Expr(default.get_value_expr())));
        }
        entries.to_object_entry()
    }
}

impl ToObjectExpr for KeyValueEntry {
    fn to_object_entry(&self) -> ObjectEntry {
        ObjectEntry::KeyValue(self.to_vec())
    }
}

// &str 简写
impl<'a> ToObjectExpr for Vec<(&'a str, ExprValue)> {
    fn to_object_entry(&self) -> ObjectEntry {
        ObjectEntry::KeyValue(
            self.iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }
}

impl ToObjectExpr for ArrayEntry {
    fn to_object_entry(&self) -> ObjectEntry {
        ObjectEntry::Array(self.to_vec())
    }
}

impl ToObjectExpr for Stat {
    fn to_object_entry(&self) -> ObjectEntry {
        let mut entries = vec![("key", ExprValue::String(self.key.clone()))];
        if self.width.is_some() {
            entries.push(("width", ExprValue::Number(self.width.unwrap().into())));
        }
        entries.to_object_entry()
    }
}

impl ToObjectExpr for ResultBranchType {
    fn to_object_entry(&self) -> ObjectEntry {
        let mut entries = vec![
            ("branch", ExprValue::String(self.branch.clone())),
            ("type", ExprValue::String(self.r#type.clone())),
            ("id", ExprValue::String(self.id.clone())),
            ("position", ExprValue::String(self.position.clone())),
        ];
        match &self.plug {
            PlugType::Error => entries.push(("plug", ExprValue::String("Error".to_string()))),
            PlugType::Unknown => entries.push(("plug", ExprValue::String("Unknown".to_string()))),
            PlugType::Value(val) => entries.push(("plug", ExprValue::Expr(val.to_expr()))),
            PlugType::Base(str) => entries.push(("plug", ExprValue::String(str.to_string()))),
            _ => (),
        };
        entries.to_object_entry()
    }
}
impl ToExpr for SerdeValue {
    fn to_expr(&self) -> Expr {
        match self {
            SerdeValue::Bool(bool) => Expr::Lit(Lit::Bool(Bool {
                span: DUMMY_SP,
                value: *bool,
            })),
            SerdeValue::Array(arr) => Expr::Array(ArrayLit {
                span: DUMMY_SP,
                elems: arr
                    .iter()
                    .map(|ele| {
                        Some(ExprOrSpread {
                            spread: None,
                            expr: Box::new(ele.to_expr()),
                        })
                    })
                    .collect::<Vec<_>>(),
            }),
            SerdeValue::Null => Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
            SerdeValue::Number(number) => Expr::from_f64(number.as_f64().unwrap().into()),
            SerdeValue::Object(obj) => obj.to_object_expr(),
            SerdeValue::String(str) => Expr::from_str(&str),
        }
    }
}
