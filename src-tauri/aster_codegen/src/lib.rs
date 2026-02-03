pub mod error;
pub mod utils;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
pub mod extract;

use aster_common::{
    action::{
        form::{FormItem, FormType},
        stat::Stat,
    },
    collect::{FormDataCollect, ResultBranchTypeCollect},
    i18n::{I18nValue, ParamI18n, ParsedI18nMap, ParsedI18nMapTrait},
    typescript::expr::{
        base::FromBase,
        object::{ExprValue, ToObjectExpr, create_array_expr, create_object_expr},
    },
};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use swc_atoms::Atom;
use swc_common::{SourceMap, Spanned};
use swc_ecma_ast::{Expr, Ident};
use swc_ecma_codegen::{
    Emitter,
    text_writer::{JsWriter, WriteJs},
};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::{
    error::Result,
    utils::{FromType, IntoI18nValueList},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CardInfo {
    #[serde(default)]
    pub title: ParsedI18nMap,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub file: String,
    #[serde(default)]
    pub result: String,

    pub action_type: String,
    pub description: ParsedI18nMap,
    pub entries: Vec<String>,
    pub keys: Vec<String>,
    pub params: Vec<ParamI18n>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Plug {
    Unknown,
    None,
    Error,
    Base(String),
    Value(Map<String, serde_json::Value>),
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub(crate) struct ResultBranchType {
    /// 分支名称
    pub branch: String,
    /// 分支类型id
    pub id: String,
    /// 分支所属的类型，目前有 primary | error
    pub r#type: String,
    /// handle位置
    pub position: String,
    /// i18n
    pub i18n: Option<ParsedI18nMap>,
    /// 插头类型
    pub plug: Plug,
}

#[derive(Debug, Clone)]
pub enum ReplaceValue {
    Ident(String),
    String(String),
    Expr(Expr),
}

struct PlaceholderReplacer {
    replacements: HashMap<String, ReplaceValue>,
}

#[derive(Debug)]
pub struct GenerateCodeContext<'a> {
    pub dist: PathBuf,
    pub card_info_list: &'a mut Vec<CardInfo>,
    pub form_data_list: &'a mut Vec<FormDataCollect>,
    pub result_branch_list: &'a mut Vec<ResultBranchTypeCollect>,
}

pub fn generate_code(ctx: &GenerateCodeContext) -> Result<()> {
    meta_data_to_typescript(&ctx)?;
    Ok(())
}

pub fn create_replacement(
    card_info: &CardInfo,
    ctx: &GenerateCodeContext,
) -> HashMap<String, ReplaceValue> {
    let mut replacements: HashMap<String, ReplaceValue> = HashMap::new();
    let action_type = &card_info.action_type;
    // 创建 $args$ 同时将表单和展示进行初始化
    let mut form = vec![];
    let mut stat = vec![];
    let args = {
        let object = create_object_expr(
            card_info
                .params
                .iter()
                .map(|ParamI18n { key, r#type, .. }| {
                    let form_type = FormType::from_type(r#type.clone());
                    let args_type = form_type.get_args_type();
                    let _effect = {
                        let width = form_type.get_width();
                        form.push(FormItem {
                            args_type: FormType::get_inner_type(r#type).unwrap_or(r#type.clone()),
                            name: key.clone(),
                            optional: form_type.is_optional(),
                            r#type: form_type,
                            default: None,
                            data: None,
                        });

                        stat.push(Stat {
                            key: key.clone(),
                            width,
                        });
                    };

                    return (key.clone(), ExprValue::String(args_type));
                })
                .collect::<Vec<_>>()
                .to_object_entry(),
        );
        ("args", ReplaceValue::Expr(object))
    };

    let mut form_i18n: HashMap<String, Vec<(String, I18nValue)>> = HashMap::new();

    let form_item_types = &ctx.form_data_list;

    let form = form
        .iter()
        .map(|item| {
            let mut item: FormItem = item.clone();

            let form = form_item_types.iter().find(|f| f.name == item.args_type);
            if let Some(form) = form {
                let form_data = &form.data;
                let i18n = form_data.get_i18n(&form.name);
                i18n.iter().for_each(|(lang, value)| {
                    form_i18n
                        .entry(lang.clone())
                        .or_insert_with(Vec::new)
                        .extend(value.clone());
                });
                item.r#type.fix_with(&form_data);
                item.data = Some(form_data.clone());
            }
            item.to_object_expr()
        })
        .collect::<Vec<_>>();

    let stat = stat
        .iter()
        .map(|stat| stat.to_object_expr())
        .collect::<Vec<_>>();

    let branches = {
        let result_branch_collectors = &ctx.result_branch_list;
        let default = &&ResultBranchTypeCollect::default();

        let branch_collector = result_branch_collectors
            .iter()
            .find(|e| e.name == card_info.result && e.file == card_info.file)
            .unwrap_or(default);

        let branches = &branch_collector.data;
        branches.iter().for_each(|b| {
            if b.id == card_info.result {
                let i18n = &b.get_i18n();
                (b.branch.as_str(), i18n).insert_into(&mut form_i18n);
            };
        });
        branches.to_object_expr()
    };

    let i18n = {
        let mut langs: HashMap<String, Vec<(String, I18nValue)>> = form_i18n;
        ("description", &card_info.description.to_filter_value()).insert_into(&mut langs);
        ("title", &card_info.title.to_filter_value()).insert_into(&mut langs);
        for ParamI18n {
            description,
            key,
            name,
            ..
        } in card_info.params.iter()
        {
            (key.as_str(), name, description).insert_into(&mut langs);
        }
        let langs = langs
            .iter()
            .map(|(lang, value)| {
                (
                    lang.to_string(),
                    value
                        .iter()
                        .map(|(key, value)| match value {
                            I18nValue::Single(str) => (key, ExprValue::String(str.to_string())),
                            I18nValue::WithDescription { title, description } => (
                                key,
                                ExprValue::Entries(
                                    vec![
                                        ("title".to_string(), ExprValue::String(title.to_string())),
                                        (
                                            "description".to_string(),
                                            ExprValue::String(description.to_string()),
                                        ),
                                    ]
                                    .to_object_entry(),
                                ),
                            ),
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        let i18n = create_object_expr(
            langs
                .iter()
                .map(|(lang, i18n_value)| {
                    (
                        lang.to_string(),
                        ExprValue::Entries(
                            i18n_value
                                .iter()
                                .map(|(key, value)| (key.to_string(), value.clone()))
                                .collect::<Vec<_>>()
                                .to_object_entry(),
                        ),
                    )
                })
                .collect::<Vec<_>>()
                .to_object_entry(),
        );
        ("i18n", ReplaceValue::Expr(i18n))
    };

    let replace_map = vec![
        (
            "card",
            ReplaceValue::Ident(card_info.action_type.to_string()),
        ),
        ("action_type", ReplaceValue::String(action_type.to_string())),
        ("parent", ReplaceValue::String(card_info.parent.to_string())),
        ("form", ReplaceValue::Expr(create_array_expr(form))),
        ("stat", ReplaceValue::Expr(create_array_expr(stat))),
        ("branches", ReplaceValue::Expr(branches)),
        args,
        i18n,
    ];

    for (key, value) in replace_map {
        replacements.insert(key.into(), value);
    }
    replacements
}

fn meta_data_to_typescript(ctx: &GenerateCodeContext) -> Result<()> {
    let src = include_str!("./action.template.ts").to_string();

    let lexer = swc_ecma_parser::lexer::Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: false,
            ..Default::default()
        }),
        swc_ecma_ast::EsVersion::Es2022,
        StringInput::new(&src, Default::default(), Default::default()),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let module = parser
        .parse_module()
        .map_err(|e| error::Error::ParseTemplateError(e.kind().msg().to_string()))?;

    for card_info in ctx.card_info_list.iter() {
        let cm = swc_common::sync::Lrc::new(SourceMap::default());
        let mut template = module.clone();
        let replacements = create_replacement(&card_info, ctx);
        let mut visitor = PlaceholderReplacer { replacements };
        template.visit_mut_with(&mut visitor);
        let mut buffer = Vec::new();
        let target = ctx
            .dist
            .join(format!("{}.ts", card_info.action_type.clone()));

        let split_tag = format!("/* This section can be used to extend or override */");
        let default_code = format!("export default {};", card_info.action_type.clone()).to_string();
        let extend_part = if target.exists() {
            let content = read_to_string(&target).unwrap();
            let parts = content.split(&split_tag).collect::<Vec<_>>();
            if parts.len() > 1 && !parts[1].is_empty() {
                parts[1].to_string()
            } else {
                default_code
            }
        } else {
            default_code
        };

        {
            let generated_mark = &format!(
                "/* This part is the automatically generated source code. Please modify it in {} */\n\n",
                card_info.file
            );
            let mut writer = JsWriter::new(cm.clone(), "\n", &mut buffer, None);
            writer.write_comment(generated_mark).unwrap();
            let mut emitter = Emitter {
                cfg: Default::default(),
                comments: None,
                cm,
                wr: Box::new(writer),
            };
            emitter
                .emit_module(&template)
                .map_err(|e| error::Error::EmitTsCodeError(e.to_string()))?;
        }

        let output = String::from_utf8(buffer).expect("Invalid UTF-8");
        std::fs::write(
            target.clone(),
            [output, split_tag, extend_part.trim_start().to_string()]
                .join("\n")
                .to_string(),
        )
        .map_err(|e| error::Error::WriteFileError(target, e.to_string()))?;
    }
    Ok(())
}

impl VisitMut for PlaceholderReplacer {
    fn visit_mut_ident(&mut self, node: &mut Ident) {
        if let Some(ident) = is_placeholder_ident(node) {
            if let Some(value) = self.replacements.get(&ident) {
                match value {
                    ReplaceValue::Ident(str) => {
                        *node = Ident::new(Atom::new(str as &str), node.span(), node.ctxt)
                    }
                    _ => (),
                }
            }
        }
    }
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        node.visit_mut_children_with(self);
        match node {
            Expr::Ident(ident) => {
                if let Some(placeholder) = is_placeholder_ident(ident) {
                    if let Some(value) = self.replacements.get(&placeholder) {
                        match value {
                            ReplaceValue::Expr(expr) => *node = expr.clone(),
                            ReplaceValue::String(str) => *node = Expr::from_str(&str),
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn is_placeholder_ident(ident: &Ident) -> Option<String> {
    let sym = &ident.sym;
    if sym.starts_with("$") && sym.ends_with("$") {
        Some(sym[1..sym.len() - 1].into())
    } else {
        None
    }
}
