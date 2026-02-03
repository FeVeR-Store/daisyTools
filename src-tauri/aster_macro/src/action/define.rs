use aster_common::action::param::{parse_param_attributes, ParamInfo};
use aster_common::nesting::NESTING_PRIFIX;
use common::utils::to_upper_camel_case;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn;
use syn::token::Pub;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, token::Comma, visit_mut::VisitMut,
    Expr, ExprCall, ExprStruct, FnArg, Ident, ItemFn, Member, ReturnType, Token, Type,
};

use crate::utils::{create_destructuring_pattern, create_struct_with_dynamic_fields};

pub fn define_action_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_fn = parse_macro_input!(input as ItemFn);

    // 获取函数名，这里的名称是snake_case
    // 将用于：
    // 1. 直接作为函数调用
    // 2. 转化为字面量之后作为action_type
    // 3. 转化为UpperCamelCase之后作为action_struct名称
    let action_name = impl_fn.sig.ident.clone();
    let action_name_str = &action_name.to_string();

    let return_type = match &impl_fn.sig.output {
        ReturnType::Type(_, ty) => {
            let ty = ty.as_ref();
            ReturnType::Type(
                Token![->](Span::call_site()),
                Box::new(Type::Verbatim(
                    quote! {::std::result::Result<#ty, ::std::boxed::Box<dyn ::std::error::Error>>},
                )),
            )
        }
        _ => panic!("Return type is required"),
    };

    impl_fn.sig.output = return_type;
    impl_fn.vis = syn::Visibility::Public(Pub {
        span: Span::call_site(),
    });

    // 存储所有参数信息的变量
    let mut all_param_info: Vec<ParamInfo> = Vec::new();

    // 遍历函数的所有参数，提取参数名和类型
    let args = impl_fn
        .sig
        .inputs
        .iter_mut()
        .enumerate()
        .map(|(_, arg)| match arg {
            // 不允许使用 self 参数，因为这是静态函数
            syn::FnArg::Receiver(_) => panic!("'self' is not allowed!"),
            syn::FnArg::Typed(typed_param) => {
                if let syn::Pat::Ident(pat_ident) = *typed_param.pat.clone() {
                    let param_name = &pat_ident.ident;
                    let param_type = &typed_param.ty;
                    let param_name_str = param_name.to_string();
                    let param_type_str = quote! { #param_type }.to_string();

                    // 使用 Darling 解析参数属性
                    match parse_param_attributes(
                        &typed_param
                            .attrs
                            .iter()
                            .map(|e| darling::ast::NestedMeta::Meta(e.meta.clone()))
                            .collect::<Vec<_>>(),
                    ) {
                        Ok(param_attr) => {
                            // 创建参数信息并存储
                            let param_info =
                                ParamInfo::new(param_name_str.clone(), param_type_str, param_attr);
                            all_param_info.push(param_info);
                        }
                        Err(e) => {
                            // 如果解析失败，创建一个空的属性配置
                            eprintln!("解析参数 '{}' 的属性失败: {}", param_name_str, e);
                            let param_info = ParamInfo::new(
                                param_name_str.clone(),
                                param_type_str,
                                Default::default(),
                            );
                            all_param_info.push(param_info);
                        }
                    }

                    // 清理已处理的属性
                    typed_param.attrs.retain(|attr| {
                        let path = attr.path();
                        match path.get_ident() {
                            Some(ident) => {
                                let ident_str = ident.to_string();
                                !["name", "description", "default"].contains(&(&ident_str as &str))
                            }
                            _ => true,
                        }
                    });

                    // 返回 (参数名字符串, 参数类型)
                    (param_name_str, param_type)
                } else {
                    panic!("Unsupported pattern")
                }
            }
        })
        .collect::<Vec<_>>();

    // 生成 Action 结构体名称（UpperCamelCase）
    let action_struct = Ident::new(&to_upper_camel_case(&action_name_str), Span::call_site());
    let action_struct_str = &action_struct.to_string();

    // 参数结构体命名为 [ActionName]Arg
    let action_arg_str = format!("{}Arg", &action_struct_str);

    // 使用工具函数动态生成参数结构体定义
    let impl_action_arg = create_struct_with_dynamic_fields(&action_arg_str, args.clone());

    // 提取所有参数名用于函数调用
    let arg_list = args
        .iter()
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();

    // 生成参数解构模式，用于从结构体中提取各个字段
    let destructuring_pattern = create_destructuring_pattern(&action_arg_str, arg_list);

    let mut visitor = ActionVisitor::new();
    visitor.visit_item_fn_mut(&mut impl_fn);

    // 使用模式匹配解构参数结构体，提取各个字段
    // 保持传递的参数为序列化结构体，并在函数内解构取值
    // 避免函数签名变化导致热重载失效
    impl_fn.block.stmts.insert(
        0,
        parse_quote!(let #destructuring_pattern = ::serde_json::from_value(arg)?;),
    );

    let mut fn_input: Punctuated<FnArg, Comma> = Punctuated::new();
    fn_input.push(parse_quote!(arg: ::serde_json::Value));

    impl_fn.sig.inputs = fn_input;

    // 生成最终代码
    let expanded = quote! {
        use ::aster_macro::*;
        // 保留原始函数定义
        #[unsafe(no_mangle)]
        #impl_fn

        // 生成参数结构体，自动实现 Debug 和 Deserialize
        #[derive(Debug, ::serde::Deserialize)]
        #impl_action_arg
    };

    // 将 quote! 生成的代码转换为 TokenStream 返回给编译器
    TokenStream::from(expanded)
}

struct ActionVisitor {
    nesting_stack: Vec<String>,
}

impl ActionVisitor {
    fn new() -> Self {
        ActionVisitor {
            nesting_stack: vec![],
        }
    }
}

impl VisitMut for ActionVisitor {
    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        // 只匹配单参数调用，且参数是 struct literal "__ { … }"
        if node.args.len() == 1 {
            if let Expr::Struct(inner) = &mut node.args[0] {
                if inner.path.segments.len() == 1 && inner.path.segments[0].ident == "__" {
                    // 准备前缀："Result_Error" 这类
                    if let Expr::Path(func_path) = &*node.func {
                        let parts: Vec<String> = func_path
                            .path
                            .segments
                            .iter()
                            .map(|seg| seg.ident.to_string())
                            .collect();
                        let prefix = parts.join("_");

                        // 重命名外层 struct literal
                        inner.path.segments[0].ident = Ident::new(
                            &format!("{}_{}", NESTING_PRIFIX, prefix),
                            inner.path.segments[0].ident.span(),
                        );

                        // 把 prefix push 进 ctx，再递归处理它的字段
                        self.nesting_stack.push(prefix);
                        self.visit_expr_struct_mut(inner);
                        self.nesting_stack.pop();
                        return; // 跳过默认 recursion
                    }
                }
            }
        }
    }
    fn visit_expr_struct_mut(&mut self, node: &mut ExprStruct) {
        // 先判断这是不是一个「真实」的 struct literal，而非我们生成的 NESTING_PRIFIX_* 也非占位符 "__"
        let seg = &node.path.segments[0].ident;
        let is_real_struct =
            seg != "__" && !seg.to_string().starts_with(&format!("{}_", NESTING_PRIFIX));
        if is_real_struct {
            // push 真实的类型名，比如 "Error"
            self.nesting_stack.push(seg.to_string());
        }

        // 遍历所有 field，寻找那些形如 `foo: __ { … }` 的子节点
        for field in &mut node.fields {
            if let Expr::Struct(inner) = &mut field.expr {
                // 如果是我们要替换的占位符
                if inner.path.segments[0].ident == "__" {
                    if let Member::Named(ref ident) = field.member {
                        // 拼前缀：ctx + 当前字段名
                        let mut parts = self.nesting_stack.clone();
                        parts.push(ident.to_string());
                        let prefix = parts.join("_");

                        // 替换类型名为 NESTING_PRIFIX_<prefix>
                        inner.path.segments[0].ident = syn::Ident::new(
                            &format!("{}_{}", NESTING_PRIFIX, prefix),
                            ident.span(),
                        );

                        // 进入这个刚改名的 struct literal 里，继续处理更深的嵌套
                        // 但注意：这里我们只往 ctx 里 push *字段名*，而不是 push 完整的 "NESTING_PRIFIX_*"
                        self.nesting_stack.push(ident.to_string());
                        // 递归
                        self.visit_expr_struct_mut(inner);
                        // 回退
                        self.nesting_stack.pop();
                        // 跳过默认的 visit，避免重复
                        continue;
                    }
                }
            }
            // 对其他所有 expr 也要继续默认遍历，找到深层的 struct literal
            syn::visit_mut::visit_expr_mut(self, &mut field.expr);
        }

        if is_real_struct {
            // 退栈
            self.nesting_stack.pop();
        }
    }
}
