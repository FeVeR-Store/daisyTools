use proc_macro2::Span;
use syn;
use syn::{
    parse_quote,
    punctuated::Punctuated,
    token::{Brace, Comma},
    Expr, ExprMacro, Field, FieldPat, Fields, FieldsNamed, Ident, ItemStruct, Member, Pat,
    PatIdent, PatStruct, Token, Type, Visibility,
};
use proc_macro::TokenStream;

pub fn create_struct_with_dynamic_fields(
    struct_name: &str,
    fields: Vec<(String, &Box<Type>)>,
) -> ItemStruct {
    // 创建结构体名
    let ident = Ident::new(struct_name, Span::call_site());

    // 动态构造字段向量
    let named_fields: Vec<Field> = fields
        .into_iter()
        .map(|(field_name, field_type)| Field {
            mutability: syn::FieldMutability::None,
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            ident: Some(Ident::new(&field_name, Span::call_site())),
            colon_token: Some(Token![:](Span::call_site())),
            ty: *field_type.clone(),
        })
        .collect();

    // 构造 FieldsNamed
    let fields_named = FieldsNamed {
        brace_token: syn::token::Brace {
            ..Default::default()
        },
        named: named_fields.into_iter().collect(),
    };

    ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(syn::token::Pub {
            ..Default::default()
        }),
        struct_token: Token![struct](Span::call_site()),
        ident,
        generics: Default::default(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    }
}

pub fn create_destructuring_pattern(struct_name: &str, fields: Vec<String>) -> Pat {
    let struct_name = Ident::new(struct_name, Span::call_site());

    // 构造字段成员的 PatIdent
    let pat_fields = fields
        .into_iter()
        .map(|field_ident| {
            let field_ident = Ident::new(&field_ident, Span::call_site());
            FieldPat {
                attrs: Vec::new(),
                member: Member::Named(field_ident.clone()),
                colon_token: None,
                pat: Box::new(Pat::Ident(PatIdent {
                    attrs: Vec::new(),
                    by_ref: None,
                    mutability: None,
                    ident: field_ident,
                    subpat: None,
                })),
            }
        })
        .collect::<Punctuated<_, Comma>>();

    Pat::Struct(PatStruct {
        attrs: Vec::new(),
        path: syn::Path::from(struct_name),
        brace_token: Brace {
            ..Default::default()
        },
        fields: pat_fields,
        rest: None,
        qself: None,
    })
}

pub fn create_vec_expr(ele: Vec<TokenStream>) -> Expr {
    // 参数表达式列表
    let args: Punctuated<proc_macro2::TokenStream, Comma> = ele
        .iter()
        .map(|e| {
            let t: proc_macro2::TokenStream = e.clone().into();
            t
        })
        .collect();
    let expr: ExprMacro = parse_quote!(vec![#args]);
    Expr::Macro(expr)
}

pub fn prepend_underscore(ident: &Ident) -> Ident {
    let new_name = format!("_{}", ident);
    Ident::new(&new_name, ident.span())
}

pub fn data_wrapper(ident: &Ident, ty: &str) -> Expr {
    match ty {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
        | "usize" => parse_quote!(::common::ty::Data::Int(#ident.into())),
        "f16" | "f32" | "f64" | "f128" => parse_quote!(::common::ty::Data::Float(#ident.into())),
        "Code" | "Text" | "String" => parse_quote!(::common::ty::Data::String(#ident)),
        "bool" => parse_quote!(::common::ty::Data::Bool(#ident)),
        "()" | "None" => parse_quote!(::common::ty::Data::Null(())),
        _ => parse_quote!(::common::ty::Data::Any(#ident.into())),
    }
}

pub fn any_error() -> Type {
    parse_quote!(::std::boxed::Box<dyn ::std::error::Error>)
}

pub fn result(ok: Type, err: Type) -> Type {
    parse_quote!(::std::result::Result<#ok, #err>)
}
