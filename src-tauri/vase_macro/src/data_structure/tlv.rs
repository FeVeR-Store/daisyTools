use aster_common::{
    rust::{expr::FromLit, lit::FromBase},
    utils::{IntoIdent, IntoString},
};
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Arm, Expr, ExprLit, ExprMatch, Field, Fields, Ident, ImplItem, ItemEnum, ItemFn, ItemImpl,
    ItemStruct, Lit, Type, Variant, parse_macro_input, parse_quote,
};

#[derive(Debug, Clone)]
pub(super) enum ParseMode {
    Bytes,
    String,
    Usize(u8),
    Cast(u8, Ident),
    Array(Expr),
}

pub fn tlv_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(input as ItemEnum);
    let enum_ident = &enum_item.ident;

    let mut type_size = None;
    let mut len_size = None;

    let mut collect_type = None;

    let mut constructors: Vec<ImplItem> = vec![];
    for var in enum_item.variants.iter() {
        let var_ident = var.ident.clone();
        let var_name = var_ident.to_string();
        if let Some((_, expr)) = &var.discriminant {
            match &expr {
                Expr::Path(expr_path) => {
                    let Some(ty) = expr_path.path.segments.last() else {
                        break;
                    };
                    let ty = ty.ident.to_string();
                    let t: &'static str = ty.leak();

                    if var_name == "_collect" {
                        collect_type = Some(t.to_string());
                        continue;
                    }
                    let size = match t {
                        u @ ("u8" | "u16" | "u32" | "u64" | "u128") => (u_size(u), u),
                        _ => break,
                    };

                    if var_name == "_type" {
                        type_size = Some(size);
                    } else if var_name == "_len" {
                        len_size = Some(size);
                    } else {
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    let variant_num = enum_item.variants.len() - 2;
    let (Some((type_size, type_ty)), Some((len_size, len_ty))) = (type_size, len_size) else {
        panic!(
            "tlv needs to specify the integer type of type and length, try _type = {}, _len = u16",
            recommended_type(variant_num)
        );
    };
    let len_ident = len_ty.into_ident();
    if variant_num > 2_usize.pow((8 * type_size) as u32) {
        panic!(
            "type size is too small to represent {} types",
            recommended_type(variant_num)
        )
    }

    let payload_ident = &format!("_{}Payload", enum_ident.to_string()).into_ident();

    let mut payload_enum: ItemEnum = parse_quote!(
        #[derive(Debug,Clone)]
        enum #payload_ident {}
    );

    let mut variant_list = vec![];
    let mut enum_list = vec![];

    let mut payload_to_bytes_match: ExprMatch = parse_quote!(match &self.0 {});

    let mut i = 0;

    for var in enum_item.variants.iter() {
        let var_ident = &var.ident;
        match &var.fields {
            Fields::Unnamed(field) => {
                if field.unnamed.len() != 1 {
                    panic!("Only one field is supported")
                }
                let Some(f) = field.unnamed.first() else {
                    panic!("Only one field is supported")
                };
                let mut storage_type = f.ty.clone();
                let mut arg_type = None;
                let mut parse_mode = ParseMode::Bytes;

                match &f.ty {
                    Type::Path(pa) => {
                        let ty = pa.path.into_string();
                        match ty.as_str() {
                            u @ ("u8" | "u16" | "u32" | "u64" | "u128") => {
                                parse_mode = ParseMode::Usize(u_size(u) as u8)
                            }
                            "::bytes::Bytes" | "bytes::Bytes" | "Bytes" => {
                                parse_mode = ParseMode::Bytes
                            }
                            // [2] Added: Recognition for String type
                            "::std::string::String" | "std::string::String" | "String" => {
                                parse_mode = ParseMode::String
                            }
                            _ => panic!(
                                "Only accept u8, u16, u32, u64, u128, [u8; LEN], bytes::Bytes, String"
                            ),
                        }
                    }
                    Type::Macro(mac) => {
                        let ident = mac.mac.path.into_string();
                        match ident.as_str() {
                            ty_str @ ("u8" | "u16" | "u32" | "u64" | "u128") => {
                                let tokens = &mac.mac.tokens;
                                let ty = ty_str.into_ident();
                                let item_enum: ItemEnum = parse_quote! {
                                    #[repr(#ty)]
                                    #[derive(Debug, Clone, Copy)]
                                    pub enum #var_ident {
                                        #tokens
                                    }
                                };
                                let mut from_match: ExprMatch = parse_quote! {
                                    match i { }
                                };
                                enum EnumType {
                                    Unknown,
                                    CLike,
                                    Nominal,
                                }
                                let mut enum_type = EnumType::Unknown;
                                let mut fallback_index = 0;
                                let mut use_fallback = false;
                                from_match.arms.extend(
                                    item_enum.variants.clone().into_iter().enumerate().map(
                                        |(i, f)| {
                                            let ident = f.ident;
                                            if matches!(enum_type, EnumType::Unknown) {
                                                if f.discriminant.is_none() {
                                                    enum_type = EnumType::Nominal;
                                                } else {
                                                    enum_type = EnumType::CLike;
                                                }
                                            }
                                            match enum_type {
                                                EnumType::Nominal => {
                                                    let i = i;
                                                    parse_quote!(#i => #var_ident::#ident)
                                                }
                                                EnumType::CLike => {
                                                    if use_fallback {
                                                        fallback_index += 1;
                                                        parse_quote!(#fallback_index => #var_ident::#ident)
                                                    } else if let Some(discriminant) = f.discriminant {
                                                        if let Expr::Lit(ExprLit {lit: Lit::Int(lit), ..}) = discriminant.1 {
                                                            let i = lit.base10_parse::<usize>().unwrap();
                                                            fallback_index = i;
                                                            parse_quote!(#fallback_index => #var_ident::#ident)
                                                        } else {
                                                            parse_quote!(#fallback_index => #var_ident::#ident)
                                                        }

                                                    } else {
                                                        use_fallback = true;
                                                        fallback_index += 1;
                                                        parse_quote!(#fallback_index => #var_ident::#ident)
                                                    }
                                                }
                                                EnumType::Unknown => panic!("Never!")
                                            }
                                        },
                                    ),
                                );

                                from_match
                                    .arms
                                    .push(parse_quote!(_ => panic!("Unknown field")));
                                let enum_impl: ItemImpl = parse_quote!(impl #var_ident {
                                    fn from(i: impl Into<usize>) -> Self {
                                        let i = i.into();
                                        #from_match
                                    }
                                });

                                let enum_item: proc_macro2::TokenStream = parse_quote! {
                                    #item_enum
                                    #enum_impl
                                };
                                enum_list.push(enum_item);

                                storage_type = parse_quote!(#ty);
                                arg_type = Some(parse_quote!(#var_ident));
                                parse_mode = ParseMode::Cast(u_size(ty_str) as u8, ty);
                            }
                            _ => (),
                        }
                    }
                    Type::Array(arr) => {
                        if let Type::Path(path) = &arr.elem.as_ref()
                            && path.path.is_ident("u8")
                        {
                            parse_mode = ParseMode::Array(arr.len.clone());
                        } else {
                            panic!(
                                "Only accept u8, u16, u32, u64, u128, [u8; LEN], bytes::Bytes, String, received: {:?}",
                                arr.into_string()
                            );
                        }
                    }
                    n => panic!(
                        "Only accept u8, u16, u32, u64, u128, [u8; LEN], bytes::Bytes, String, received: {:?}",
                        n
                    ),
                }
                let arg_type = arg_type.unwrap_or(f.ty.clone());

                let type_id =
                    Expr::from_lit(Lit::from_int_str(&format!("{}_{}", i as i64, type_ty)));

                i += 1;

                let mut payload_len: Expr =
                    parse_quote!((::std::mem::size_of::<#storage_type>() as #len_ident));
                let mut value_slice: Expr = parse_quote!(&(*value as #storage_type).to_be_bytes());

                match &storage_type {
                    Type::Array(arr) if arr.elem.into_string().as_str() == "u8" => {
                        let len = Expr::from_lit(Lit::from_int_str(&format!(
                            "{}_{}",
                            &arr.len.into_string(),
                            len_ty
                        )));
                        payload_len = parse_quote!(#len);
                        value_slice = parse_quote!(value);
                    }
                    _ => (),
                }

                if matches!(parse_mode, ParseMode::Bytes) {
                    payload_len = parse_quote!((value.len() as #len_ident));
                    value_slice = parse_quote!(&value);
                }

                // [3] Added: Encoding logic for String
                if matches!(parse_mode, ParseMode::String) {
                    payload_len = parse_quote!((value.len() as #len_ident));
                    value_slice = parse_quote!(value.as_bytes());
                }

                payload_to_bytes_match.arms.push(parse_quote! {
                    #payload_ident::#var_ident(value) => {
                        out.extend_from_slice(&#type_id.to_be_bytes());
                        out.extend_from_slice(&#payload_len.to_be_bytes());
                        out.extend_from_slice(#value_slice);
                    }
                });
                let mut constructor: ItemFn = parse_quote! {
                    pub fn #var_ident(value: #arg_type) -> Self {
                        Self(#payload_ident::#var_ident(value))
                    }
                };
                constructor.attrs = var.attrs.clone();
                constructors.push(parse_quote!(#constructor));
                variant_list.push((var_ident.clone(), arg_type, parse_mode));
            }
            Fields::Unit => {
                if matches!(
                    var_ident.into_string().as_str(),
                    "_type" | "_len" | "_collect"
                ) {
                    continue;
                } else {
                    panic!("Only unnamed fields are supported");
                }
            }
            _ => panic!("Only unnamed fields are supported"),
        }
    }

    payload_enum.variants.extend(
        variant_list
            .iter()
            .map(|(ident, ty, _)| parse_quote!(#ident(#ty)))
            .collect::<Vec<Variant>>(),
    );
    let mut match_impl: ExprMatch = parse_quote! {
        match member.0 {}
    };

    let struct_name = &enum_item.ident;
    let mut collect_token = quote! {};

    if let Some(collect_type) = collect_type {
        let collect_type = collect_type.into_ident();

        let mut collect_impl: ItemStruct = parse_quote! {
            #[allow(non_snake_case)]
            #[derive(Debug, Clone, Default)]
            pub struct #collect_type {
                inner: ::bytes::BytesMut,
            }
        };
        let Fields::Named(named) = &mut collect_impl.fields else {
            panic!("???");
        };

        named.named.extend(
            variant_list
                .iter()
                .map(|(ident, ty, _)| parse_quote!(pub #ident: ::std::option::Option<#ty>))
                .collect::<Vec<Field>>(),
        );

        match_impl.arms.extend(
            variant_list
                .iter()
                .map(|(ident, _ , _)| {
                    parse_quote!(#payload_ident::#ident(payload) => self.#ident = ::std::option::Option::Some(payload))
                })
                .collect::<Vec<Arm>>(),
        );

        let type_ident = type_ty.into_ident();
        let len_ident = len_ty.into_ident();

        let mut bytes_to_payload_match: ExprMatch = parse_quote! {
            match ty { }
        };

        bytes_to_payload_match.arms.extend(variant_list.iter().enumerate().map(|(i, (ident, ty, parse_mode))| {
            let i = Expr::from_lit(Lit::from_int(i as i64));
            match parse_mode.clone() {
                ParseMode::Cast(size, type_ty) => {
                    let size = Expr::from_lit(Lit::from_int(size as i64));
                    parse_quote! {
                        #i => {
                            let mut payload_buf = [0u8; #size];
                            payload_buf.copy_from_slice(&buf[pointer..pointer + len]);
                            let i = #type_ty::from_be_bytes(payload_buf);

                            metadata.#ident = ::std::option::Option::Some(#ident::from(i));
                        }
                    }
                }
                ParseMode::Array(len) => {
                    parse_quote! {
                        #i => {
                            let mut payload_buf = [0u8; #len];
                            payload_buf.copy_from_slice(&buf[pointer..pointer + len]);

                            metadata.#ident = ::std::option::Option::Some(payload_buf);
                        }
                    }
                }
                ParseMode::Bytes => {
                    parse_quote! {
                        #i => {
                            let mut payload_buf = ::bytes::BytesMut::new();
                            payload_buf.extend_from_slice(&buf[pointer..pointer + len]);

                            metadata.#ident = ::std::option::Option::Some(payload_buf.freeze());
                        }
                    }
                }
                // [4] Added: Decoding logic for String
                ParseMode::String => {
                    parse_quote! {
                        #i => {
                            let mut payload_vec = ::std::vec![0u8; len];
                            payload_vec.copy_from_slice(&buf[pointer..pointer + len]);
                            // 使用 unwrap 或 expect，假设TLV编码的String必须是合法的UTF-8
                            let s = ::std::string::String::from_utf8(payload_vec).expect("TLV String field invalid UTF-8");

                            metadata.#ident = ::std::option::Option::Some(s);
                        }
                    }
                }
                ParseMode::Usize(size) => {
                    let size = Expr::from_lit(Lit::from_int(size as i64));
                    parse_quote! {
                        #i => {
                            let mut payload_buf = [0u8; #size];
                            payload_buf.copy_from_slice(&buf[pointer..pointer + len]);
                            let i = #ty::from_be_bytes(payload_buf);

                            metadata.#ident = ::std::option::Option::Some(i);
                        }
                    }
                }
            }
        }).collect::<Vec<Arm>>());
        bytes_to_payload_match.arms.push(parse_quote!(_ => ()));

        collect_token = quote! {
            #collect_impl
            impl #collect_type {
                fn add_meta(&mut self, member: #struct_name) {
                    member.encode_into(&mut self.inner);
                    #match_impl
                }
                pub fn to_bytes(self) -> ::bytes::Bytes {
                    self.inner.freeze()
                }
                pub fn from_bytes(buf: &[u8]) -> Self {
                    let mut metadata = Self::default();
                    let mut pointer = 0;
                    loop {
                        if pointer >= buf.len() {
                            break;
                        }
                        // 先读type_len个字节，确定类型
                        let mut ty_buf = [0u8; #type_size];
                        ty_buf.copy_from_slice(&buf[pointer..pointer + #type_size]);
                        let ty = #type_ident::from_be_bytes(ty_buf);
                        pointer += #type_size;
                        // 然后读取数据长度
                        let mut len_buf = [0u8; #len_size];
                        len_buf.copy_from_slice(&buf[pointer..pointer + #len_size]);
                        let len = #len_ident::from_be_bytes(len_buf) as usize;

                        pointer += #len_size;
                        // 读取数据
                        #bytes_to_payload_match

                        pointer += len;

                    }
                    metadata
                }
                pub fn len(&self) -> usize {
                    self.inner.len()
                }
            }

            impl ::std::convert::Into<::bytes::Bytes> for #collect_type {
                fn into(self) -> ::bytes::Bytes {
                    self.inner.freeze()
                }
            }
            impl ::std::ops::Add<#struct_name> for #collect_type {
                type Output = #collect_type;
                fn add(mut self, member: #struct_name) -> Self::Output {
                    self.add_meta(member);
                    self
                }
            }
            impl ::std::ops::Add<::std::option::Option<#struct_name>> for #collect_type {
                type Output = #collect_type;
                fn add(mut self, member: ::std::option::Option<#struct_name>) -> Self::Output {
                    if let Some(member) = member {
                        self.add_meta(member);
                    }
                    self
                }
            }
            impl ::std::ops::Add<#struct_name> for #struct_name {
                type Output = #collect_type;
                fn add(self, member: #struct_name) -> Self::Output {
                    let mut collect = #collect_type::default();
                    collect.add_meta(self);
                    collect.add_meta(member);
                    collect
                }
            }
        };
    }

    let mut item_impl: ItemImpl = parse_quote! {
        #[allow(non_snake_case)]
        impl #struct_name {
            fn encode_into(&self, out: &mut ::bytes::BytesMut) {
                #payload_to_bytes_match
            }
        }
    };

    item_impl.items.extend(constructors);

    let mut struct_impl: ItemStruct = parse_quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name(#payload_ident);
    };

    struct_impl.attrs = enum_item.attrs;

    let mut tokens = quote! {
        #struct_impl
        #payload_enum
        #item_impl
        #collect_token
    };
    tokens.extend(
        enum_list
            .iter()
            .map(|e| e.into_token_stream())
            .collect::<Vec<proc_macro2::TokenStream>>(),
    );
    tokens.into()
}

#[inline]
fn recommended_type(variant_num: usize) -> &'static str {
    for (ty, max) in [
        ("u8", u8::MAX as usize),
        ("u16", u16::MAX as usize),
        ("u32", u32::MAX as usize),
        ("u64", u64::MAX as usize),
        ("u128", u128::MAX as usize),
    ] {
        if variant_num > max {
            continue;
        } else {
            return ty;
        }
    }
    return "u128";
}

#[inline]
fn u_size(size: &str) -> usize {
    match size {
        "u8" => size_of::<u8>(),
        "u16" => size_of::<u16>(),
        "u32" => size_of::<u32>(),
        "u64" => size_of::<u64>(),
        "u128" => size_of::<u128>(),
        _ => panic!("never!"),
    }
}
