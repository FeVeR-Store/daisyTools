use std::{fs::{exists, read_to_string, write}, path::PathBuf};

use crate::{action::{error::ActionError, Action}, application::Application};

pub trait ActionManager {
    fn get_action_file() -> PathBuf;
    fn get_action_list() -> Vec<Action>;
    fn lit_action(action: Action) -> Result<(), ActionError>;
}


impl ActionManager for Application {
    fn get_action_file() -> PathBuf {
        Self::get_path("action.json")
    }
    fn get_action_list() -> Vec<Action> {
        let path = Self::get_action_file();
        if !exists(path.clone()).unwrap() {
            write(path, "[]").unwrap();
            return vec![];
        }
        let result = read_to_string(path).unwrap();
        let result: Vec<Action> = serde_json::from_str(&result).unwrap();
        result
    }
    /// 点亮一个action，如果action已经存在，则更新
    fn lit_action(action: Action) -> Result<(), ActionError> {
        let path = Self::get_action_file();
        let mut action_list = Application::get_action_list();
        let index = action_list
            .iter()
            .position(|current_action| current_action.id == action.id);
        if index.is_some() {
            action_list[index.unwrap()] = action;
        } else {
            action_list.push(action);
        }
        let content = serde_json::to_string(&action_list)
            .map_err(|e| ActionError::LitActionCardError(e.to_string()))?;
        write(path, content).map_err(|e| ActionError::LitActionCardError(e.to_string()))?;
        Ok(())
    }
}
