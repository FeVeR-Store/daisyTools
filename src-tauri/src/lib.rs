use application::command::{get_config, open_window, save_config};
// use pipe::client::communicate_with_service;
use service::{
    action::command::{
        get_lit_action, register_action, remove_action, run_action_by_id, update_action_plug,
    },
    status::{get_service_state, get_service_state_file, launch_service},
    task::create_task,
    trigger::{
        command::{get_lit_trigger, register_trigger, remove_trigger},
        time::is_cron_expression_vaild,
    },
};
use tauri::{webview, LogicalPosition, LogicalSize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod application;
pub mod elevation;
// pub mod ipc;
pub mod runtime;
pub mod service;
pub mod utils;
// #[tauri::command]
// fn call_service() {
//     communicate_with_service().unwrap();
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let width = 800.;
            let height = 600.;
            let window = tauri::window::WindowBuilder::new(app, "main")
                .inner_size(width, height)
                .decorations(false)
                .build()?;
            window.add_child(
                webview::WebviewBuilder::new("main", tauri::WebviewUrl::App(Default::default()))
                    .auto_resize(),
                LogicalPosition::new(0., 0.),
                LogicalSize::new(width, height),
            )?;
            Ok(())
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            register_action,
            run_action_by_id,
            get_lit_trigger,
            register_trigger,
            get_lit_action,
            get_service_state,
            launch_service,
            get_service_state_file,
            create_task,
            remove_action,
            remove_trigger,
            is_cron_expression_vaild,
            update_action_plug,
            save_config,
            get_config,
            open_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
