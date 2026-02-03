use std::{collections::HashMap, fs::write};

use serde_json::Value;

use crate::{
    action::{
        Action, entry::ActionEntry, error::ActionError,
        manager::ActionManager,
    },
    application::Application,
};

impl Action {
    pub fn create_workflow(
        workflow: &HashMap<String, ActionEntry>,
    ) -> Result<HashMap<String, Action>, ActionError> {
        let action_list = Application::get_action_list();
        let action_map: HashMap<_, _> = action_list.iter().map(|item| (&item.id, item)).collect();
        Ok(workflow
            .iter()
            .filter_map(|(key, action)| match action {
                ActionEntry::LitRef { id, .. } => action_map
                    .get(&id)
                    .and_then(|&action| Some((key.clone(), action.clone()))),
                ActionEntry::Inline {
                    uid,
                    r#type: ty,
                    data,
                } => Some((
                    key.clone(),
                    Action {
                        id: uid.to_string(),
                        label: String::new(),
                        r#type: ty.to_string(),
                        data: data.clone(),
                        plug: Value::Null,
                    },
                )),
            })
            .collect::<HashMap<_, _>>())
    }
    pub fn find_from_id(id: &str) -> Result<Action, ActionError> {
        let action_list = Application::get_action_list();
        let action = action_list.iter().find(|action| action.id == id);
        if action.is_none() {
            return Err(ActionError::RunActionCardError(format!(
                "Action id {} not found",
                id
            )));
        }
        Ok(action.unwrap().clone())
    }
    // pub fn run(&self, context: &HashMap<String, Data>) -> Result<CardResult, ActionError> {
    //     info!("Run action {}", &self.id);
    //     let action_type = self.r#type.as_str();
    //     info!("Action type: {}", action_type);
    //     let action = Self::get (action_type)?;
    //     let data = parse_data(context, self.data.clone())
    //         .map_err(|e| ActionError::RunActionCardError(e.to_string()))?;
    //     Ok(action.run(data)?)
    // }
    // 自动发现所有注册的 Action
    // pub fn get_action_instance_from_type(
    //     action_type: &str,
    // ) -> Result<Box<dyn ActionTrait>, ActionError> {
    //     for creator_info in inventory::iter::<ActionCreatorInfo> {
    //         println!("{:?}", creator_info.action_type);
    //         if creator_info.action_type == action_type {
    //             return Ok((creator_info.creator_fn)());
    //         }
    //     }
    //     Err(ActionError::RunActionCardError(format!(
    //         "Action type {} not found",
    //         action_type
    //     )))
    // }
    pub fn remove(id: &str) -> Result<(), ActionError> {
        let mut action_list = Application::get_action_list();
        let index = action_list.iter().position(|trigger| trigger.id == id);
        if index.is_none() {
            return Err(ActionError::RemoveActionError(
                id.to_string(),
                "not found".to_string(),
            ));
        }
        let index = index.unwrap();
        action_list.remove(index);
        let content = serde_json::to_string(&action_list)
            .map_err(|e| ActionError::RemoveActionError(id.to_string(), e.to_string()))?;
        let path = Application::get_action_file();
        write(path, content)
            .map_err(|e| ActionError::RemoveActionError(id.to_string(), e.to_string()))?;
        Ok(())
    }
}
