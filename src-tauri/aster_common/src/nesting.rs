use crate::utils::{normalize_type, GetIdent, IntoIdent, IntoString};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use serde_json::{Map, Value, json};
use syn::{
    Attribute, ExprCast, Field, Fields, FieldsNamed, ItemStruct, Type, TypeMacro, Visibility,
    parse_quote, token::Pub,
};
type NestingResult = Result<Type, syn::Error>;

#[derive(Debug, Default)]
struct ParsedAttribute {
    rename: String,
}

pub const NESTING_PRIFIX: &str = "__NESTING";

pub fn parse_nesting(
    field: &Field,
    token: &proc_macro2::TokenStream,
    ident: &str,
    (stmt_list, plug): (&mut Vec<TokenStream>, &mut Map<String, Value>),
) -> NestingResult {
    let attr = parse_attr(&field.attrs);

    let ident = if attr.rename.is_empty() {
        ident
    } else {
        attr.rename.as_str()
    };
    let ident_name: &str = if ident.starts_with(NESTING_PRIFIX) {
        &format!("{}_{}", ident, field.get_ident("unknown").to_string())
    } else {
        &format!("{}_{}", NESTING_PRIFIX, ident)
    };

    let actual_ident = if attr.rename.is_empty() {
        ident_name
    } else {
        ident
    }
    .into_ident();

    let mut st: ItemStruct = parse_quote! {
        #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize)]
        struct #actual_ident { #token }
    };
    let Fields::Named(FieldsNamed {
        named: mut struct_fields,
        ..
    }) = st.fields
    else {
        panic!("Nested content must use named fields")
    };
    for f in struct_fields.iter_mut() {
        f.vis = Visibility::Public(Pub {
            span: Span::call_site(),
        });

        let sub_ident_name = f.get_ident("unknown").to_string();

        let mut plug_type = Value::Null;
        match &f.ty {
            Type::Macro(TypeMacro { mac, .. }) => {
                if mac.path.is_ident("nesting") {
                    let mut plug = Map::new();
                    plug.insert("\0type".to_string(), json!("object"));
                    let ty = parse_nesting(
                        f,
                        &mac.tokens,
                        &format!("{}_{}", ident_name, &sub_ident_name),
                        (stmt_list, &mut plug),
                    )?;
                    f.ty = ty;
                    plug_type = plug.into();
                } else if mac.path.is_ident("plug") {
                    let token = &mac.tokens;
                    let cast: ExprCast = parse_quote!(#token);
                    let (expr, ty) = (cast.expr.as_ref(), cast.ty.as_ref());
                    plug_type = match ty {
                        Type::Macro(m) => {
                            let mut plug = Map::new();
                            plug.insert("\0type".to_string(), json!("object"));

                            let _ = parse_nesting(&f, &m.mac.tokens, "", (&mut vec![], &mut plug));
                            plug.into()
                        }
                        t @ _ => normalize_type(&t.into_string()),
                    };
                    f.ty = parse_quote!(#expr);
                }
            }
            ty @ _ => plug_type = normalize_type(&ty.into_string()),
        }
        plug.insert(sub_ident_name, plug_type);
    }

    st.fields = Fields::Named(FieldsNamed {
        brace_token: Default::default(),
        named: struct_fields,
    });
    stmt_list.push(quote!(#st).into());
    let ty: Type = parse_quote!(#actual_ident);
    Ok(ty)
}

fn parse_attr(attrs: &Vec<Attribute>) -> ParsedAttribute {
    let mut pa = ParsedAttribute::default();

    for attr in attrs {
        let attr_ident = attr.path().require_ident().unwrap();
        let attr_str = attr_ident.to_string();
        let attr_str: &str = &attr_str;
        match attr_str {
            "rename" => {
                pa.rename = attr.meta.path().to_token_stream().to_string();
            }
            _ => (),
        }
    }

    pa
}
