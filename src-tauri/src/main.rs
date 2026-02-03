// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env::set_var,
    fs::{create_dir, exists},
};

use clap::Parser;
use daisytools_lib::{
    elevation,
    runtime::javascript::execute_javascript_from_tauri,
    service::{
        install_service, launch_service, start_service, status::query_service_status,
        unintall_service,
    },
};
use ftail::Ftail;
use log::LevelFilter;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Service {
        #[command(subcommand)]
        action: Option<ServiceCommand>,
    },
    Task {
        #[command(subcommand)]
        action: TaskCommand,
    },
}

#[derive(Debug, clap::Subcommand)]
enum ServiceCommand {
    Launch,
    Install,
    Uninstall,
}

#[derive(Debug, clap::Subcommand)]
enum TaskCommand {
    Run { code: String },
}

fn main() {
    #[cfg(debug_assertions)]
    set_var("RUST_BACKTRACE", "1");
    let cli = Args::parse();
    if !exists("logs").unwrap() {
        create_dir("logs").unwrap()
    }
    let Some(cmd) = cli.command else {
        // 如果可以查询到状态，说明已安装服务
        // 如果没有安装服务，则判断是否授予管理员权限
        if query_service_status().is_ok() || privilege::user::privileged() {
            Ftail::new()
                .single_file("logs/client.log", true, LevelFilter::Off)
                .init()
                .unwrap();
            // 启动UI
            return daisytools_lib::run();
        };
        // 若未安装服务且没有权限，则请求权限
        elevation::request_elevation(None).unwrap();
        return;
    };
    // cli
    Ftail::new()
        .single_file(
            "D:\\codes\\daisyTools\\src-tauri\\logs\\service.log",
            true,
            LevelFilter::Off,
        )
        .init()
        .unwrap();
    let res = match cmd {
        // 服务相关
        Command::Service {
            action: Some(action),
        } => {
            if privilege::user::privileged() {
                // 下面的内容需要管理员权限
                match action {
                    ServiceCommand::Launch => launch_service(),
                    ServiceCommand::Install => install_service(),
                    ServiceCommand::Uninstall => unintall_service(),
                }
            } else {
                let arg = match action {
                    ServiceCommand::Launch => "launch",
                    ServiceCommand::Install => "install",
                    ServiceCommand::Uninstall => "unintall",
                };
                // 若没有权限，则请求权限
                elevation::request_elevation(Some(&("service ".to_string() + arg))).unwrap();
                return;
            }
        }
        Command::Service { action: None } => {
            log::info!("start service");
            start_service()
        }
        Command::Task { action } => match action {
            TaskCommand::Run { code: action_id } => {
                execute_javascript_from_tauri(&action_id).unwrap();
                return;
            }
        },
    };
    match res {
        Err(e) => panic!("{} ", e),
        Ok(()) => (),
    }
}
