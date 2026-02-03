use std::collections::HashMap;

use aster_loader::ActionProvider;
use common::{
    action::{manager::ActionManager, Action},
    application::Application,
    ty::Data,
};
use serde_json::Value;

pub fn rm_action_impl(id: &str) -> Result<(), String> {
    Action::remove(id).map_err(|e| e.to_string())
}

#[tauri::command]
/// 删除action
pub fn remove_action(id: &str) -> Result<(), String> {
    Action::remove(id).map_err(|e| e.to_string())
}
/// 注册action
#[tauri::command]
pub fn register_action(action_type: String, name: String, args: Data) -> Result<String, String> {
    let action = Action::get_action_instance_from_type(&action_type).map_err(|e| e.to_string())?;
    let id = action.lit(name, args).map_err(|e| e.to_string())?;
    Ok(id)
}
#[tauri::command]
/// 获取已点亮action
pub fn get_lit_action() -> Result<Vec<Action>, String> {
    Ok(Application::get_action_list())
}

#[tauri::command]
/// 更新action插头
pub fn update_action_plug(id: String, plug: Value) -> Result<Action, String> {
    let mut action = Action::find_from_id(&id).map_err(|e| e.to_string())?;
    action.plug = plug;
    Application::lit_action(action.clone()).map_err(|e| e.to_string())?;
    Ok(action)
}

#[tauri::command]
/// 根据id运行action
pub fn run_action_by_id(id: String) -> Result<(), String> {
    let action = Action::find_from_id(&id).map_err(|e| e.to_string())?;
    action.run(&HashMap::new()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
/// 根据类型运行action
pub fn run_action(action_type: String, args: Data) -> Result<(), String> {
    let action = Action::get_action_instance_from_type(&action_type).map_err(|e| e.to_string())?;
    action.run(args).map_err(|e| e.to_string())?;
    Ok(())
}
