use common::application::Application;

use super::{Trigger, TriggerManager};

#[tauri::command]
pub fn register_trigger(trigger_type: String, name: String, args: common::ty::Data) -> Result<String, String> {
    let trigger = Trigger::find(&trigger_type);
    let id = trigger.lit(name, args).map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub async fn remove_trigger(id: &str) -> Result<(), String> {
    Trigger::remove(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_lit_trigger() -> Result<Vec<Trigger>, String> {
    Ok(Application::get_trigger_list())
}
