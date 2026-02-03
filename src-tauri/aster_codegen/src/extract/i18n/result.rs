use aster_common::{
    action::result::{PlugType, ResultBranchType},
    attr::parse_attr,
    i18n::ParsedI18nMap,
    nesting::parse_nesting,
    utils::{IntoIdent, IntoString, normalize_type},
};
use serde_json::{Map, Value, json};
use syn::{
    Attribute, ExprCast, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, ItemEnum, Meta, Type,
    Variant, parse_quote,
};

pub(crate) struct ParsedAttribute {
    pub position: String,
    pub into_error: bool,
    pub into_branch: bool,
    pub i18n: Option<ParsedI18nMap>,
}

struct Context<'a> {
    pub result_branch_types: &'a mut Vec<ResultBranchType>,
    /// result 宏修饰的枚举名
    pub result_ident: &'a Ident,
}

type PlugValue = Value;

fn extract_plug_value_from_arg(field: &Field) -> PlugValue {
    let mut plug_value = Value::Null;
    // nesting
    loop {
        if let Type::Macro(m) = &field.ty {
            if !m.mac.path.is_ident("nesting") {
                break;
            }
            let mut stmt = vec![];
            let mut nesting_plug = Map::new();
            nesting_plug.insert("\0type".to_string(), json!("object"));
            parse_nesting(&field, &m.mac.tokens, "_", (&mut stmt, &mut nesting_plug)).unwrap();
        }
        break;
    }
    loop {
        if let Type::Macro(m) = &field.ty {
            if !m.mac.path.is_ident("plug") {
                break;
            }
            let token = &m.mac.tokens;
            let cast: ExprCast = parse_quote!(#token);
            let (_, ty) = (cast.expr.as_ref(), cast.ty.as_ref());
            plug_value = match ty {
                Type::Macro(m) => {
                    let mut plug = Map::new();
                    plug.insert("\0type".to_string(), json!("object"));

                    let _ = parse_nesting(&field, &m.mac.tokens, "", (&mut vec![], &mut plug));
                    plug.into()
                }
                t @ _ => normalize_type(&t.into_string()),
            };
        };
        break;
    }
    plug_value
}

pub fn extract_i18n_and_position_from_result(item: &ItemEnum) -> Vec<ResultBranchType> {
    let result_ident = &item.ident;
    let mut result_branch_types = vec![ResultBranchType {
        branch: "source".to_string(),
        id: result_ident.clone().to_string(),
        r#type: String::from("source"),
        position: String::from("left"),
        plug: PlugType::None,
        i18n: None,
    }];

    for e in item.attrs.iter() {
        if e.path().is_ident("source") {
            if let Meta::List(list) = &e.meta {
                let default = &"left".into_ident();
                let ident = list.path.get_ident().unwrap_or_else(|| default);
                match ident.to_string().as_str() {
                    p @ ("left" | "right" | "top" | "bottom") => {
                        result_branch_types.clear();
                        result_branch_types.push(ResultBranchType {
                            branch: "source".to_string(),
                            id: result_ident.to_string(),
                            r#type: String::from("source"),
                            position: p.to_string(),
                            plug: PlugType::None,
                            i18n: None,
                        });
                    }
                    _ => (),
                }
            }
        };
    }

    let mut context = Context {
        result_branch_types: &mut result_branch_types,
        result_ident,
    };

    for variant in &item.variants {
        let attribute = parse_result_attr(&variant.attrs);
        match &variant.fields {
            Fields::Unnamed(fields) => {
                handle_unnamed_variant(variant, fields, attribute, &mut context)
            }
            Fields::Named(fields) => handle_named_variant(variant, fields, attribute, &mut context),
            Fields::Unit => handle_unit_variant(variant, attribute, &mut context),
        };
    }
    result_branch_types
}

fn handle_unnamed_variant(
    variant: &Variant,
    unname: &FieldsUnnamed,
    ParsedAttribute { position, i18n, .. }: ParsedAttribute,
    context: &mut Context,
) {
    let ident = variant.ident.clone();
    let ident_name = ident.to_string();

    let mut members = unname.unnamed.clone();
    let len: usize = members.len();
    let mut plug = PlugType::None;
    for (idx, f) in members.iter_mut().enumerate() {
        let p = extract_plug_value_from_arg(f);
        if len == 1 {
            plug = match p {
                Value::String(str) => PlugType::Base(str),
                Value::Object(map) => PlugType::Value(map.into()),
                _ => PlugType::None,
            }
        } else {
            loop {
                match plug {
                    PlugType::None => {
                        plug = PlugType::Value(json!({ "\0type": "tuple" }));
                        continue;
                    }
                    PlugType::Value(mut val) => {
                        let map = val.as_object_mut().unwrap();
                        map.insert(format!("[{}]", idx), p);
                        plug = PlugType::Value(val);
                    }
                    _ => (),
                }
                break;
            }
        };
    }
    context.result_branch_types.push(ResultBranchType {
        branch: ident_name,
        id: context.result_ident.to_string(),
        r#type: String::from("primary"),
        position,
        plug,
        i18n,
    });
}

fn handle_named_variant(
    variant: &Variant,
    named: &FieldsNamed,
    ParsedAttribute {
        position,
        into_error,
        i18n: g_i18n,
        into_branch,
    }: ParsedAttribute,
    context: &mut Context,
) {
    let ident = variant.ident.clone();
    let ident_name = ident.to_string();
    if !into_branch {
        let mut plug_type = Map::new();

        let fields = &named.named;
        for f in fields.iter() {
            let ident_name = if let Some(ident) = &f.ident {
                ident.to_string()
            } else {
                String::from("unknown")
            };
            let plug = extract_plug_value_from_arg(f);
            plug_type.insert(ident_name, plug);
        }

        let mut plug = json!({ "\0type": "object" });
        let map = plug.as_object_mut().unwrap();

        plug_type.iter().for_each(|(k, v)| {
            map.insert(k.to_string(), v.clone());
        });

        // 添加分支结果类型
        context.result_branch_types.push(ResultBranchType {
            branch: ident_name,
            id: context.result_ident.clone().to_string(),
            r#type: if into_error {
                String::from("error")
            } else {
                String::from("primary")
            },
            position: position.clone(),
            plug: PlugType::Value(plug),
            i18n: g_i18n.clone(),
        });
        return;
    }
    for Field {
        attrs,
        ident: id,
        ty,
        ..
    } in named.named.iter()
    {
        // 实现子分支的fn
        let Some(sub_branch_ident) = id else {
            panic!("Named branch in {} has no ident", &variant.ident);
        };
        let variant_name = format!("{}_{}", &ident.to_string(), sub_branch_ident.to_string());

        let ParsedAttribute {
            position: pos,
            i18n,
            ..
        } = parse_result_attr(&attrs);

        let Context { result_ident, .. } = context;

        let mut plug;
        match ty {
            // 元组类型转化为多参数函数
            Type::Tuple(tuple) => {
                plug = json!({ "\0type": "tuple" });
                let map = plug.as_object_mut().unwrap();

                let members: Vec<String> = tuple.elems.iter().map(|e| e.into_string()).collect();
                members.iter().enumerate().for_each(|(idx, m)| {
                    map.insert(format!("[{}]", idx), normalize_type(&m));
                });
            }
            Type::Path(type_path) => {
                let type_name = type_path.into_string();
                plug = normalize_type(&type_name);
            }
            _ => todo!(),
        };
        // 添加分支结果类型
        context.result_branch_types.push(ResultBranchType {
            branch: variant_name,
            id: result_ident.clone().to_string(),
            r#type: if into_error {
                String::from("error")
            } else {
                String::from("primary")
            },
            position: if pos.is_empty() {
                position.clone()
            } else {
                pos
            },
            plug: if into_error {
                PlugType::Error
            } else {
                PlugType::Value(plug)
            },
            i18n: i18n.or(g_i18n.clone()),
        });
    }
}

fn handle_unit_variant(
    variant: &Variant,
    ParsedAttribute {
        position,
        into_error,
        i18n,
        ..
    }: ParsedAttribute,
    context: &mut Context,
) {
    let variant_ident = variant.ident.clone();
    let variant_name = variant_ident.to_string();

    // 添加分支结果类型
    context.result_branch_types.push(ResultBranchType {
        branch: variant_name.clone(),
        id: context.result_ident.to_string(),
        r#type: if into_error {
            String::from("error")
        } else {
            String::from("primary")
        },
        position,
        plug: PlugType::None,
        i18n,
    });
}

fn parse_result_attr(attrs: &Vec<Attribute>) -> ParsedAttribute {
    let mut pa = ParsedAttribute {
        position: String::new(),
        into_error: false,
        i18n: None,
        into_branch: false,
    };

    let mut label_i18n = false;
    for attr in attrs {
        let attr_ident = attr.path().require_ident().unwrap();
        let attr_str = attr_ident.to_string();
        let attr_str: &str = Box::leak(Box::new(attr_str));
        match attr_str {
            p @ ("right" | "left" | "top" | "bottom") => {
                if let Ok((i18n, _)) = parse_attr::<ParsedI18nMap>(attr) {
                    if !label_i18n {
                        pa.i18n = Some(i18n);
                    }
                }
                if pa.position.is_empty() {
                    pa.position.push_str(p);
                }
            }
            "label" => {
                if let Ok((i18n, _)) = parse_attr::<ParsedI18nMap>(attr) {
                    label_i18n = true;
                    pa.i18n = Some(i18n);
                }
            }
            "branch" => {
                match &attr.meta {
                    Meta::List(list) => {
                        if &list.tokens.to_string() == "error" {
                            pa.into_error = true;
                        }
                    }
                    _ => panic!("only receive error"),
                }
                pa.into_branch = true;
            }
            _ => (),
        }
    }

    pa
}
