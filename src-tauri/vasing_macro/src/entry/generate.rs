use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Item, Visibility};

use crate::entry::parse::EnteyContext;

pub fn generate(
    ctx: &EnteyContext,
    process: fn(ProcessArgs) -> proc_macro2::TokenStream,
) -> TokenStream {
    let EnteyContext {
        spawns,
        signals,
        other_items,
        cfg,
        mod_vis,
        mod_name,
        ..
    } = ctx;

    let signal_inits = signals
        .iter()
        .map(|name| {
            quote! {
                #[allow(non_upper_case_globals)]
                static #name: ::std::sync::LazyLock<::vasing::Signal> = ::std::sync::LazyLock::new(|| ::vasing::Signal::new());
            }
        })
        .collect::<Vec<_>>();

    let spawn_calls = spawns
        .iter()
        .map(|(name, body)| {
            let name_str = name.to_string();

            // let mut clone_list = Vec::new();
            // let mut capture_list = Vec::new();
            // signals.iter().for_each(|s| {
            //     let task_signal_name = format!("__signal_{}_task_{}", s, index);
            //     let task_signal = Ident::new(&task_signal_name, Span::call_site());
            //     clone_list.push(quote! {
            //         let #task_signal = #s.clone();
            //     });
            //     capture_list.push(quote! {
            //         #[allow(unused_variables)]
            //         let #s = #task_signal;
            //     })
            // });
            quote! {
                stage.spawn(#name_str, move || async move {
                    #body
                });
            }
        })
        .collect::<Vec<_>>();
    let cfg_attr = if let Some(cfg) = cfg {
        quote! { #[cfg(#cfg)] }
    } else {
        quote! {}
    };
    process((
        cfg_attr,
        other_items,
        signal_inits,
        spawn_calls,
        mod_vis,
        mod_name,
    ))
}

type ProcessArgs<'a> = (
    TokenStream,
    &'a Vec<Item>,
    Vec<TokenStream>,
    Vec<TokenStream>,
    &'a Visibility,
    &'a Ident,
);

pub fn entry(ctx: &EnteyContext) -> TokenStream {
    fn process((cfg_attr, other_items, signal_inits, spawn_calls, ..): ProcessArgs) -> TokenStream {
        quote! {
            #cfg_attr
            #[::tokio::main]
            async fn vasing_entry() {
                #(#other_items)*
                let mut stage = ::vasing::Stage::new();

                #(#signal_inits)*
                #(#spawn_calls)*

                stage.run().await;
            }
        }
    }
    generate(ctx, process)
}

pub fn test(ctx: &EnteyContext) -> TokenStream {
    fn process(
        (cfg_attr, other_items, signal_inits, spawn_calls, mod_vis, mod_name): ProcessArgs,
    ) -> TokenStream {
        quote! {
            #cfg_attr
            #[cfg(test)]
            #mod_vis mod #mod_name {
                use super::*;
                // 确保 vasing 可用，假设用户在该文件引入了 vasing crate
                // 或者使用绝对路径 crate::vasing (取决于你的 crate 结构)
                // 这里假设用户已经在 Cargo.toml 引入了 vasing

                #(#signal_inits)*
                // 放入保留的 item (use, structs, helpers)
                #(#other_items)*

                #[tokio::test]
                async fn vasing_runner() {
                    let mut stage = ::vasing::Stage::new();

                    // 初始化信号

                    // 注册任务
                    #(#spawn_calls)*

                    // 运行
                    stage.run().await;
                }
            }
        }
    }
    generate(ctx, process)
}
