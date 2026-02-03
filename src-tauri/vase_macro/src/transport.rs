use aster_common::utils::IntoString;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprStruct, FieldValue, Ident, ItemFn, ItemStruct, PatType, Token, Type, braced,
    bracketed, parenthesized, parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated,
    token::Comma,
};
#[derive(Debug)]
struct TransportField(Punctuated<(Ident, Option<(Type, Option<Expr>)>), Token![,]>);

impl Parse for TransportField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Punctuated::new();
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let ty = if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let ty = input.parse()?;

                let default_val = if input.peek(Token![=]) {
                    let _: Token![=] = input.parse()?;
                    let val: Expr = input.parse()?;
                    Some(val)
                } else {
                    None
                };
                Some((ty, default_val))
            } else {
                None
            };

            fields.push((ident, ty));
            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>();
            }
        }
        Ok(Self(fields))
    }
}

#[derive(Debug)]
struct TransportInput {
    name: Ident,
    _lt: Token![<],
    adapter: Ident,
    _gt: Token![>],
    args: Punctuated<PatType, Token![,]>,
    fields: TransportField,
}

impl Parse for TransportInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let _lt: Token![<] = input.parse()?;
        let adapter: Ident = input.parse()?;
        let _gt: Token![>] = input.parse()?;

        let args;
        let _parent = parenthesized!(args in input);
        let args = args.parse_terminated(PatType::parse, Token![,])?;

        let field;
        let _braced = braced!(field in input);

        let fields = field.parse()?;

        Ok(Self {
            name,
            _lt,
            adapter,
            _gt,
            args,
            fields,
        })
    }
}

pub fn transport_impl(input: TokenStream) -> TokenStream {
    let TransportInput {
        name,
        args,
        fields,
        adapter,
        ..
    } = parse_macro_input!(input as TransportInput);

    let args_input: Punctuated<Ident, Comma> = args
        .iter()
        .map(|arg| -> Ident {
            let pat = &arg.pat;
            parse_quote!(#pat)
        })
        .collect();
    let args_fields = fields
        .0
        .iter()
        .map(|(ident, op)| -> FieldValue {
            match op {
                Some((_, val)) => parse_quote!(#ident: #val),
                None => parse_quote!(#ident),
            }
        })
        .collect::<Punctuated<FieldValue, Comma>>();

    let mut struct_fields = args.clone();
    for (ident, op) in fields.0 {
        match op {
            Some((ty, _)) => struct_fields.push(parse_quote!(#ident: #ty)),
            None => (),
        }
    }

    let output = quote! {
        pub type #name = ::vase::ipc::GenericTransport<#adapter>;

        impl #name {
            pub fn new_arc(#args) -> ::std::sync::Arc<Self> {
                let mut engine = ::vase::ipc::transport::driver::engine::TransportEngine::<#adapter>::new();
                Arc::new(Self {
                    adapter: Arc::new(#adapter {
                        #args_fields
                    }),
                    engine: Arc::new(engine),
                })
            }
        }
        pub struct #adapter {
            #struct_fields
        }
    };
    output.into()
}
