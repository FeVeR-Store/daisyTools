use std::time::SystemTime;

use time::OffsetDateTime;

pub trait ToString {
    fn to_string(self) -> String;
}

impl ToString for SystemTime {
    fn to_string(self) -> String {
        let t: OffsetDateTime = SystemTime::now().into();
        format!(
            "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
            t.year(),
            t.month() as u8,
            t.day(),
            t.hour(),
            t.minute(),
            t.second()
        )
    }
}

use std::{
    env,
    path::{Path, PathBuf},
};

/// 统一路径表示，避免 HashMap 出现重复键
pub fn normalize_path(path: &Path) -> PathBuf {
    // 获取 crate 根目录（CARGO_MANIFEST_DIR）
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // 转成绝对路径
    let abs = if path.is_absolute() {
        path.to_path_buf()
    } else {
        manifest_dir.join(path)
    };

    // 规范化（消除 . 和 ..）
    let canon = abs.components().collect::<PathBuf>();

    // 再尝试相对化：去掉前缀 manifest_dir
    let rel = canon.strip_prefix(&manifest_dir).unwrap_or(&canon);

    // 注意：PathBuf 内部分隔符会自动跟随平台
    // 如果要统一成 `/`（跨平台 JSON/持久化），可以转 String 再替换
    rel.to_path_buf()
}

pub trait Idents {
    fn get_idents(&self) -> Vec<String>;
    fn is_idents(&self, idents: &[&str]) -> bool;
}

impl Idents for syn::Path {
    fn get_idents(&self) -> Vec<String> {
        self.segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .collect()
    }
    fn is_idents(&self, idents: &[&str]) -> bool {
        self.get_idents() == idents
    }
}
