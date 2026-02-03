use aster_common::rust::lit::FromBase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, format_ident, quote};
use syn::{
    Expr, FnArg, Ident, ItemFn, ItemImpl, Lit, Path, Type, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Async, Comma},
};

pub fn handle_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    // 1. 解析路径列表
    // 既然我们强制使用命名空间，用户传入的 path (如 LocalSocketDevice::Test)
    // 本身就是合法的 Type Path，不需要做任何 _ 前缀修改。
    let device_list: Vec<Path> = attr
        .to_string()
        .split(',')
        .map(|st| {
            let st = st.trim();
            if st.is_empty() {
                return None;
            }
            // 直接解析，不做任何修改
            Some(syn::parse_str::<Path>(st).expect("Invalid path in handle attribute"))
        })
        .filter_map(|x| x)
        .collect();

    let mut fn_impl = parse_macro_input!(input as ItemFn);
    let is_async = fn_impl.sig.asyncness.is_some();
    let fn_ident = &fn_impl.sig.ident;
    let fn_ident_str = fn_ident.to_string();

    // 生成内部实现函数名和 Handler 函数名
    // 为了防止不同 path 下同名函数冲突，建议在 Handler 名字里包含一些额外信息，
    // 但鉴于我们使用 inventory 注册时 module_path 已经区分了上下文，这里可以用简单命名。
    // 如果你在同一个文件里对同一个 Device 实现了两个同名函数(不可能)，Rust 编译器本身就会报错。
    let fn_impl_ident = format_ident!("_{}_impl", fn_ident_str);
    let fn_handler_ident = format_ident!("_{}_handler", fn_ident_str);

    // 2. 参数解析 (保持不变)
    let fn_args = fn_impl.sig.inputs.clone();
    let fn_args_tuple: Vec<(Ident, Type)> = fn_args
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => {
                let ty = *pat.ty.clone();
                let pat_ident = if let syn::Pat::Ident(p) = &*pat.pat {
                    p.ident.clone()
                } else {
                    return None;
                };
                Some((pat_ident, ty))
            }
            FnArg::Receiver(_) => None,
        })
        .collect();

    let mut fn_args_input = Punctuated::<Expr, Comma>::new();
    let mut fn_args_names = Punctuated::<Ident, Comma>::new();
    let mut fn_args_types = Punctuated::<Type, Comma>::new();

    fn_args_tuple
        .iter()
        .enumerate()
        .for_each(|(idx, (ident, ty))| {
            let idx_lit = Lit::from_int(idx as i64);
            let expr: Expr = parse_quote!(value.#idx_lit);
            fn_args_input.push(expr);
            fn_args_names.push(ident.clone());
            fn_args_types.push(ty.clone());
        });

    let ty_tuple: Type = if fn_args_tuple.is_empty() {
        parse_quote!(())
    } else {
        parse_quote!((#fn_args_types,))
    };

    let args_vars_expr: Expr = if fn_args_names.is_empty() {
        parse_quote!(())
    } else {
        parse_quote!((#fn_args_names,))
    };

    // 3. 生成 Client Stub (直接 impl 给 path)
    let mut fn_entry: ItemFn = parse_quote! {
        pub async fn #fn_ident() -> anyhow::Result<()> { // 返回值根据原函数调整，这里简化为 Result<()>
             // TODO: 这里如果原函数有返回值，需要修改上面的 -> Result<T> 并处理 Result<Val>
             // 目前你的代码里好像统一返回 Result<serde_json::Value> 然后转 Struct?
             // 我们假设 Client Stub 返回值类型会做自动推导或在这里补全。

            use ::vase::ipc::device::traits::DeviceRef;

            // 1. 序列化参数
            let value = ::serde_json::to_value(&#args_vars_expr)?;

            // 2. 获取实例
            // 关键点：我们相信 device! 宏生成的 Namespace Struct 已经实现了 current() 方法
            // 并且 current() 返回的是父级 Device 的 Arc<Mutex<State>>
            let device_arc = Self::client_instance().await?;
            let device = device_arc.lock().await;

            // 3. RPC 调用
            // 保持 module_path! 与 Handler 注册一致
            let rpc_method_name = format!("{}::{}", module_path!(), #fn_ident_str);

            let res = device.call(rpc_method_name.clone(), value).await?;

            #[cfg(test)]
            println!("[Vase RPC] Call: {} Result: {:?}", rpc_method_name, res);

            ::std::result::Result::Ok(::serde_json::from_value(res)?)
        }
    };

    // 复制签名（主要为了复制参数列表，但移除了 self）
    // 注意：我们需要重新修正返回值类型，上面的 parse_quote 里的返回值是写死的
    fn_entry.sig.inputs = fn_impl.sig.inputs.clone();
    fn_entry.sig.output = fn_impl.sig.output.clone();
    fn_entry.sig.asyncness = Some(Async {
        span: Span::call_site(),
    });

    // 移除 self 参数 (Namespace 模式下通常是静态函数调用 LocalSocketDevice::Test::func())
    // 如果你想支持 instance.func()，需要 device! 宏生成的 struct 能被实例化。
    // 这里假设是静态调用，过滤掉 self
    fn_entry.sig.inputs = fn_entry
        .sig
        .inputs
        .into_iter()
        .filter(|arg| !matches!(arg, FnArg::Receiver(_)))
        .collect();

    // 生成 impl 块
    // 直接 impl LocalSocketDevice::Test
    let impl_list: Vec<ItemImpl> = device_list
        .iter()
        .map(|target_struct_path| {
            parse_quote! {
                impl #target_struct_path {
                    #fn_entry
                }
            }
        })
        .collect();

    // 4. 生成 Server Handler (保持逻辑不变)
    let call_expr: Expr = if is_async {
        parse_quote!(#fn_impl_ident(#fn_args_input).await)
    } else {
        parse_quote!(#fn_impl_ident(#fn_args_input))
    };

    let fn_handler: ItemFn = parse_quote! {
        fn #fn_handler_ident(value: ::serde_json::Value) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<Output = ::anyhow::Result<::serde_json::Value>> + ::core::marker::Send
            >
        > {
            ::std::boxed::Box::pin(async move {
                let value: #ty_tuple = ::serde_json::from_value(value)?;
                // 这里调用的是重命名后的本地实现
                let result = #call_expr?;
                ::std::result::Result::Ok(::serde_json::to_value(result)?)
            })
        }
    };

    fn_impl.sig.ident = fn_impl_ident.clone();

    // 5. 组合输出
    let mut expand = quote! {
        ::inventory::submit! {
            ::vase::ipc::HandlerRegistration {
                module: module_path!(),
                name: #fn_ident_str,
                func: #fn_handler_ident,
            }
        }
        #fn_impl
        #fn_handler
    };
    expand.extend(impl_list.iter().map(|imp| imp.to_token_stream()));
    expand.into()
}
