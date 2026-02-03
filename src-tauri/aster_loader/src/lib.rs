use std::collections::HashMap;

use common::{
    action::{Action, ActionTrait, error::ActionError},
    ty::{CardResult, Data, type_convert::parse_data},
};

use crate::collector::ActionCreatorInfo;

pub mod collector;
pub mod manifest;

pub trait ActionProvider {
    fn get_action_instance_from_type(
        action_type: &str,
    ) -> Result<Box<dyn ActionTrait>, ActionError>;
    fn run(&self, context: &HashMap<String, Data>) -> Result<CardResult, ActionError>;
}

impl ActionProvider for Action {
    fn get_action_instance_from_type(
        action_type: &str,
    ) -> Result<Box<dyn common::action::ActionTrait>, common::action::error::ActionError> {
        for creator_info in inventory::iter::<ActionCreatorInfo>.into_iter() {
            println!("{:?}", creator_info.action_type);
            if creator_info.action_type == action_type {
                return Ok((creator_info.creator_fn)());
            }
        }
        Err(ActionError::RunActionCardError(format!(
            "Action type {} not found",
            action_type
        )))
    }
    fn run(&self, context: &HashMap<String, Data>) -> Result<CardResult, ActionError> {
        // info!("Run action {}", &self.id);
        let action_type = self.r#type.as_str();
        // info!("Action type: {}", action_type);
        let action = Self::get_action_instance_from_type(action_type)?;
        let data = parse_data(context, self.data.clone())
            .map_err(|e| ActionError::RunActionCardError(e.to_string()))?;
        Ok(action.run(data)?)
    }
}
