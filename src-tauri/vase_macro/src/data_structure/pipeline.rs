use std::{panic, str::FromStr};

use aster_common::utils::IntoIdent;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Block, Expr, ExprArray, Ident, Stmt, Token, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
};

// 对应: input -> [m1, m2] as mid { ... }
struct PipelineInput {
    input_ident: Ident,
    mutability: Option<Token![mut]>,
    _arrow: Token![->],
    middleware_list: Punctuated<Expr, Token![,]>, // 逗号分隔的表达式列表
    _as_token: Token![as],
    alias_ident: Ident,
    process_block: Block,
}

impl Parse for PipelineInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 1. 解析 input 变量名
        let input_ident: Ident = input.parse()?;

        // 2. 解析 ->
        let _arrow: Token![->] = input.parse()?;

        // 3. 解析 [m1, m2, ...]
        let content;
        let _brackets = bracketed!(content in input);
        let middleware_list = content.parse_terminated(Expr::parse, Token![,])?;

        // 4. 解析 as
        let _as_token: Token![as] = input.parse()?;

        let mutability: Option<Token![mut]> = if input.peek(Token![mut]) {
            Some(input.parse()?)
        } else {
            None
        };

        // 5. 解析别名
        let alias_ident: Ident = input.parse()?;

        // 6. 解析处理逻辑块 { ... }
        let process_block: Block = input.parse()?;

        Ok(PipelineInput {
            input_ident,
            _arrow,
            mutability,
            middleware_list,
            _as_token,
            alias_ident,
            process_block,
        })
    }
}

pub fn pipeline_impl(input: TokenStream) -> TokenStream {
    // 自动解析，如果格式错误，syn 会生成带有正确 Span 的编译错误
    let PipelineInput {
        input_ident,
        middleware_list,
        alias_ident,
        process_block,
        mutability,
        ..
    } = parse_macro_input!(input as PipelineInput);

    // 生成处理链
    // 逻辑：利用 Rust 的变量遮蔽 (shadowing)。
    // 每次迭代生成一个新的作用域，将结果重新绑定给 input_ident。
    let expansions = middleware_list.iter().map(|middleware| {
        let mutability = if mutability.is_some() {
            quote! { mut }
        } else {
            quote! {}
        };
        quote! {
            let #input_ident = {
                // 将中间件表达式绑定到别名 (例如: let mid = middleware1;)
                let #mutability #alias_ident = #middleware;
                // 直接插入用户定义的块，块的返回值将赋值给外层的 input_ident
                #process_block
            };
        }
    });

    // 组合最终代码
    // 将所有步骤包裹在一个块中，最后返回 input_ident
    let expanded = quote! {
        {
            #(#expansions)*
            #input_ident
        }
    };

    expanded.into()
}
