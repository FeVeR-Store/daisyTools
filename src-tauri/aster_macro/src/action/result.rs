use aster_common::nesting::{parse_nesting, NESTING_PRIFIX};
use aster_common::utils::{create_string_literal, IntoIdent};
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use serde_json::Map;
use syn::ItemType;
use syn::{
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Brace, Comma, Pub},
    Attribute, ExprCast, ExprClosure, ExprStruct, Field, Fields, FieldsNamed, FieldsUnnamed, FnArg,
    Ident, ImplItem, ImplItemFn, ItemEnum, ItemImpl, ItemStruct, ItemTrait, Lit, Meta, TraitItem,
    TraitItemFn, Type, Variant, Visibility,
};

use crate::utils::{any_error, create_vec_expr, data_wrapper, prepend_underscore, result};
#[derive(Debug)]
pub(crate) struct Context<'a> {
    /// result 宏修饰的枚举名
    pub result_ident: &'a Ident,
    /// 各类实现的返回值，即 Result<CardResult, Box<dyn Error>>
    pub return_type: &'a Type,

    /// ReturnExt trait
    pub result_ext_trait: &'a mut ItemTrait,
    pub result_ext_impl: &'a mut ItemImpl,
}

type VariantHandlerOutput = (Vec<TraitItem>, Vec<ImplItem>, Vec<TokenStream2>);

struct ArgsContext<'a> {
    nesting_ident: &'a str,
}

fn parse_arg(
    field: &mut Field,
    ArgsContext { nesting_ident }: &ArgsContext,
    _context: &mut Context,
) -> VariantHandlerOutput {
    let (trait_item, impl_item, mut stmt_list) = VariantHandlerOutput::default();

    // nesting
    loop {
        if let Type::Macro(m) = &field.ty {
            if !m.mac.path.is_ident("nesting") {
                break;
            }
            let mut stmt = vec![];
            let mut _nesting_plug = Map::new();
            let ty = parse_nesting(
                &field,
                &m.mac.tokens,
                nesting_ident,
                (&mut stmt, &mut _nesting_plug),
            )
            .unwrap();
            field.ty = ty;
            stmt_list.extend(stmt);
        };
        break;
    }
    // plug
    loop {
        if let Type::Macro(m) = &field.ty {
            if !m.mac.path.is_ident("plug") {
                break;
            }
            let token = &m.mac.tokens;
            let cast: ExprCast = parse_quote!(#token);
            let (expr, _) = (cast.expr.as_ref(), cast.ty.as_ref());

            field.ty = parse_quote!(#expr);
        };
        break;
    }

    (trait_item, impl_item, stmt_list)
}

/// 创建分支Result，根据枚举生成的Result，直接将返回值对应到workflow的下一个分支
pub fn result_branch_impl(input: TokenStream) -> TokenStream {
    // 需要用到enum上
    let mut result_item = parse_macro_input!(input as ItemEnum);
    let result_ident = &result_item.ident;

    let inner_ident = prepend_underscore(&result_ident);
    let mut trait_item: ItemTrait = parse_quote! {
        trait #inner_ident {}
    };
    let mut trait_impl_item: ItemImpl = parse_quote! {
        impl #inner_ident for #result_ident {}
    };
    let result_ext_ident = Ident::new(
        &format!("_ResultExt{}", &result_ident.to_string()),
        Span::call_site(),
    );
    let result_ext_trait: &mut ItemTrait = &mut parse_quote! {
        trait #result_ext_ident<T> {}
    };

    let result_ext_impl: &mut ItemImpl = &mut parse_quote! {
        impl<T, E> #result_ext_ident<T> for ::std::result::Result<T, E>
        where
            E: ::std::fmt::Debug + 'static,
        {}
    };

    let mut stmts: Vec<TokenStream2> = vec![];

    // 存储TraitItem，枚举对应的成员将在此处以trait方式声明
    let mut result_trait_members: Vec<TraitItem> = vec![];

    let any_err = any_error();
    let return_type = &result(parse_quote!(#result_ident), any_err.clone());

    let mut context = Context {
        result_ident,
        return_type,
        result_ext_impl,
        result_ext_trait,
    };

    let mut result_struct: ItemType = parse_quote! {
        // pub struct #result_ident {
        //     pub variant: &'static str,
        //     pub data: Data,
        // }
        pub type #result_ident = ::common::ty::CardResult;
    };

    result_item
        .attrs
        .retain(|e| !(e.path().is_ident("source") || e.path().is_ident("result")));
    result_struct.attrs = result_item.attrs;

    for variant in &result_item.variants {
        let attribute = parse_result_attr(&variant.attrs);
        let (trait_items, impl_items, stmt_items) = match &variant.fields {
            Fields::Unnamed(fields) => {
                handle_unnamed_variant(variant, fields, attribute, &mut context)
            }
            Fields::Named(fields) => handle_named_variant(variant, fields, attribute, &mut context),
            Fields::Unit => handle_unit_variant(variant, attribute, &mut context),
        };
        result_trait_members.extend(trait_items);
        trait_impl_item.items.extend(impl_items);
        stmts.extend(stmt_items);
    }

    trait_item.items = result_trait_members;

    let Context {
        result_ext_trait,
        result_ext_impl,
        ..
    } = context;

    let mut token: TokenStream = quote! {
        #result_struct
        // impl ::std::convert::Into<::common::ty::CardResult> for #result_ident {
        //     fn into(self) -> ::common::ty::CardResult {
        //         ::common::ty::CardResult {
        //             variant: self.variant,
        //             data: self.data,
        //         }
        //     }
        // }
        #result_ext_trait
        #result_ext_impl
        #trait_item
        #trait_impl_item
    }
    .into();
    let stmts: TokenStream = TokenStream2::from_iter(stmts).into();

    token.extend(stmts);
    token
}

pub fn handle_unnamed_variant(
    variant: &Variant,
    unname: &FieldsUnnamed,
    ParsedAttribute { into_error, .. }: ParsedAttribute,
    context: &mut Context,
) -> VariantHandlerOutput {
    let ident = variant.ident.clone();
    let ident_name = ident.to_string();
    let ident_lit = create_string_literal(&ident_name);

    let result_ident = context.result_ident.clone();
    let return_type = context.return_type.clone();

    let mut members = unname.unnamed.clone();

    let mut stmt_list = vec![];
    for f in members.iter_mut() {
        let (_, _, stmt) = parse_arg(
            f,
            &ArgsContext {
                nesting_ident: &&format!(
                    "{}_{}",
                    &context.result_ident.to_string(),
                    &variant.ident.to_string(),
                    // &ident_name
                ),
            },
            context,
        );
        stmt_list.extend(stmt);
    }

    if into_error {
        handle_into_error(&ident, &ident_lit, context);
    }

    // 构建函数的参数列表
    let args: Punctuated<FnArg, Comma> = members
        .iter()
        .enumerate()
        .map(|(idx, Field { ty, .. })| {
            let arg_ident = &format!("arg_{}", idx).into_ident();
            let arg: FnArg = parse_quote! {
                #arg_ident: #ty
            };
            arg
        })
        .collect();

    // 构建TraitItem
    let item_fn: TraitItemFn = parse_quote! {
        #[allow(non_snake_case)]
        fn #ident(#args) -> #return_type;
    };
    let data_expr = if members.len() == 1 {
        let Some(ty) = members.first() else {
            panic!("Never!");
        };
        data_wrapper(&"arg_0".into_ident(), &ty.ty.to_token_stream().to_string())
    } else {
        let tokens: Vec<TokenStream> = members
            .iter()
            .enumerate()
            .map(|(i, _)| {
                Ident::new(&format!("arg_{}", i), Span::call_site())
                    .into_token_stream()
                    .into()
            })
            .collect();
        let data_expr = create_vec_expr(tokens);
        parse_quote!(::common::ty::Data::Vec(#data_expr))
    };

    // 构建ImplItem
    let impl_item: ImplItem = parse_quote! {
        fn #ident(#args) -> #return_type {
            ::std::result::Result::Ok(#result_ident {
                variant: #ident_lit,
                data: #data_expr,
            })
        }
    };

    (vec![TraitItem::Fn(item_fn)], vec![impl_item], stmt_list)
}

fn handle_named_variant(
    variant: &Variant,
    named: &FieldsNamed,
    ParsedAttribute {
        into_error,
        raw,
        into_branch,
        ..
    }: ParsedAttribute,
    context: &mut Context,
) -> VariantHandlerOutput {
    // named 形似结构体，需要把它转化为一个struct
    // 默认情况下，需要传递一个同名的结构体
    // 如果使用了 #[branch] 标记，则转化为子分支
    let ident = variant.ident.clone();
    let ident_name = ident.to_string();

    let inner_ident = prepend_underscore(&ident);

    if !into_branch {
        // 如果没有branch，那么就创建一个struct，然后创建参数为该结构体的trait和impl
        // 首先创建struct
        let branch_ident = format!(
            "{}_{}_{}",
            NESTING_PRIFIX,
            &context.result_ident.to_string(),
            &ident_name
        )
        .as_str()
        .into_ident();

        let mut st: ItemStruct = parse_quote! {
            #[derive(::std::fmt::Debug, ::serde::Deserialize, ::serde::Serialize)]
            struct #branch_ident {}
        };

        let mut stmt: Vec<TokenStream2> = vec![];

        let mut fields: Punctuated<Field, Comma> = named.named.clone();
        // 将字段的可见性改为pub
        for f in fields.iter_mut() {
            let ident_name = if let Some(ident) = &f.ident {
                ident.to_string()
            } else {
                String::from("unknown")
            };
            f.vis = Visibility::Public(Pub {
                span: Span::call_site(),
            });
            let (_, _, stmt_list) = parse_arg(
                f,
                &ArgsContext {
                    nesting_ident: &&format!(
                        "{}_{}_{}",
                        &context.result_ident.to_string(),
                        &variant.ident.to_string(),
                        &ident_name
                    ),
                },
                context,
            );
            stmt.extend(stmt_list);
        }

        st.fields = Fields::Named(FieldsNamed {
            brace_token: Brace(Span::call_site()),
            named: fields,
        });

        let return_type = context.return_type;
        let result_ident = context.result_ident;
        // 然后创建trait
        let item_trait = parse_quote! {
            #[allow(non_snake_case)]
            fn #ident(arg_0: #branch_ident) -> #return_type;
        };

        // 然后是impl
        let ident_lit = create_string_literal(&ident.to_string());
        let item_impl = parse_quote! {
            fn #ident(arg_0: #branch_ident) -> #return_type {
                ::std::result::Result::Ok(#result_ident {
                    variant: #ident_lit,
                    data: ::common::ty::Data::Any(::serde_json::to_value(arg_0)?),
                })
            }
        };
        stmt.push(quote! {#st}.into());

        return (vec![item_trait], vec![item_impl], stmt);
    }
    let item_const = parse_quote! {
        #[allow(non_upper_case_globals)]
        const #ident: #inner_ident;
    };
    // 如果在变体上使用 #[error] 标记，则将其子分支创建为result_ext
    let into_error_branch = into_error;

    // 将常量赋值加入到impl中
    let item_const_impl: ImplItem = parse_quote! {const #ident: #inner_ident = #inner_ident;};

    let mut muilt_branch_impl: ItemImpl = parse_quote! {
      impl #inner_ident {}
    };

    let mut stmts = vec![
        // 先创建struct
        quote! {struct #inner_ident;}.into(),
        // 然后加入实现语句
        quote!(#muilt_branch_impl).into(),
    ];
    muilt_branch_impl.items = named
        .named
        .iter()
        .map(
            |Field {
                 attrs,
                 ident: id,
                 ty,
                 ..
             }| {
                // 实现子分支的fn
                let Some(sub_branch_ident) = id else {
                    panic!("Named branch in {} has no ident", &variant.ident);
                };

                let mut into_error = into_error_branch;
                let mut raw = raw;
                let ParsedAttribute {
                    into_error: i_e,
                    raw: r,
                    ..
                } = parse_result_attr(&attrs);

                if i_e {
                    into_error = true
                };
                if r {
                    raw = true
                };

                let variant_name =
                    format!("{}_{}", &ident.to_string(), sub_branch_ident.to_string());

                let variant_lit = create_string_literal(&variant_name);
                if into_error {
                    handle_into_error(sub_branch_ident, &variant_lit, context);
                };
                let Context {
                    result_ident,
                    return_type,
                    ..
                } = context;

                let item: ImplItem = match ty {
                    // 元组类型转化为多参数函数
                    Type::Tuple(tuple) => {
                        let mut emit_precision_warning = None;
                        let mut args_token: Vec<TokenStream> = vec![];
                        let args: Punctuated<FnArg, Comma> = if raw {
                            let mut container = Punctuated::new();
                            container.push(parse_quote!(arg_0: #tuple));
                            container
                        } else {
                            tuple
                                .elems
                                .iter()
                                .enumerate()
                                .map(|(idx, ele)| {
                                    let t: &str = &ele.clone().to_token_stream().to_string();
                                    match t {
                                        i @ ("i128" | "u128") => {
                                            emit_precision_warning = Some(i.to_string());
                                        }
                                        _ => (),
                                    };
                                    let ident_str = format!("arg_{}", idx);
                                    let ident = Ident::new(&ident_str, Span::call_site());
                                    args_token.push(quote!(#ident.into()).into());
                                    let arg: FnArg = parse_quote! (#ident: #ele);
                                    arg
                                })
                                .collect()
                        };

                        let warning = if let Some(i) = emit_precision_warning {
                            let warning_content = create_string_literal(&format!(
                                "Using {} may cause precision problems",
                                i
                            ));
                            quote! {#[doc = #warning_content]}
                        } else {
                            quote! {}
                        };

                        let data_expr = create_vec_expr(args_token);

                        let impl_fn: ImplItemFn = parse_quote! {
                            #warning
                            pub fn #sub_branch_ident(self, #args) -> #return_type {
                                ::std::result::Result::Ok(#result_ident {
                                    variant: #variant_lit,
                                    data: ::common::ty::Data::Vec(#data_expr),
                                })
                            }
                        };
                        ImplItem::Fn(impl_fn)
                    }
                    Type::Infer(_) => {
                        if !into_error {
                            panic!(
                                "{} {} {}",
                                "Using inferred type _ requires converting the",
                                sub_branch_ident.to_string(),
                                "branch into an error variant using #[error]"
                            )
                        } else {
                            ImplItem::Verbatim(TokenStream::new().into())
                        }
                    }
                    Type::Path(type_path) => {
                        let type_name = type_path.to_token_stream().to_string();
                        let data_expr = data_wrapper(&"arg_0".into_ident(), &type_name);
                        parse_quote! {
                            fn #sub_branch_ident(arg_0: #type_path) -> #return_type {
                                ::std::result::Result::Ok(#result_ident {
                                    variant: #variant_lit,
                                    data: #data_expr,
                                })
                            }
                        }
                    }
                    Type::Macro(_) => {
                        stmts.push(quote! { struct Token {} }.into());
                        ImplItem::Verbatim(quote! { struct Token {} }.into());
                        parse_quote! {
                            fn nesting() -> #return_type {
                                ::std::result::Result::Ok(#result_ident {
                                    variant: #variant_lit,
                                    data: $::common::ty::Data::Null,
                                })
                            }
                        }
                    }
                    _ => todo!(),
                };
                item
            },
        )
        .collect();

    // 倘若转化为错误分支，那么原有的Result成员就不需要注入
    if into_error_branch {
        VariantHandlerOutput::default()
    } else {
        (vec![item_const], vec![item_const_impl], stmts)
    }
}

fn handle_unit_variant(
    variant: &Variant,
    ParsedAttribute {
        into_error, raw, ..
    }: ParsedAttribute,
    context: &mut Context,
) -> VariantHandlerOutput {
    let variant_ident = variant.ident.clone();
    let variant_name = variant_ident.to_string();
    let variant_lit = create_string_literal(&variant_name);

    let result_ident = context.result_ident.clone();
    let return_type = &context.return_type;

    if into_error {
        let ident = Ident::new(&variant_name.to_lowercase(), Span::call_site());
        handle_into_error(
            if raw { &variant_ident } else { &ident },
            &variant_lit,
            context,
        );
        return VariantHandlerOutput::default();
    }
    let item_const = parse_quote! {
        #[allow(non_upper_case_globals)]
        const #variant_ident: #return_type;
    };
    // 将常量赋值加入到impl中
    let item_const_impl: ImplItem = parse_quote! {
        const #variant_ident: #return_type =
            ::std::result::Result::Ok(
            #result_ident {
                variant: #variant_lit,
                data: ::common::ty::Data::Null
            }
        );
    };

    (vec![item_const], vec![item_const_impl], vec![])
}

pub(crate) struct ParsedAttribute {
    pub into_error: bool,
    pub into_branch: bool,
    pub raw: bool,
}

fn parse_result_attr(attrs: &Vec<Attribute>) -> ParsedAttribute {
    let mut pa = ParsedAttribute {
        into_error: false,
        raw: false,
        into_branch: false,
    };

    for attr in attrs {
        let attr_ident = attr.path().require_ident().unwrap();
        let attr_str = attr_ident.to_string();
        let attr_str: &str = Box::leak(Box::new(attr_str));
        match attr_str {
            "branch" => {
                match &attr.meta {
                    Meta::List(list) => {
                        if &list.tokens.to_string() == "error" {
                            pa.into_error = true;
                        }
                    }
                    _ => panic!("only receive error"),
                }
                pa.into_branch = true;
            }
            "raw" => {
                pa.raw = true;
            }
            _ => (),
        }
    }

    pa
}

fn handle_into_error(
    branch_ident: &Ident,
    branch_lit: &Lit,
    Context {
        result_ext_trait,
        result_ext_impl,
        ..
    }: &mut Context,
) {
    let any_err = any_error();
    let ext_return_type = result(parse_quote!(T), any_err);

    result_ext_trait.items.push(TraitItem::Verbatim(
        quote! {fn #branch_ident(self) -> #ext_return_type;},
    ));

    let error_wrap: ExprStruct = parse_quote!(::common::ty::ErrorWrap {
        inner: ::std::boxed::Box::new(e),
        branch: ::std::string::String::from(#branch_lit)
    });

    let map_err: ExprClosure = parse_quote! {|e| #error_wrap};

    result_ext_impl.items.push(parse_quote! {
       fn #branch_ident(self) -> #ext_return_type {
           ::std::result::Result::Ok(self.map_err(#map_err)?)
       }
    });
}
