pub mod path;
use std::{
    collections::HashMap,
    env,
    fs::File,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{error::Result, utils::normalize_path};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileIndex {
    pub path: PathBuf,
    pub mtime: String,
    pub size: u64,
    pub hash: u32,
}

const INDEX_FILE: &str = "vase-scan-index";

pub fn load_index() -> Result<HashMap<PathBuf, FileIndex>> {
    let out_dir = env::var("OUT_DIR")?;
    let index_file = Path::new(&out_dir).join(INDEX_FILE);
    let mut index_map = HashMap::new();
    if !index_file.exists() {
        return Ok(index_map);
    }
    let file = File::open(index_file)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let index: FileIndex = result?;
        index_map.insert(normalize_path(&index.path), index);
    }
    Ok(index_map)
}
