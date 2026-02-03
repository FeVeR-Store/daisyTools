use quote::quote;
use syn;
use proc_macro::TokenStream;
mod action;
mod hot_lib_reloader;
mod loader;
mod utils;

// 导出主要的宏
use action::define_action_impl;

use crate::{
    action::{define_options_proc, result_branch_impl, to_value_derive_impl},
    hot_lib_reloader::hot_module,
};
// 定义属性宏 - 仅作为标记使用，实际解析由 Darling 完成
#[proc_macro_attribute]
pub fn name(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn description(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

// 导出 define_action 宏
#[proc_macro_attribute]
pub fn action(attr: TokenStream, input: TokenStream) -> TokenStream {
    define_action_impl(attr, input)
}

#[proc_macro_attribute]
pub fn options(_attr: TokenStream, input: TokenStream) -> TokenStream {
    define_options_proc(input)
}

#[proc_macro_attribute]
pub fn label(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn entry(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(ToValue)]
pub fn to_value_derive(input: TokenStream) -> TokenStream {
    to_value_derive_impl(input)
}

#[proc_macro_attribute]
pub fn result(_attr: TokenStream, input: TokenStream) -> TokenStream {
    result_branch_impl(input).into()
}

#[proc_macro_attribute]
pub fn left(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
#[proc_macro_attribute]
pub fn right(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
#[proc_macro_attribute]
pub fn bottom(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
#[proc_macro_attribute]
pub fn top(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn source(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
#[proc_macro_attribute]
pub fn branch(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
/// 将该分支转化为错误分支，可在任意Result后调用该分支名，该Result为Err时会自动归入该指定分支\
/// ```rust
/// #[result]
/// enum Result {
///     #[left]
///     Success(),
///     #[error]
///     Error {
///         #[bottom]
///         io: _ // 类型将被忽略，因此可填入 _ (infer)
///     }
/// }
/// #[action(...)]
/// fn some_action() -> Result {
///     let file = read("file").io()?; // 错误时会进入Error::io分支
///     Result::Success()
/// }
/// ```
/// 注意，如果这样做，分支的类型将被忽略，并且不可从Result中直接调用
/// ```rust
/// // 此用法将不再可用
/// Result::Error.io()
/// ```
pub fn error(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
#[proc_macro_attribute]
pub fn raw(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn load_action(input: TokenStream) -> TokenStream {
    loader::load_action_impl(input)
}

/// This macro is the top-level interface for making a dynamic Rust library
/// hot-reloadable. The attribute macro will insert code into the module it
/// accompanies that will do several things:
///
/// 1. In the context of that module a global
///    [`hot_lib_reloader::LibReloader`](https://docs.rs/hot-lib-reloader/latest/hot_lib_reloader/struct.LibReloader.html)
///    instance is maintained that loads the library specified by the `dylib`
///    argument and provides access to its symbols.
///
/// 2. A thread is started that drives the `LibReloader`: It waits for library
///    file changes and then
///    [updates](https://docs.rs/hot-lib-reloader/latest/hot_lib_reloader/struct.LibReloader.html#method.update)
///    the library.
///
/// 3. Allows access to a
///    [`hot_lib_reloader::LibReloadNotifier`](https://docs.rs/hot-lib-reloader/latest/hot_lib_reloader/struct.LibReloadNotifier.html)
///    that can be used to get events about library changes. See the
///    `#[lib_change_subscription]` attribute below.
///
/// In addition, the module can contain normal items. You can define functions,
/// types etc normally and you can import and export from other modules. In
/// particular re-exporting all items from the target library can make it easy
/// to create a 1:1 replacement with static modules from that library.
///
/// A few pseudo-macros can appear in the modules context:
///
/// ```ignore
/// // The `dylib` attribute should be the name of the library to hot-reload,
/// // typically the crate name.
/// #[hot_module(dylib = "lib")]
/// mod foo {
///
///   // reads `#[unsafe(no_mangle)]` public functions from `file.rs` and generates
///   // forwarding functions in the context of this module that have the exact
///   // same signatures. Those generated functions will automatically use the
///   // newest version of the library.
///   hot_functions_from_file!("path/to/file.rs");
///
///   // As an alternative to `hot_functions_from_file!` you can manually
///   // declare functions that the library should export and for which hot-reload
///   // implementations should be generated. It is more tedious but plays nicer
///   // with tools like rust-analalyzer and auto completion.
///   #[hot_function]
///   pub fn do_stuff(arg: &str) -> u32 { /*generated*/ }
///
///   // Same as `hot_function` but as a block, multiple declarations are allowed.
///   #[hot_functions]
///   extern "Rust" {
///       pub fn do_stuff(arg: &str) -> u32;
///   }
///
///   // To get access to a `LibReloadObserver` you can create an empty function
///   // with a `#[lib_change_subscription]` attribute.
///    #[lib_change_subscription]
///    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
/// }
/// ```
///
/// In case you get errors when using the macro or are generally curious, run
/// `cargo expand` to see the generated code.
#[proc_macro_attribute]
pub fn hot_module(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as hot_module::HotModuleAttribute);
    let mut module = syn::parse_macro_input!(item as hot_module::HotModule);
    module.hot_module_args = Some(args);

    (quote! { #module }).into()
}
