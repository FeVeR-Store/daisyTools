pub mod entry;
pub mod error;
pub mod r#impl;
pub mod manager; 

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    action::{error::ActionError, manager::ActionManager},
    application::Application,
    ty::{CardResult, Data},
    utils::get_uid,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Action {
    pub id: String,
    pub label: String,
    pub r#type: String,
    pub data: Data,
    #[serde(default)]
    pub plug: Value,
}

pub trait ActionTrait {
    fn new_action(&self, r#type: &str, name: String, args: Data) -> Action {
        let id = get_uid();
        Action {
            id,
            label: name,
            r#type: r#type.to_string(),
            data: args,
            plug: Value::Null,
        }
    }
    fn lit(&self, name: String, args: Data) -> Result<String, ActionError> {
        let action = self.get_action(name, args);
        Application::lit_action(action.clone())?;
        Ok(action.id)
    }
    fn get_action(&self, name: String, args: Data) -> Action;
    fn run(&self, args: Data) -> Result<CardResult, ActionError>;
}
