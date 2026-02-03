use std::path::PathBuf;

use common::application::Application;
use tauri::{LogicalPosition, LogicalSize, Runtime, WebviewBuilder, WindowBuilder};

use super::config::{Config, ConfigManager};

#[tauri::command]
pub fn get_config() -> Config {
    Application::get_config()
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), String> {
    Application::save_config(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_window<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let window = WindowBuilder::new(&app, "workflow")
        .build()
        .map_err(|e| e.to_string())?;
    window
        .add_child(
            WebviewBuilder::new(
                "task",
                tauri::WebviewUrl::App(PathBuf::from("/#/task".to_string())),
            )
            .auto_resize(),
            LogicalPosition::new(0, 0),
            LogicalSize::new(800, 600),
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}
