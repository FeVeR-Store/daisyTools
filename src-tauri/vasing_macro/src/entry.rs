use proc_macro::TokenStream;

use crate::entry::parse::create_entry;
mod generate;
mod parse;

pub fn entry_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    create_entry(attr, input, generate::entry)
}

pub fn test_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    create_entry(attr, input, generate::test)
}
