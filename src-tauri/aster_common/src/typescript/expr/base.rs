use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, IdentName, Lit, Number, PropName, Str};

pub trait FromBase {
    fn from_str(s: &str) -> Self;
    fn from_f64(f: f64) -> Self;
    fn from_bool(b: bool) -> Self;
}

pub trait ToExpr {
    fn to_expr(&self) -> Expr;
}

impl FromBase for Expr {
    fn from_str(s: &str) -> Self {
        Expr::Lit(Lit::Str(Str {
            span: DUMMY_SP,
            value: s.into(),
            raw: None,
        }))
    }
    fn from_f64(f: f64) -> Self {
        Expr::Lit(Lit::Num(Number {
            span: DUMMY_SP,
            value: f,
            raw: None,
        }))
    }
    fn from_bool(b: bool) -> Self {
        Expr::Lit(Lit::Bool(swc_ecma_ast::Bool {
            span: DUMMY_SP,
            value: b,
        }))
    }
}

impl FromBase for PropName {
    fn from_str(s: &str) -> Self {
        if is_valid_ident_name(s) {
            PropName::Ident(IdentName {
                span: DUMMY_SP,
                sym: s.into(),
            })
        } else {
            PropName::Str(Str::from(s))
        }
    }
    fn from_bool(b: bool) -> Self {
        PropName::Ident(IdentName {
            span: DUMMY_SP,
            sym: if b { "true".into() } else { "false".into() },
        })
    }
    fn from_f64(f: f64) -> Self {
        PropName::Num(f.into())
    }
}

fn is_valid_ident_name(ident: &str) -> bool {
    use swc_ecma_ast::Ident;
    for (i, c) in ident.chars().enumerate() {
        if i == 0 {
            if Ident::is_valid_start(c) {
                continue;
            }
            return false;
        }
        if Ident::is_valid_continue(c) {
            continue;
        }
        return false;
    }
    return true;
}
