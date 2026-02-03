use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, PatType, ReturnType, Type, parse_macro_input, parse_quote, token::Async};

pub fn listen_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    // 1. 解析要监听的事件类型 (e.g. LocalSocketDevice::Message)
    // 这是一个 Type Path，因为我们之前已经实现了 Mod + Struct 的结构
    let event_type_path = parse_macro_input!(attr as syn::Path);
    let event_name = event_type_path.segments.last().unwrap();

    let mut func = parse_macro_input!(input as ItemFn);
    let func_ident = &func.sig.ident;
    let func_ident_str = func_ident.to_string();
    let is_async = func.sig.asyncness.is_some();

    // =======================================================================
    // 2. 核心逻辑：类型替换 (Infer -> Type)
    // =======================================================================
    // 遍历所有参数，找到类型为 "_" (Infer) 的，替换为 event_type_path
    for input in &mut func.sig.inputs {
        if let FnArg::Typed(PatType { ty, .. }) = input {
            if matches!(**ty, Type::Infer(_)) {
                *ty = Box::new(Type::Path(syn::TypePath {
                    qself: None,
                    path: event_type_path.clone(),
                }));
            }
        }
    }

    // =======================================================================
    // 3. 核心逻辑：返回值自动补全
    // =======================================================================
    // 策略：如果用户没有写返回值 (Default)，则自动补充 -> anyhow::Result<()>
    // 如果用户写了返回值，则保持原样 (交给编译器检查是否符合 Result 约束)
    if let ReturnType::Default = func.sig.output {
        func.sig.output = parse_quote!(-> ::anyhow::Result<()>);
    }

    // 强制加上 async，因为 wrapper 调用它是 await 的
    // 如果用户写的不是 async，这里强制加会让代码统一，但要注意逻辑是否允许
    // 通常 listen 回调涉及 IO，建议强制 async
    if func.sig.asyncness.is_none() {
        func.sig.asyncness = Some(Async {
            span: Span::call_site(),
        });
    }

    // =======================================================================
    // 4. 生成 Wrapper Handler
    // =======================================================================
    // Wrapper 负责：接收通用 Value -> 反序列化为具体 Struct -> 调用用户函数
    let handler_ident = format_ident!("_{}_listen_handler", func_ident_str);

    // 根据用户函数是否 async 生成调用代码
    // (上面我们强制了 user func 为 async，所以这里直接 await)
    let call_expr = quote!(#func_ident(event).await);

    let handler_fn = quote! {
        // Handler 签名必须匹配 ListenerRegistration 中的 func 定义
        // 假设签名为: fn(Value) -> Pin<Box<dyn Future<Output = Result<()>>>>
        fn #handler_ident(
            value: ::serde_json::Value
        ) -> ::core::pin::Pin<
            ::std::boxed::Box<
                dyn ::core::future::Future<Output = ::anyhow::Result<()>> + ::core::marker::Send
            >
        > {
            ::std::boxed::Box::pin(async move {
                // 1. 反序列化事件数据
                // 直接使用宏传入的 Path 作为目标类型
                let event: #event_type_path = ::serde_json::from_value(value)?;

                // 2. 调用用户逻辑
                #call_expr
            })
        }
    };

    // =======================================================================
    // 5. 生成 Inventory 注册代码
    // =======================================================================
    let inventory_registry = quote! {
        ::inventory::submit! {
            ::vase::ipc::ListenerRegistration {
                event: stringify!(#event_name),
                func: #handler_ident,
            }
        }
    };

    // 组合最终代码
    quote! {
        #inventory_registry
        #func
        #handler_fn
    }
    .into()
}
