use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, ExprStruct, Fields, FnArg, Ident, Item, ItemFn, ItemImpl, ItemStruct, Local, Member,
    Meta, Pat, PatType, Stmt, Type, parse_macro_input, parse_quote, punctuated::Punctuated,
    token::Comma,
};

struct Context<'a> {
    struct_impl: &'a mut ItemImpl,
    item_struct: &'a mut ItemStruct,
    struct_init: &'a mut ExprStruct,
    device_ident: &'a Ident,
    state_ident: &'a Ident,
    device_ref_instance_ident: &'a Ident,
    device_instance_ident: &'a Ident,
    event_list: &'a mut Vec<(Ident, Item)>,
    meta_list: &'a mut Vec<Expr>,
    flag_list: &'a mut Vec<Expr>,
    mod_items: &'a mut Vec<Item>,
    // [New] 传递 cfg 条件给子处理函数
    server_cfg: &'a Option<Meta>,
    client_cfg: &'a Option<Meta>,
}

pub fn device_impl(input: TokenStream) -> TokenStream {
    let device = parse_macro_input!(input as ExprStruct);
    let ExprStruct {
        path, fields, rest, ..
    } = device;

    let device_ident = path.require_ident().unwrap();
    let state_ident = format_ident!("_{}", device_ident);
    let device_instance_ident =
        format_ident!("{}_INSTANCE", device_ident.to_string().to_uppercase());
    let device_ref_instance_ident =
        format_ident!("{}_REF_INSTANCE", device_ident.to_string().to_uppercase());

    // 初始化基础结构
    let mut item_struct: ItemStruct = parse_quote! { pub struct #state_ident {} };
    let mut struct_impl: ItemImpl = parse_quote! { impl #state_ident { } };
    let mut struct_init: ExprStruct = parse_quote!(Self {});

    let mut meta_list: Vec<Expr> = Vec::new();
    let mut flag_list: Vec<Expr> = Vec::new();
    let mut event_list: Vec<(Ident, Item)> = Vec::new();
    let mut mod_items: Vec<Item> = Vec::new();

    let mut package_name: Option<String> = None;

    let mut server_cfg: Option<Meta> = None;
    let mut client_cfg: Option<Meta> = None;

    let mut macro_init: ItemImpl = parse_quote! {
        impl ::vase::MacroInit for _transport { }
    };
    // 1. 字段解析
    fields.iter().for_each(|field| {
        let expr = &field.expr;
        let Member::Named(ident) = &field.member else {
            panic!("Expected named field");
        };
        let ident_str = ident.to_string();

        if ident_str == "transport" {
            let (trans, args) = match expr {
                Expr::Call(call) => (&call.func, call.args.clone()),
                _ => panic!("Expected call expression"),
            };
            item_struct =
                parse_quote! { pub struct #state_ident { transport: ::std::sync::Arc<#trans> } };
            macro_init.self_ty = parse_quote! { ::std::sync::Arc<#trans> };
            macro_init.items.push(parse_quote! {
                fn new() -> Self {
                    #trans::new_arc(#args)
                }
            });
            struct_init
                .fields
                .push(parse_quote! { transport: ::vase::ipc::transport::traits::Transport::new() });
        } else if ident_str == "package" {
            let pkg_str = quote!(#expr).to_string().replace(" ", "");
            package_name = Some(pkg_str);
        // [New] 解析 server 条件: server: "feature = \"server\""
        } else if ident_str == "server" {
            let s = quote!(#expr).to_string();
            server_cfg = Some(parse_quote!(feature = #s));
        // [New] 解析 client 条件
        } else if ident_str == "client" {
            let s = quote!(#expr).to_string();
            client_cfg = Some(parse_quote!(feature = #s));
        } else if ident_str == "compression" {
            meta_list.push(parse_quote!(Meta::CompressionAlgorithm(CompressionAlgorithm::#expr)));
            flag_list.push(parse_quote!(Flags::COMPRESSED));
        } else if ident_str == "encryption" {
            meta_list.push(parse_quote!(Meta::EncryptionAlgorithm(EncryptionAlgorithm::#expr)));
            flag_list.push(parse_quote!(Flags::ENCRYPTED));
        } else if ident_str == "keepAlive" {
            flag_list.push(parse_quote!(Flags::KEEP_ALIVE));
        } else if ident_str == "batch" {
            flag_list.push(parse_quote!(Flags::BATCH));
        }
    });

    // 辅助宏：生成 #[cfg(...)] 属性
    let mk_cfg = |cfg: &Option<Meta>| -> proc_macro2::TokenStream {
        if let Some(c) = cfg {
            quote!(#[cfg(#c)])
        } else {
            quote!()
        }
    };
    let s_cfg_attr = mk_cfg(&server_cfg);
    let c_cfg_attr = mk_cfg(&client_cfg);

    // 2. rest 块处理
    if let Some(rest) = &rest {
        let expr = parse_quote!(#rest);
        match expr {
            Expr::Block(mut block) => {
                for item in block.block.stmts.iter_mut() {
                    let ctx = Context {
                        struct_impl: &mut struct_impl,
                        device_ident,
                        state_ident: &state_ident,
                        device_instance_ident: &device_instance_ident,
                        device_ref_instance_ident: &device_ref_instance_ident,
                        item_struct: &mut item_struct,
                        struct_init: &mut struct_init,
                        meta_list: &mut meta_list,
                        flag_list: &mut flag_list,
                        event_list: &mut event_list,
                        mod_items: &mut mod_items,
                        // [New] 传入 cfg
                        server_cfg: &server_cfg,
                        client_cfg: &client_cfg,
                    };
                    match item {
                        Stmt::Item(Item::Struct(item_struct)) => handle_struct(item_struct, ctx),
                        Stmt::Item(Item::Fn(item_fn)) => handle_fn(item_fn, ctx),
                        Stmt::Local(item) => handle_let(item, ctx),
                        Stmt::Item(Item::Mod(item_mod)) => handle_mod(item_mod, ctx),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    struct_impl
        .items
        .push(parse_quote! { pub fn new() -> Self { #struct_init } });

    // 3. Mod 中的静态方法 (Setup & Call)

    // Server Setup (受 server_cfg 保护)
    mod_items.push(parse_quote! {
        #s_cfg_attr
        pub async fn setup() -> ::vase::ipc::Result<()> {
            use ::vase::ipc::device::traits::Device;
            // 注意：这里调用 server_instance()
            let device_arc = #state_ident::server_instance().await?;
            let mut device = device_arc.lock().await;
            Device::setup(&mut *device).await?;
            Ok(())
        }
    });

    // Client Setup (受 client_cfg 保护)
    mod_items.push(parse_quote! {
        #c_cfg_attr
        pub async fn setup_ref() -> ::vase::ipc::Result<()> {
            use ::vase::ipc::device::traits::{DeviceRef, DeviceConfig};
            // 注意：这里调用 client_instance()
            let device_arc = #state_ident::client_instance().await?;
            let mut device = device_arc.lock().await;
            device.setup().await?;
            if device.package().is_some() {
                device.ping().await
            } else {
                Ok(())
            }
        }
    });

    // // Server Call (受 server_cfg 保护)
    // mod_items.push(parse_quote! {
    //     #s_cfg_attr
    //     pub async fn call(target_package: impl AsRef<str>, method: impl AsRef<str>, payload: ::serde_json::Value) -> ::anyhow::Result<::serde_json::Value> {
    //         use ::vase::ipc::device::traits::Device;
    //         let device_arc = #state_ident::server_instance().await?;
    //         let device = device_arc.lock().await;
    //         let res = device.unicast(target_package.as_ref(), method.as_ref(), payload).await?;
    //         Ok(res)
    //     }
    // });

    // Metadata & Flags
    let default: Expr = parse_quote!(Metadata::default());
    let metadata_gen: Expr = meta_list
        .iter()
        .fold(default, |curr, next| parse_quote!(#curr + #next));
    let default: Expr = parse_quote!(Flags::empty());
    let flags_gen: Expr = flag_list
        .iter()
        .fold(default, |curr, next| parse_quote!(#curr | #next));
    let package_gen = if let Some(pkg) = package_name {
        quote!(Some(#pkg))
    } else {
        quote!(None)
    };

    // 4. 构建输出
    quote! {
        // Shared: State Struct & Impl & DeviceConfig
        #item_struct
        #struct_impl

        #macro_init
        impl ::vase::ipc::device::traits::DeviceConfig for #state_ident {
            fn meta(&self) -> ::vase::ipc::envelope::meta::Metadata {
                use ::vase::ipc::envelope::meta::*;
                #metadata_gen + self.package().map(|pkg| ::vase::ipc::envelope::meta::Meta::PackageName(pkg.to_string()))
            }
            fn flags(&self) -> ::vase::ipc::envelope::flags::Flags {
                use ::vase::ipc::envelope::flags::*;
                #flags_gen
            }
            fn package(&self) -> Option<&'static str> {
                #package_gen
            }
        }

        // 增加两个 Helper 方法到 State Impl，方便内部获取实例
        impl #state_ident {
            #s_cfg_attr
            pub async fn server_instance() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<Self>>> {
                 use ::vase::ipc::device::status::DeviceStatus;
                 let mut instance = #device_instance_ident.lock().await;
                 if let Some(ins) = &instance.instance { return Ok(ins.clone()); }
                 // Lazy init for server
                 let mut device = Self::new();
                 let device = ::std::sync::Arc::new(::tokio::sync::Mutex::new(device));
                 instance.instance = Some(device.clone());
                 instance.status = DeviceStatus::Ready;
                 Ok(device)
            }

            #c_cfg_attr
            pub async fn client_instance() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<Self>>> {
                 use ::vase::ipc::device::status::DeviceStatus;
                 let mut instance = #device_ref_instance_ident.lock().await;
                 if let Some(ins) = &instance.instance { return Ok(ins.clone()); }
                 // Lazy init for client
                 let mut device = Self::new();
                 let device = ::std::sync::Arc::new(::tokio::sync::Mutex::new(device));
                 instance.instance = Some(device.clone());
                 instance.status = DeviceStatus::Ready;
                 Ok(device)
            }
        }

        // --- Server Side Code ---
        #s_cfg_attr
        static #device_instance_ident: ::std::sync::LazyLock<
            ::tokio::sync::Mutex<::vase::ipc::device::status::DeviceInstance<#state_ident>>
        > = ::std::sync::LazyLock::new(|| {
            ::tokio::sync::Mutex::new(::vase::ipc::device::status::DeviceInstance::default())
        });

        #s_cfg_attr
        #[::vase::async_trait]
        impl ::vase::ipc::device::traits::Device for #state_ident {
            fn transport(&self) -> ::vase::ipc::Result<impl ::vase::ipc::transport::traits::TransportForServer> {
                ::vase::ipc::transport::traits::Transport::as_server(&self.transport)
            }
            async fn current() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<Self>>> {
                Self::server_instance().await
            }
            async fn setup(&mut self) -> ::vase::ipc::Result<()> {
                use ::vase::ipc::transport::traits::Transport;

                Transport::server_entry(&mut self.transport).await
            }
            async fn shutdown(self) -> ::vase::ipc::Result<()> {
                use ::vase::ipc::transport::traits::{TransportForServer, Transport};
                let transport = Transport::as_server(&self.transport)?;
                TransportForServer::shutdown(&transport).await
            }
        }

        // --- Client Side Code ---
        #c_cfg_attr
        static #device_ref_instance_ident: ::std::sync::LazyLock<
            ::tokio::sync::Mutex<::vase::ipc::device::status::DeviceInstance<#state_ident>>
        > = ::std::sync::LazyLock::new(|| {
            ::tokio::sync::Mutex::new(::vase::ipc::device::status::DeviceInstance::default())
        });

        #c_cfg_attr
        #[::vase::async_trait]
        impl ::vase::ipc::device::traits::DeviceRef for #state_ident {
            fn transport(&self) -> ::vase::ipc::Result<impl ::vase::ipc::transport::traits::TransportForClient> {
                ::vase::ipc::transport::traits::Transport::as_client(&self.transport)
            }
            async fn current() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<Self>>> {
                Self::client_instance().await
            }
            async fn setup(&mut self) -> ::vase::ipc::Result<()> {
                use ::vase::ipc::transport::traits::Transport;
                use ::vase::ipc::device::traits::DeviceRef;
                Transport::client_entry(&mut self.transport).await
            }
            async fn shutdown(self) -> ::vase::ipc::Result<()> {
                Ok(())
            }
        }

        #[allow(non_snake_case)]
        pub mod #device_ident {
            use super::*;
            #(#mod_items)*
        }
    }.into()
}

fn handle_struct(
    item_struct: &mut ItemStruct,
    Context {
        event_list,
        mod_items,
        state_ident,
        ..
    }: Context,
) {
    let is_event = item_struct
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("event"));
    item_struct
        .attrs
        .retain(|attr| !attr.meta.path().is_ident("event"));
    let ident = item_struct.ident.clone();
    let item = item_struct.clone();

    // 依然添加到 event_list 用于生成底层的 Enum (如果需要)
    if is_event {
        event_list.push((ident.clone(), Item::Struct(item.clone())));

        // [Change] 核心改动：Message 结构体直接放入 mod_items
        // 并且为 Message 生成 emit 方法

        // 1. 生成结构体 (带上 Serialize/Deserialize/Debug/Clone)
        // 注意：这里可能需要手动添加 derive，或者假设 fields 里的 attr 已经有了
        let mut struct_def = item.clone();
        // 强制添加 derive，确保可以用
        struct_def
            .attrs
            .push(parse_quote!(#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]));
        mod_items.push(Item::Struct(struct_def));

        // 2. 生成 emit 实现
        let emit_impl: Item = parse_quote! {
            impl #ident {
                pub async fn emit(self) -> anyhow::Result<()> {
                    use ::vase::ipc::device::traits::Device;
                    let device_arc = #state_ident::current().await?;
                    let device = device_arc.lock().await;
                    let event_name = format!(stringify!(#ident));
                    device.broadcast(event_name, self).await?;
                    Ok(())
                }
            }
        };
        mod_items.push(emit_impl);
    }
}

fn handle_fn(
    item_fn: &mut ItemFn,
    Context {
        struct_impl,
        state_ident, // 使用 State Ident
        device_instance_ident,
        mod_items, // 写入 proxy 到这里
        ..
    }: Context,
) {
    let mut require_instance = false;
    let mut mutability = false;
    let require_async = item_fn.sig.asyncness.is_some();

    if let Some(FnArg::Receiver(receiver)) = item_fn.sig.inputs.first() {
        require_instance = true;
        mutability = receiver.mutability.is_some();
    };

    let mut fn_state_impl = item_fn.clone(); // 放入 State 的实现
    let fn_ident = &item_fn.sig.ident;

    // 1. 处理 State 中的实现 (原汁原味的代码)
    // 你的原代码用了一个复杂的 trait 技巧来绕过 self，
    // 现在既然是在 State 的 impl 块里，我们可以直接放入，或者根据需要改名避免冲突
    // 这里为了简单，假设放入 impl LocalSocketDeviceState 的函数名为 `_impl_funcName`
    let impl_ident = format_ident!("_impl_{}", fn_ident);
    fn_state_impl.sig.ident = impl_ident.clone();

    // 放入 State 的 impl 块
    struct_impl.items.push(parse_quote! {
        #fn_state_impl
    });

    // 2. 处理 Mod 中的 Proxy (静态入口)
    // 只有当是实例方法时才需要生成 Proxy
    if require_instance {
        // 提取参数列表 (去除 self)
        let fn_entry_args = item_fn
            .sig
            .inputs
            .iter()
            .skip(1)
            .map(FnArg::clone)
            .collect::<Punctuated<_, Comma>>();

        // 提取调用参数名
        let fn_call_args: Punctuated<Ident, Comma> = fn_entry_args
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(arg) = arg {
                    let pat = &arg.pat;
                    let ident: Ident = parse_quote!(#pat);
                    Some(ident)
                } else {
                    None
                }
            })
            .collect();

        // 构造 Proxy 函数
        let proxy_fn: ItemFn = if require_async {
            parse_quote! {
               pub async fn #fn_ident(#fn_entry_args) -> anyhow::Result<()> {
                   // 获取单例
                   let guard = super::#device_instance_ident.lock().await;
                   if let Some(device_arc) = &guard.instance {
                       let mut device = device_arc.lock().await;
                       // 调用 State 的内部实现
                       device.#impl_ident(#fn_call_args).await
                   } else {
                       Err(anyhow::anyhow!("Device not initialized"))
                   }
               }
            }
        } else {
            // 同步版本
            parse_quote! {
               pub fn #fn_ident(#fn_entry_args) -> anyhow::Result<()> {
                    let guard = super::#device_instance_ident.blocking_lock();
                    if let Some(device_arc) = &guard.instance {
                       let mut device = device_arc.blocking_lock();
                       device.#impl_ident(#fn_call_args)
                   } else {
                       Err(anyhow::anyhow!("Device not initialized"))
                   }
               }
            }
        };

        mod_items.push(Item::Fn(proxy_fn));
    }
}

fn handle_let(
    item: &Local,
    Context {
        item_struct,
        struct_init,
        meta_list,
        flag_list,
        ..
    }: Context,
) {
    // 这里的 item_struct 已经被 device_impl 指向了 State 结构体
    // 所以逻辑保持不变，字段会被添加到 LocalSocketDeviceState 中

    let mut is_meta = false;
    let mut is_flag = false;
    for attr in item.attrs.iter() {
        if attr.path().is_ident("meta") {
            is_meta = true;
        } else if attr.path().is_ident("flag") {
            is_flag = true;
        }
    }
    match &item.pat {
        Pat::Type(PatType { ty, pat, .. }) => {
            let is_initialized;
            let ident: Ident = parse_quote!(#pat);
            let ty: Type = parse_quote!(#ty);
            if let Some(init) = &item.init {
                is_initialized = true;
                let expr = &init.expr;
                struct_init.fields.push(parse_quote! { #pat: #expr });
            } else {
                is_initialized = false;
                struct_init
                    .fields
                    .push(parse_quote! { #pat: ::std::option::Option::None });
            }
            let fields = &mut item_struct.fields;

            let visit_expr: Expr = parse_quote!(self.#ident.clone());

            if is_meta {
                meta_list.push(visit_expr.clone());
            }
            if is_flag {
                flag_list.push(visit_expr.clone());
            }

            if let Fields::Named(named) = fields {
                if is_initialized {
                    named.named.push(parse_quote! { pub #ident: #ty });
                } else {
                    named
                        .named
                        .push(parse_quote! { pub #ident: ::std::option::Option<#ty> });
                }
            }
        }
        _ => {}
    }
}

fn handle_mod(
    item_mod: &syn::ItemMod,
    Context {
        mod_items,
        state_ident,
        server_cfg, // 传入 context
        client_cfg, // 传入 context
        ..
    }: Context,
) {
    let mod_name = &item_mod.ident;

    let mk_cfg = |cfg: &Option<Meta>| -> proc_macro2::TokenStream {
        if let Some(c) = cfg {
            quote!(#[cfg(#c)])
        } else {
            quote!()
        }
    };
    let s_cfg_attr = mk_cfg(server_cfg);
    let c_cfg_attr = mk_cfg(client_cfg);

    mod_items.push(parse_quote! {
        #[derive(Debug, Clone, Copy)]
        pub struct #mod_name;
    });

    mod_items.push(parse_quote! {
        impl #mod_name {
            // Server 端获取实例 (给 handle 用)
            #s_cfg_attr
            pub async fn server_instance() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<super::#state_ident>>> {
                super::#state_ident::server_instance().await
            }

            // Client 端获取实例 (给 on/export 用)
            #c_cfg_attr
            pub async fn client_instance() -> ::vase::ipc::Result<::std::sync::Arc<::tokio::sync::Mutex<super::#state_ident>>> {
                super::#state_ident::client_instance().await
            }

            #s_cfg_attr
            pub async fn call<Pa: ::serde::Serialize + ::serde::de::DeserializeOwned + Send>(package: &str, method: &str, args: Pa) -> ::vase::ipc::Result<::serde_json::Value> {
                use ::vase::ipc::device::traits::Device;
                let instance = Self::server_instance().await?;
                let mut instance = instance.lock().await;
                instance.unicast(package, &format!("{}::{}", stringify!(#mod_name), method), ::serde_json::to_value((args,))?).await
            }
        }
    });
}
