mod handler;

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use quote::ToTokens;
use syn::{Attribute, Item};

use crate::{scan::file::FileParser, utils::Idents};

pub(crate) struct NormalParser;

pub(crate) enum Attr {
    Handler(String),
    Others,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Context {
    handler: HashMap<String, String>,
    dependencies: HashMap<String, String>,
}

static CONTEXT: LazyLock<Mutex<Context>> = LazyLock::new(|| {
    Mutex::new(Context {
        handler: HashMap::new(),
        dependencies: HashMap::new(),
    })
});

impl FileParser for NormalParser {
    fn parse(&self, item: Item) {
        match item {
            Item::Fn(item_fn) => match match_attr(&item_fn.attrs) {
                Attr::Handler(prop) => {
                    if let Ok(ctx) = CONTEXT.lock() {
                        handler::parse(item_fn, &prop, ctx);
                    }
                }
                Attr::Others => (),
            },
            _ => (),
        }
    }
}

fn match_attr(attrs: &Vec<Attribute>) -> Attr {
    let mut res: Attr = Attr::Others;
    for attr in attrs {
        let path = attr.path();
        if path.is_ident("handler") || path.is_idents(&["vase", "handler"]) {
            res = Attr::Handler(attr.meta.to_token_stream().to_string());
            break;
        }
    }
    res
}
