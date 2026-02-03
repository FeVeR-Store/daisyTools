pub mod expr;

use swc_ecma_ast::Expr;

use crate::typescript::expr::base::FromBase;

#[derive(Debug, Clone)]
pub enum DefaultValue {
    String(String),
    Int(i64),
    Float(f64),
    Boolean(bool),
}

impl DefaultValue {
    pub fn get_value_expr(&self) -> Expr {
        match self {
            DefaultValue::String(value) => Expr::from_str(value),
            DefaultValue::Int(value) => Expr::from_f64(*value as f64),
            DefaultValue::Boolean(value) => Expr::from_str(&value.to_string()),
            DefaultValue::Float(value) => Expr::from_f64(*value),
        }
    }
}
