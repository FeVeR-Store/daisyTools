mod attr_entry;
// mod depencence;
mod no_attr;

use std::str::FromStr;

use proc_macro2::TokenStream;
use syn::{File, Item, parse_quote};

pub fn scan_file(content: Vec<u8>) {
    let content = str::from_utf8(&content).unwrap();
    let token = TokenStream::from_str(content).unwrap();
    let item_file: File = parse_quote!(#token);
    let parser = match_parser(item_file.shebang.clone());
    parser.parse_file_items(item_file.items);
}

pub(crate) fn match_parser(attr: Option<String>) -> Box<dyn FileParser> {
    let attr = attr.unwrap_or_default();
    match attr.as_str() {
        "vase::entry" => Box::new(attr_entry::VaseEntryParser),
        _ => Box::new(no_attr::NormalParser),
    }
}

pub(crate) trait FileParser {
    fn parse(&self, item: Item);
    fn parse_file_items(&self, items: Vec<Item>) {
        for item in items {
            self.parse(item);
        }
    }
}
