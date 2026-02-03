use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    Ident, Result, Token,
};

/// 单个函数入口，可能带 `async`
pub struct FuncEntry {
    pub is_async: bool,
    pub name: Ident,
}

impl Parse for FuncEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![async]) {
            input.parse::<Token![async]>()?;
            let name: Ident = input.parse()?;
            Ok(FuncEntry {
                is_async: true,
                name,
            })
        } else {
            let name: Ident = input.parse()?;
            Ok(FuncEntry {
                is_async: false,
                name,
            })
        }
    }
}

pub struct LoadActionInput {
    pub name: Ident,
    pub funcs: Vec<FuncEntry>,
}

impl Parse for LoadActionInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; // 第一个 ident

        input.parse::<Token![,]>()?; // 逗号

        let content;
        bracketed!(content in input); // [ ... ]
        let funcs: Punctuated<FuncEntry, Comma> =
            content.parse_terminated(FuncEntry::parse, Comma)?;

        Ok(Self {
            name,
            funcs: funcs.into_iter().collect(),
        })
    }
}
