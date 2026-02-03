use std::time::Duration;

use common::application::Application;
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use tauri::async_runtime::block_on;

// use crate::ipc::service::{setup_tcp_server, TcpServer};

use super::task::{scheduler::setup_task, TaskManager};

pub fn main() {
    log::info!("start setup tasks");
    // 启动所有的任务
    let mut scheduler = match setup_task() {
        Ok(scheduler) => scheduler,
        Err(e) => {
            log::error!("Failed to setup tasks: {}", e);
            return;
        }
    };
    log::info!("Scheduler setup successfully");

    log::info!("start setup tcp server");

    // setup_tcp_server();
    // log::info!("TCP server setup successfully");
    // 启动任务列表监听器
    let path = Application::get_task_file();
    let (tx, rx) = std::sync::mpsc::channel();

    // no specific tickrate, max debounce time 2 seconds
    let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx).unwrap();
    log::info!("Debouncer created successfully");

    debouncer
        .watch(path.clone(), RecursiveMode::Recursive)
        .unwrap();
    log::info!("Watching path: {:?}", path);

    for result in rx {
        match result {
            Ok(_) => {
                block_on(async {
                    log::info!("Task file changed, restarting tasks");
                    scheduler.shutdown().await;
                    log::info!("Scheduler shutdown successfully")
                });
                log::info!("Scheduler restarting");
                scheduler = match setup_task() {
                    Ok(scheduler) => scheduler,
                    Err(e) => {
                        log::error!("Failed to setup tasks: {}", e);
                        return;
                    }
                };
                log::info!("Tasks re-setup successfully");
            }
            Err(e) => {
                log::error!("Error in task setup: {:?}", e);
            }
        }
    }
}
