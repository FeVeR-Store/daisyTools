use aster_common::rust::lit::FromBase;
use proc_macro::TokenStream;
use quote::{format_ident, quote}; // 移除了 ToTokens，因为这里没用到
use syn::{
    Expr, FnArg, Ident, ItemFn, Lit, Path, Type, parse_macro_input, parse_quote,
    punctuated::Punctuated,
    token::{Async, Comma},
};

pub fn expose_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    // 1. 解析路径列表 (e.g. LocalSocketDevice::Test)
    let device_paths: Vec<Path> = attr
        .to_string()
        .split(',')
        .map(|st| {
            let st = st.trim();
            if st.is_empty() {
                return None;
            }
            Some(syn::parse_str::<Path>(st).expect("Invalid path in expose attribute"))
        })
        .filter_map(|x| x)
        .collect();

    let mut fn_impl = parse_macro_input!(input as ItemFn);
    let is_async = fn_impl.sig.asyncness.is_some();
    let fn_ident = &fn_impl.sig.ident;
    let fn_ident_str = fn_ident.to_string();

    // 生成内部实现函数名和 Handler 函数名
    let fn_impl_ident = format_ident!("_{}_impl", fn_ident_str);
    let fn_handler_ident = format_ident!("_{}_expose_handler", fn_ident_str);

    // 2. 参数解析 (与 handle 保持一致，支持 Tuple 解包)
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
    let mut fn_args_types = Punctuated::<Type, Comma>::new();

    fn_args_tuple.iter().enumerate().for_each(|(idx, (_, ty))| {
        let idx_lit = Lit::from_int(idx as i64);
        let expr: Expr = parse_quote!(value.#idx_lit);
        fn_args_input.push(expr);
        fn_args_types.push(ty.clone());
    });

    let ty_tuple: Type = if fn_args_tuple.is_empty() {
        parse_quote!(())
    } else {
        parse_quote!((#fn_args_types,))
    };

    // 3. 生成 Wrapper Handler (Client 收到 Server 请求时执行)
    // 逻辑：Value -> Tuple -> Original Fn -> Result -> Value
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
                // 反序列化参数
                let value: #ty_tuple = ::serde_json::from_value(value)?;
                // 调用用户逻辑
                let result = #call_expr;
                // 这里的 result 应该是 Result<T>，我们需要处理它
                // 假设用户函数签名是 -> anyhow::Result<T>
                let result = result?;
                ::std::result::Result::Ok(::serde_json::to_value(result)?)
            })
        }
    };

    fn_impl.sig.ident = fn_impl_ident.clone();

    // 4. 生成注册块
    // 注意：expose 需要为每一个路径生成注册信息
    // 注册名必须符合 Device::Mod::call 的拼接规则
    let register_blocks = device_paths.iter().map(|path| {
        let segments: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();

        // 构造 RPC 方法名
        // 如果路径是 LocalSocketDevice::Test，则注册名为 "Test::func_name"
        // 如果路径是 LocalSocketDevice (无命名空间)，则注册名为 "func_name"
        let rpc_method_name = if segments.len() > 1 {
            // 取出除了 DeviceName 之外的部分作为前缀
            let prefix = segments[1..].join("::");
            format!("{}::{}", prefix, fn_ident_str)
        } else {
            fn_ident_str.clone()
        };

        quote! {
            ::inventory::submit! {
                ::vase::ipc::ExposedHandlerRegistration {
                    name: #rpc_method_name,
                    func: #fn_handler_ident,
                }
            }
        }
    });

    // 5. 组合输出
    let expand = quote! {
        #(#register_blocks)*
        #fn_impl
        #fn_handler
    };

    expand.into()
}
