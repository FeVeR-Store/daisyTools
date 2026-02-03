#!vase::entry


use std::{env, fs::read, path::PathBuf};

use walkdir::WalkDir;

use crate::{
    error::Result,
    index::{FileIndex, load_index},
    utils::{ToString, normalize_path},
};

pub fn scan_dir() -> Result<()> {
    let mut index_map = load_index()?;
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    while let Some(dir) = WalkDir::new(&manifest_dir)
        .into_iter()
        .find_map(std::result::Result::ok)
    {
        let path = normalize_path(dir.path());
        let metadata = dir.metadata()?;
        let mtime = metadata.modified()?.to_string();
        let size = metadata.len();
        let Some(index) = index_map.get(&path) else {
            let (hash, _content) = read_file_with_hash(&path);
            index_map.insert(
                path.clone(),
                FileIndex {
                    path,
                    mtime,
                    size,
                    hash,
                },
            );
            continue;
        };
        if index.size == size && index.mtime == mtime {
            continue;
        }
        let (hash, _content) = read_file_with_hash(&path);
        index_map.insert(
            path.clone(),
            FileIndex {
                path: path.clone(),
                mtime,
                size,
                hash,
            },
        );
    }
    Ok(())
}

#[inline]
fn read_file_with_hash(path: &PathBuf) -> (u32, Vec<u8>) {
    read(&path)
        .map(|content| (crc32fast::hash(&content), content))
        .unwrap_or_default()
}
