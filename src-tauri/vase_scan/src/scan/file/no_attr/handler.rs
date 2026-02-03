use std::sync::MutexGuard;

use syn::ItemFn;

use crate::scan::file::no_attr::Context;

pub fn parse(item_fn: ItemFn, _prop: &str, _ctx: MutexGuard<'_, Context>) {
    item_fn.sig.inputs;
}
