use std::path::PathBuf;

use aster_loader::ActionProvider;
use common::{action::Action, application::Application, ty::Data};
use tauri::{generate_context, generate_handler, LogicalPosition, LogicalSize, Manager};

use crate::{elevation::request_elevation, runtime::RuntimeManager};

use super::{error::RuntimeError, Runtime};

pub struct JavaScriptRuntime {}

impl Runtime for JavaScriptRuntime {
    fn new() -> Self {
        JavaScriptRuntime {}
    }
    fn execute(&self, code: String) -> Result<(), super::error::RuntimeError> {
        let id = Application::create_script(&code, "js")?;
        println!("called, {}", code);
        request_elevation(Some(&format!("task run {}", id)))
            .map_err(|e| RuntimeError::CreateScriptError(e.to_string()))?;
        Ok(())
    }
    fn language(&self) -> String {
        "JavaScript".to_string()
    }
    fn create_task(&self, code: &str) -> Result<String, RuntimeError> {
        let id = Application::create_script(code, "js")?;
        Ok(id)
    }
}

#[tauri::command]
fn run_action(action_type: String, args: Data) -> Result<(), String> {
    let action = Action::get_action_instance_from_type(&action_type).map_err(|e| e.to_string())?;
    action.run(args).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_script_by_id(action_id: String) -> Result<String, String> {
    Application::get_script_by_id(action_id, "js").map_err(|e| e.to_string())
}

pub fn execute_javascript_from_tauri(id: &str) -> Result<(), RuntimeError> {
    let id = id.to_string();
    // 启动一个无窗口的tauri应用
    tauri::Builder::default()
        .setup(move |app| {
            let window: tauri::Window = tauri::window::WindowBuilder::new(app, "main1")
                .visible(false)
                .build()?;
            window.add_child(
                tauri::webview::WebviewBuilder::new(
                    "main1",
                    tauri::WebviewUrl::App(PathBuf::from(format!(
                        "js_engine/index.html?action_id={}",
                        id
                    ))),
                )
                .auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(100., 100.),
            )?;
            #[cfg(debug_assertions)] // 仅在调试(debug)版本中包含此代码
            {
                let window = app.get_webview_window("main1").unwrap();
                window.open_devtools();
                println!("3");
            }
            Ok(())
        })
        .invoke_handler(generate_handler![run_action, get_script_by_id])
        .run(generate_context!())
        .unwrap();
    Ok(())
}
