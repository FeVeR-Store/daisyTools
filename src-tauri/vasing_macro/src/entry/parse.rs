use proc_macro::TokenStream;
use syn::{
    Block, Ident, Item, ItemFn, ItemMacro, ItemMod, Visibility, parse_macro_input, parse_quote,
};

pub struct EnteyContext {
    pub mod_vis: Visibility,
    pub mod_name: Ident,
    pub cfg: Option<Ident>,

    pub spawns: Vec<(Ident, Block)>,
    pub _forks: Vec<(Ident, Block)>,

    pub signals: Vec<Ident>,
    pub other_items: Vec<Item>,
}

pub fn create_entry(
    attr: TokenStream,
    input: TokenStream,
    generator: fn(&EnteyContext) -> proc_macro2::TokenStream,
) -> TokenStream {
    let mod_item = parse_macro_input!(input as ItemMod);
    let (_, content) = if let Some(content) = mod_item.content {
        content
    } else {
        panic!("No content found in the module");
    };

    let cfg = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr as Ident))
    };
    let mut ctx = EnteyContext {
        mod_vis: mod_item.vis,
        mod_name: mod_item.ident,
        cfg,
        spawns: Vec::new(),
        _forks: Vec::new(),
        signals: Vec::new(),
        other_items: vec![
            parse_quote!(
                #[allow(unused_macros)]
                macro_rules! resolve {
                    ($s:ident) => {
                        $s.resolve()
                    };
                }
            ),
            parse_quote!(
                #[allow(unused_macros)]
                macro_rules! pending {
                    ($s:ident) => {
                        $s.pending().await
                    };
                }
            ),
        ],
    };

    for item in content {
        match item {
            Item::Fn(fn_item) => handle_fn(fn_item, &mut ctx),
            Item::Macro(macro_item) => handle_macro(macro_item, &mut ctx),
            _ => ctx.other_items.push(item),
        }
    }
    let output = generator(&ctx);
    output.into()
}

fn handle_fn(fn_item: ItemFn, ctx: &mut EnteyContext) {
    let mut is_fork = false;
    let mut is_spawn = false;
    for attr in &fn_item.attrs {
        if attr.meta.path().is_ident("fork") {
            is_fork = true;
            continue;
        }
        if attr.meta.path().is_ident("spawn") {
            is_spawn = true;
            continue;
        }
    }
    if is_spawn {
        let name = fn_item.sig.ident;
        let body: Block = {
            let boxed = fn_item.block;
            parse_quote!(#boxed)
        };

        ctx.spawns.push((name, body));
        return;
    } else if is_fork {
        todo!("fork mode is not supported")
    }
    ctx.other_items.push(Item::Fn(fn_item));
}

fn handle_macro(macro_item: ItemMacro, ctx: &mut EnteyContext) {
    let mac_path = &macro_item.mac.path;
    if mac_path.is_ident("signal") {
        let tokens = macro_item.mac.tokens;
        let ident: Ident = parse_quote!(#tokens);
        ctx.signals.push(ident);
        return;
    }
    ctx.other_items.push(Item::Macro(macro_item));
}
