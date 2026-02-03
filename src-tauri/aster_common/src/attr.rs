use std::error::Error;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use syn::{parse::{Parse, ParseStream}, Attribute, Ident};

// 自定义解析器结构体，用于解析逗号分隔的 NestedMeta 列表
#[derive(Debug)]
pub struct NestedMetaList(Vec<NestedMeta>);

impl Parse for NestedMetaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated =
            syn::punctuated::Punctuated::<NestedMeta, syn::Token![,]>::parse_terminated(input)?;
        Ok(NestedMetaList(punctuated.into_iter().collect()))
    }
}

pub fn parse_attr<T: FromMeta>(attr: &Attribute) -> Result<(T, Ident), darling::Error> {
    let ident = attr.path().require_ident()?;

    Ok((T::from_meta(&attr.meta)?, ident.clone()))
}

pub fn parse_proc_attr<T: FromMeta>(attr_token: &TokenStream) -> Result<T, Box<dyn Error>> {
    let parsed_result: Result<NestedMetaList, _> = syn::parse2(attr_token.clone());
    let Ok(NestedMetaList(nested_metas)) = parsed_result else {
        return Err(Box::new(parsed_result.unwrap_err()));
    };
    Ok(T::from_list(&nested_metas)?)
}
