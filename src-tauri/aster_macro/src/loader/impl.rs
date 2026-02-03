use aster_common::utils::create_string_literal;
use aster_common::utils::IntoIdent;
use common::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::loader::ty::LoadActionInput;

pub fn load_action_impl(input: TokenStream) -> TokenStream {
    let action = parse_macro_input!(input as LoadActionInput);
    let group = action.name;

    let group_str = group.to_string();
    let group_lit = create_string_literal(&group_str);
    let file_name = create_string_literal(&format!("actions/{}/src/lib.rs", &group_str));

    let mod_name = &format!("{}_lib", &group_str).into_ident();

    let mut token_stream_list = vec![];

    let mut expand = quote! {
        #[::aster_macro::hot_module(dylib = #group_lit)]
        mod #mod_name {
            hot_functions_from_file!(#file_name);
        }
    };
    // 检查函数是否为异步函数，如果是则包装为 block_on 调用
    for func in action.funcs.iter() {
        // 生成 Action 结构体名称（UpperCamelCase）
        let action_struct = &utils::to_upper_camel_case(&group_str).into_ident();

        let action_name = &func.name;
        let action_str = action_name.to_string();
        let action_lit = create_string_literal(&action_str);

        let func_call = quote! { #mod_name::#action_name(args) };
        let maybe_block_on = if func.is_async {
            quote! {
                // 异步函数需要使用 block_on 来在同步上下文中执行
                ::common::executor::get_executor().block_on(#func_call)
            }
        } else {
            func_call
        };
        let creator_name = quote::format_ident!("create_{}", action_name);

        token_stream_list.push(quote! {
            // 生成 Action 结构体
            pub struct #action_struct;

            // 为 Action 结构体实现 ActionTrait
            impl ::common::action::ActionTrait for #action_struct {
                fn get_action(&self, name: ::std::string::String, args: ::common::ty::Data) -> ::common::action::Action {
                    // 使用生成的 action_type 字符串
                    self.new_action(#action_lit, name, args)
                }

                fn run(&self, args: ::common::ty::Data) -> ::std::result::Result<::common::ty::CardResult, ::common::action::error::ActionError> {
                    let args: ::serde_json::Value = args.to_value();
                    // 调用原始函数（可能包装了 block_on）
                    let result = #maybe_block_on
                        .map_err(|e| ::common::action::error::ActionError::RunActionCardError(e.to_string()))?;

                    ::std::result::Result::Ok(result)
                }
            }

            fn #creator_name() -> ::std::boxed::Box<dyn ::common::action::ActionTrait> {
                ::std::boxed::Box::new(#action_struct)
            }

            ::inventory::submit!(crate::collector::ActionCreatorInfo {
                action_type: #action_lit,      // Action 类型字符串
                creator_fn: #creator_name,      // 创建器函数指针
            });
        });
    }

    expand.extend(token_stream_list);
    expand.into()
}
