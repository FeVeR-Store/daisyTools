use std::{
    fs::{create_dir, exists, read_to_string, write},
    path::PathBuf,
};

use common::{application::Application, utils::get_uid};
use error::RuntimeError;

pub mod error;
pub mod javascript;

pub trait Runtime {
    fn new() -> Self;
    fn language(&self) -> String;
    fn execute(&self, code: String) -> Result<(), RuntimeError>;
    fn create_task(&self, code: &str) -> Result<String, RuntimeError>;
}

pub trait RuntimeManager {
    fn get_script_dir() -> PathBuf;
    fn create_script(code: &str, ext: &str) -> Result<String, RuntimeError>;
    fn get_script_by_id(action_id: String, ext: &str) -> Result<String, RuntimeError>;
}

impl RuntimeManager for Application {
    fn get_script_dir() -> PathBuf {
        Self::get_path("script")
    }
    fn create_script(code: &str, ext: &str) -> Result<String, RuntimeError> {
        let id = get_uid();
        let path = Self::get_script_dir();
        if !exists(&path).unwrap() {
            create_dir(&path).map_err(|e| RuntimeError::CreateScriptError(e.to_string()))?;
        }
        write(path.join(id.clone() + "." + ext), code)
            .map_err(|e| RuntimeError::CreateScriptError(e.to_string()))?;
        Ok(id)
    }
    fn get_script_by_id(action_id: String, ext: &str) -> Result<String, RuntimeError> {
        let content = read_to_string(Application::get_script_dir().join(action_id + "." + ext))
            .map_err(|e| RuntimeError::ReadScriptError(e.to_string()))?;
        Ok(content)
    }
}
