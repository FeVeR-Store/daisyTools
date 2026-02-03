mod entry;

use entry::entry_impl;
use proc_macro::TokenStream;

use crate::entry::test_impl;

#[proc_macro_attribute]
pub fn entry(input: TokenStream, attr: TokenStream) -> TokenStream {
    entry_impl(input, attr)
}

#[proc_macro_attribute]
pub fn test(input: TokenStream, attr: TokenStream) -> TokenStream {
    test_impl(input, attr)
}
