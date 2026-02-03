use std::{fs, path::PathBuf};

use serde::Deserialize;
use toml;

use crate::extract::error::Result;

#[derive(Debug, Deserialize)]
pub struct CargoMetadata {
    pub package: Package,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
}

pub fn extract_cargo_matedata(file_path: &PathBuf) -> Result<CargoMetadata> {
    let file_content = fs::read_to_string(file_path)?;
    let metadata: CargoMetadata = toml::from_str(&file_content)?;
    Ok(metadata)
}
