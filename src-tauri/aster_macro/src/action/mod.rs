use aster_common::{action::param::parse_param_attributes, i18n::I18n};
use proc_macro::TokenStream;
// 过程宏的输入输出类型
use darling::ast::NestedMeta;
use quote::quote;
use serde::{Deserialize, Serialize};
// 用于生成 Rust 代码的宏
use syn::{parse_macro_input, ItemEnum};

pub mod define;
pub mod result;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub label: Option<I18n>,
    pub value: String,
}

pub fn define_options_proc(input: TokenStream) -> TokenStream {
    // 将输入的TokenStream解析为ItemEnum类型
    let mut options_enum = parse_macro_input!(input as ItemEnum);
    for variant in options_enum.variants.iter_mut() {
        variant.attrs.retain(|attr| {
            let ident = attr.meta.path().get_ident().unwrap();
            let ident = ident.to_string();
            if ident == "label" {
                match parse_param_attributes(&[NestedMeta::Meta(attr.meta.clone())]) {
                    Ok(_) => false,
                    Err(e) => {
                        eprintln!("解析选项属性失败: {}", e);
                        true
                    }
                }
            } else {
                true
            }
        })
    }

    let expand = quote! {
        #[derive(Debug, ::aster_macro::ToValue, ::serde::Serialize, ::serde::Deserialize)]
        #options_enum
    };
    expand.into()
}

pub fn to_value_derive_impl(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);

    let id = ast.ident;
    let mut match_ast = quote! {};

    for variant in ast.variants {
        if !variant.fields.is_empty() {
            panic!(
                "options is only for enums with no fields, but found variant `{}` with fields.",
                variant.ident
            );
        }
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        let match_arm = quote! {
            #id::#variant_name => #variant_str.to_string(),
        };
        match_ast.extend(match_arm);
    }

    let expand = quote! {
        impl ToString for #id {
            fn to_string(&self) -> String {
                match self {
                    #match_ast
                }
            }
        }
    };
    expand.into()
}

pub use define::define_action_impl;

/// 创建分支Result，根据枚举生成的Result，直接将返回值对应到workflow的下一个分支
pub use result::result_branch_impl;
