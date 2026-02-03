use proc_macro2::Span;
use quote::ToTokens;
use serde_json::{Value, json};
use syn::{Field, Ident};

pub use crate::rust::lit::create_string_literal;

pub trait IntoString {
    fn into_string(&self) -> String;
}

impl<T: ToTokens> IntoString for T {
    fn into_string(&self) -> String {
        std::string::ToString::to_string(&self.to_token_stream())
    }
}

pub trait IntoIdent {
    fn into_ident(self) -> Ident;
}

impl<'a> IntoIdent for &'a str {
    fn into_ident(self) -> Ident {
        Ident::new(self, Span::call_site())
    }
}
pub trait GetIdent<'a> {
    fn get_ident(&self, default: &'a str) -> Ident;
}

impl<'a> GetIdent<'a> for Field {
    fn get_ident(&self, default: &'a str) -> Ident {
        match &self.ident {
            Some(ident) => ident.clone(),
            None => default.into_ident(),
        }
    }
}

pub fn normalize_type(type_name: &str) -> Value {
    match type_name {
        "String" | "string" | "Code" | "Text" => json!("string"),
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" | "f16" | "f32" | "f64" | "f128" | "number" => json!("number"),
        "bool" | "boolean" => json!("boolean"),
        _ => json!({ "\0type": "object" }),
    }
}
