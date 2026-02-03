use aster_common::action::{form::FormData, param::parse_param_attributes};
use aster_common::i18n::Label;
use darling::ast::NestedMeta;
use syn::ItemEnum;

pub fn extract_i18n_from_options(item: &ItemEnum) -> FormData {
    let mut options: Vec<_> = Vec::new();
    for (idx, variant) in item.variants.iter().enumerate() {
        options.push(Label {
            label: None,
            value: variant.ident.to_string(),
        });
        for attr in variant.attrs.iter() {
            match parse_param_attributes(&[NestedMeta::Meta(attr.meta.clone())]) {
                Ok(parsed) => {
                    options[idx] = Label {
                        label: parsed.label,
                        value: variant.ident.to_string(),
                    };
                }
                Err(e) => {
                    eprintln!("解析选项属性失败: {}", e);
                }
            }
        }
    }
    FormData::Option(options)
}
