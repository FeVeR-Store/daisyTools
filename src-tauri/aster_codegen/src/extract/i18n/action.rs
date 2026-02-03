use aster_common::action::param::{ParamInfo, parse_param_attributes};
use aster_common::attr::parse_attr;
use aster_common::card::CardAttr;
use aster_common::i18n::{ParamI18n, ParsedI18nMap};
use aster_common::utils::IntoString;
use darling;
use proc_macro2::TokenStream;
use syn::{self, ItemFn, Meta, ReturnType, parse_quote};

use crate::CardInfo;

pub fn extract_i18n_from_action(action: &ItemFn) -> CardInfo {
    let action_name = action.sig.ident.clone();
    let action_name_str = action_name.to_string();
    let mut func_description_attrs = CardAttr::default();

    let result_return_type;

    match &action.sig.output {
        ReturnType::Type(_, ty) => {
            let ty = ty.as_ref();
            result_return_type = ty.into_string();
        }
        _ => panic!("Return type is required"),
    };

    for attr in &action.attrs {
        match attr.path().into_string().as_str() {
            "entry" => {
                if let Meta::List(ref list) = attr.meta {
                    // 将 TokenStream 转换为 NestedMeta 列表进行 Darling 解析
                    let token_stream = list.tokens.clone().to_string();
                    func_description_attrs.entries.push(token_stream);
                }
            }
            ty @ ("description" | "action") => {
                // let desc = vec![NestedMeta::Meta(attr.meta.clone())];
                // match parse_param_attributes(&desc) {
                //     Ok(parsed) => {
                //         if ty == "description" {
                //             func_description_attrs.description = parsed.description;
                //         } else {
                //             func_description_attrs.title = parsed.action;
                //         }
                //     }
                //     Err(e) => {
                //         eprintln!("解析函数 description 属性失败: {}", e);
                //     }
                // };
                match parse_attr::<ParsedI18nMap>(attr) {
                    Ok((i18n, _)) => {
                        if !i18n.is_empty() {
                            if ty == "description" {
                                func_description_attrs.description = Some(i18n);
                            } else {
                                func_description_attrs.title = Some(i18n);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("解析函数 description 属性失败: {}", e);
                    }
                };
            }
            _ => (),
        }
    }
    let mut all_param_info: Vec<ParamInfo> = Vec::new();

    // 遍历函数的所有参数，提取参数名和类型
    for arg in action.sig.inputs.iter() {
        match arg {
            // 不允许使用 self 参数，因为这是静态函数
            syn::FnArg::Receiver(_) => panic!("'self' is not allowed!"),
            syn::FnArg::Typed(typed_param) => {
                if let syn::Pat::Ident(pat_ident) = *typed_param.pat.clone() {
                    let param_name = &pat_ident.ident;
                    let param_type = &typed_param.ty;
                    let param_name_str = param_name.to_string();
                    let param_type_str: TokenStream = parse_quote! { #param_type };
                    let param_type_str = param_type_str.to_string();

                    // 使用 Darling 解析参数属性
                    match parse_param_attributes(
                        &typed_param
                            .attrs
                            .iter()
                            .map(|e| darling::ast::NestedMeta::Meta(e.meta.clone()))
                            .collect::<Vec<_>>(),
                    ) {
                        Ok(param_attr) => {
                            // 创建参数信息并存储
                            let param_info =
                                ParamInfo::new(param_name_str.clone(), param_type_str, param_attr);
                            all_param_info.push(param_info);
                        }
                        Err(e) => {
                            // 如果解析失败，创建一个空的属性配置
                            eprintln!("解析参数 '{}' 的属性失败: {}", param_name_str, e);
                            let param_info = ParamInfo::new(
                                param_name_str.clone(),
                                param_type_str,
                                Default::default(),
                            );
                            all_param_info.push(param_info);
                        }
                    }
                } else {
                    panic!("Unsupported pattern")
                }
            }
        }
    }

    let mut card_info =
        generate_action_processing(&action_name_str, &func_description_attrs, &all_param_info);

    card_info.result = result_return_type;

    card_info
}

// 生成完整的 action 信息，包括 action 本身的多语言信息
fn generate_action_processing(
    action_type: &str,
    func_description_attrs: &CardAttr,
    param_info: &[ParamInfo],
) -> CardInfo {
    // 收集参数信息
    let params_i18n: Vec<_> = param_info
        .iter()
        .map(|param| {
            let name_info = param.attributes.get_all_names();
            let desc_info = param.attributes.get_all_descriptions();

            ParamI18n {
                description: desc_info,
                key: param.name.clone(),
                name: name_info,
                r#type: param.r#type.clone(),
            }
        })
        .collect();

    // 收集 action 标题信息（来自 #[action] 宏属性）
    let title_info = func_description_attrs.get_all_titles();

    // 收集 action 描述信息（来自 #[description] 属性）
    let description_info = func_description_attrs.get_all_descriptions();

    CardInfo {
        title: title_info,
        parent: String::new(),
        file: String::new(),
        result: String::new(),
        action_type: action_type.to_string(),
        description: description_info,
        entries: func_description_attrs.entries.clone(),
        keys: func_description_attrs.keys.clone(),
        params: params_i18n,
    }
}
