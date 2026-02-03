pub mod action;
pub mod error;
pub mod service_main;
pub mod status;
pub mod task;
pub mod trigger;

use std::{
    ffi::{OsStr, OsString},
    sync::{atomic::AtomicBool, mpsc, Arc},
    time::{Duration, Instant},
};

use error::ServiceError;
use status::notify_state;
use windows_service::{
    define_windows_service,
    service::{
        Service, ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl,
        ServiceExitCode, ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
    service_dispatcher,
    service_manager::{ServiceManager, ServiceManagerAccess},
};

use windows_service::Error as WinSvcError;

const SERVICE_NAME: &str = "DaisyToolsService";
const SERVICE_DISPLAY_NAME: &str = "DaisyTools Service";
const SERVICE_DESCRIPTION: &str = "DaisyTools 后台服务";

pub fn start_service() -> Result<(), ServiceError> {
    log::info!("start service");
    service_dispatcher::start(SERVICE_NAME, ffi_service_entry)
        .map_err(|e| ServiceError::StartupServiceError(e))
}

#[allow(dead_code)]
enum ServiceMessage {
    Timer(Instant),
    Custom(String),
    Shutdown,
}

fn open_service(access: ServiceAccess) -> Result<Service, windows_service::Error> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;
    manager.open_service(SERVICE_NAME, access)
}

define_windows_service!(ffi_service_entry, service_entry);

pub fn launch_service() -> Result<(), ServiceError> {
    let service =
        open_service(ServiceAccess::START).map_err(|e| ServiceError::StartupServiceError(e))?;
    let args: &[&OsStr; 0] = &[];
    service
        .start(args)
        .map_err(|e| ServiceError::StartupServiceError(e))
}

fn service_entry(_arg: Vec<OsString>) {
    log::info!("service entry");
    if let Err(e) = run_service() {
        log::error!("Service error: {}", e);
    }
}

fn run_service() -> Result<(), ServiceError> {
    log::info!("run service");
    let (message_tx, _message_rx) = mpsc::channel();
    let _running = Arc::new(AtomicBool::new(true));

    // 创建服务状态句柄
    let status_handle = service_control_handler::register(SERVICE_NAME, {
        let message_tx = message_tx.clone();
        move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Stop => {
                    message_tx.send(ServiceMessage::Shutdown).unwrap();
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        }
    })
    .map_err(|e| ServiceError::StartupServiceError(e))?;
    // 设置服务状态为运行中
    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    notify_state()?;
    log::info!("notify state");
    service_main::main();

    // 设置服务状态为已停止
    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    notify_state()?;
    log::debug!("server stop");
    Ok(())
}

pub fn install_service() -> Result<(), ServiceError> {
    // 打开服务管理器
    let manager =
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)
            .map_err(|e| {
                ServiceError::InstallServiceError(format!("Manager creation Failed: {}", e))
            })?;

    // 检查是否存在同名服务
    if manager
        .open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS)
        .is_ok()
    {
        return Err(ServiceError::InstallServiceError(format!(
            "Service '{}' already exists",
            SERVICE_NAME
        )));
    }

    // 获取当前可执行路径
    let service_binary_path = std::env::current_exe().map_err(|e| {
        ServiceError::InstallServiceError(format!("Failed to get executable path: {}", e))
    })?;
    println!("Service binary path: {:?}", service_binary_path);

    // 构建服务信息
    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec!["service".into()],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    // 创建服务
    let service = manager
        .create_service(&service_info, ServiceAccess::START)
        .map_err(|e| map_error_with_code(e, "Failed to create service"))?;

    // 设置服务描述
    service
        .set_description(SERVICE_DESCRIPTION)
        .map_err(|e| map_error_with_code(e, "Failed to set service description"))?;

    Ok(())
}

fn map_error_with_code(e: WinSvcError, context: &str) -> ServiceError {
    match e {
        WinSvcError::Winapi(err_code) => {
            let error_message = format!("{}: WinAPI error code: {}", context, err_code);
            ServiceError::InstallServiceError(error_message)
        }
        _ => ServiceError::InstallServiceError(format!("{}: Unknown error", context)),
    }
}

pub fn unintall_service() -> Result<(), ServiceError> {
    // 获取活动的服务数据库
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .map_err(|e| ServiceError::UninstallServiceError(e.to_string()))?;

    // 获取服务实例
    let service = manager
        .open_service(SERVICE_NAME, ServiceAccess::DELETE)
        .map_err(|e| ServiceError::UninstallServiceError(e.to_string()))?;

    // 删除服务
    service
        .delete()
        .map_err(|e| ServiceError::UninstallServiceError(e.to_string()))?;
    Ok(())
}
