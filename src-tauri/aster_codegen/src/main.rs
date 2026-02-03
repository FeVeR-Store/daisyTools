use std::{
    env::current_dir,
    error::Error,
    fs::{self, read_dir, read_to_string, write},
    path::PathBuf,
    str::FromStr,
};

use aster_codegen::{
    GenerateCodeContext,
    extract::{
        cargo_metadata::extract_cargo_matedata,
        i18n::{
            action::extract_i18n_from_action, options::extract_i18n_from_options,
            result::extract_i18n_and_position_from_result,
        },
    },
};
use aster_common::{
    collect::{FormDataCollect, ResultBranchTypeCollect},
    utils::IntoIdent,
};
use proc_macro2;
use quote::quote;
use syn::{self, Item, parse_quote, punctuated::Punctuated, token::Comma};
use toml_edit::{DocumentMut, Table, value};

const ACTIONS_DIR: &str = "actions";

const PATH_ASTER_LOADER: &str = "aster_loader";
const PATH_MANIFEST_RS: &str = "manifest.rs";

const PATH_CARGO_TOML: &str = "Cargo.toml";
const PATH_SRC: &str = "src";
const PATH_LIB_RS: &str = "lib.rs";

struct ParseFileContext<'a, 'b: 'a> {
    crate_name: &'a str,
    file: &'a syn::File,
    path: &'a str,
    stmts: &'a mut Vec<String>,
    generate_ctx: &'a mut GenerateCodeContext<'b>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = current_dir()?;
    let dir = PathBuf::from(ACTIONS_DIR);
    let dir = read_dir(dir)?;

    let manifest_rs = current_dir
        .join(PATH_ASTER_LOADER)
        .join(PATH_SRC)
        .join(PATH_MANIFEST_RS);
    let loader_cargo_metadata_path = PathBuf::from(PATH_ASTER_LOADER).join(PATH_CARGO_TOML);

    let mut loader_cargo_metadata =
        read_to_string(&loader_cargo_metadata_path)?.parse::<DocumentMut>()?;

    let mut deps_table = match loader_cargo_metadata["fixed-dependencies"].as_table() {
        Some(table) => {
            let mut table = table.clone();
            table.decor_mut().clear();
            table
        }
        None => Table::default(),
    };
    println!("{:?}", deps_table);

    let mut card_info_list = vec![];
    let mut form_data_list = vec![];
    let mut result_branch_list = vec![];

    let mut ctx = GenerateCodeContext {
        dist: PathBuf::from("../src/invoke/actions"),
        card_info_list: &mut card_info_list,
        form_data_list: &mut form_data_list,
        result_branch_list: &mut result_branch_list,
    };
    let mut stmts: Vec<String> = vec![];
    for entry in dir.into_iter().filter_map(Result::ok) {
        let path = entry.path();
        let cargo_metadata_path = path.join(PATH_CARGO_TOML);
        let cargo_metadata = extract_cargo_matedata(&cargo_metadata_path)?;
        let crate_name = cargo_metadata.package.name;

        let mut dep_details = Table::default();
        dep_details.insert("path", value(format!("../{}/{}", ACTIONS_DIR, crate_name)));
        deps_table.insert(&crate_name, dep_details.into());

        let src_dir = path.join(PATH_SRC);
        if !src_dir.exists() {
            return Err(aster_codegen::error::Error::PathNotExist(src_dir.clone()).into());
        }
        let lib_rs_path = src_dir.join(PATH_LIB_RS);
        let lib_rs = fs::read_to_string(lib_rs_path)?;
        let lib_rs_path = format!(
            "{}/{}/{}/{}",
            ACTIONS_DIR, &crate_name, PATH_SRC, PATH_LIB_RS
        );

        let tokens = proc_macro2::TokenStream::from_str(&lib_rs)?;
        let file: syn::File = parse_quote!(#tokens);

        parse_action(ParseFileContext {
            crate_name: &crate_name,
            file: &file,
            path: &lib_rs_path,
            generate_ctx: &mut ctx,
            stmts: &mut stmts,
        });
    }

    aster_codegen::generate_code(&ctx)?;

    let loader_contents = stmts.join("\n\n");

    loader_cargo_metadata["dependencies"] = deps_table.into();

    write(
        loader_cargo_metadata_path,
        loader_cargo_metadata.to_string(),
    )?;
    write(manifest_rs, loader_contents)?;

    Ok(())
}

enum EnumType {
    Options,
    Result,
    Others,
}

fn parse_action(
    ParseFileContext {
        crate_name,
        file,
        path,
        generate_ctx,
        stmts,
    }: ParseFileContext,
) {
    let mut actions: Punctuated<proc_macro2::TokenStream, Comma> = Punctuated::new();
    for e in file.items.iter() {
        let path = path.to_string();
        match e {
            Item::Fn(item) => {
                let mut card_info = extract_i18n_from_action(&item);
                let action_name = &card_info.action_type.into_ident();
                if item.sig.asyncness.is_some() {
                    actions.push(quote! { async #action_name })
                } else {
                    actions.push(quote! { #action_name });
                }

                card_info.parent = format!("action.{}", crate_name);
                card_info.file = path;
                generate_ctx.card_info_list.push(card_info);
            }
            Item::Enum(item) => {
                let item_name = item.ident.to_string();
                let mut enum_type = EnumType::Others;

                for attr in item.attrs.iter() {
                    let path = attr.path();
                    if path.is_ident("options") {
                        enum_type = EnumType::Options;
                        break;
                    } else if path.is_ident("result") {
                        enum_type = EnumType::Result;
                        break;
                    }
                }

                match enum_type {
                    EnumType::Options => {
                        let data = extract_i18n_from_options(&item);
                        generate_ctx.form_data_list.push(FormDataCollect {
                            file: path,
                            name: item_name,
                            data,
                        })
                    }
                    EnumType::Result => {
                        let data = extract_i18n_and_position_from_result(&item);
                        generate_ctx
                            .result_branch_list
                            .push(ResultBranchTypeCollect {
                                file: path,
                                name: item_name,
                                data,
                            });
                    }
                    EnumType::Others => (),
                }
            }
            _ => (),
        }
    }
    let crate_name = crate_name.into_ident();

    let tokens = quote! { ::aster_macro::load_action!(#crate_name, [#actions]); }.to_string();
    stmts.push(tokens);
}
