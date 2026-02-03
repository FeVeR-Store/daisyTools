use proc_macro2::Span;
use syn::{Lit, LitBool, LitFloat, LitInt, LitStr};

pub trait FromBase {
    fn from_str(value: &str) -> Self;
    fn from_bool(value: bool) -> Self;
    fn from_int(value: i64) -> Self;
    fn from_float(value: f64) -> Self;
    fn from_int_str(value: &str) -> Self;
    fn from_float_str(value: &str) -> Self;
}

pub fn create_string_literal(value: &str) -> Lit {
    Lit::from_str(value)
}

impl FromBase for Lit {
    fn from_bool(value: bool) -> Self {
        Lit::Bool(LitBool::new(value, Span::call_site()))
    }
    fn from_float(value: f64) -> Self {
        Lit::Float(LitFloat::new(&format!("{}", value), Span::call_site()))
    }
    fn from_int(value: i64) -> Self {
        Lit::Int(LitInt::new(&format!("{}", value), Span::call_site()))
    }
    fn from_float_str(value: &str) -> Self {
        Lit::Float(LitFloat::new(value, Span::call_site()))
    }
    fn from_int_str(value: &str) -> Self {
        Lit::Int(LitInt::new(value, Span::call_site()))
    }
    fn from_str(value: &str) -> Self {
        Lit::Str(LitStr::new(value, Span::call_site()))
    }
}
