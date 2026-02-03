use syn::{Expr, ExprLit, Lit};

pub trait FromLit {
    fn from_lit(lit: Lit) -> Self;
}

impl FromLit for Expr {
    fn from_lit(lit: Lit) -> Self {
        Expr::Lit(ExprLit { attrs: vec![], lit })
    }
}
