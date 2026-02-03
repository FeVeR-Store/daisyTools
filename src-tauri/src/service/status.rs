use std::{fs, path::PathBuf};

use crate::elevation::request_elevation;
use common::{application::Application, ty::type_convert::ToString};
use log::info;
use windows_service::service::{ServiceAccess, ServiceState, ServiceStatus};

use super::{error::ServiceError, open_service};

// 定义服务状态管理器的特征
pub trait ServiceStateManager {
    // 获取服务状态文件的路径
    fn get_service_state_file() -> PathBuf;
    // 通知服务状态
    fn notify_service_state(state: ServiceState) -> Result<(), ServiceError>;
}

// 为Application实现ServiceStateManager特征
impl ServiceStateManager for Application {
    // 获取服务状态文件的路径
    fn get_service_state_file() -> PathBuf {
        Self::get_path(".service_state")
    }
    // 通知服务状态
    fn notify_service_state(state: ServiceState) -> Result<(), ServiceError> {
        let path = Self::get_service_state_file();
        info!("Service state file: {}", path.display());

        fs::write(path.clone(), state.to_string())
            .map_err(|e| ServiceError::NotifyServiceStateError(e.to_string()))?;
        info!("Service state file updated");
        Ok(())
    }
}

// 查询服务状态
pub fn query_service_status() -> Result<ServiceStatus, ServiceError> {
    // 打开服务
    let service = open_service(ServiceAccess::QUERY_STATUS)
        .map_err(|e| ServiceError::StartupServiceError(e))?;
    // 查询服务状态
    service
        .query_status()
        .map_err(|e| ServiceError::QueryServiceStatusError(e.to_string()))
}

// 查询服务状态并返回当前状态
pub fn query_service_state() -> Result<ServiceState, ServiceError> {
    let status = query_service_status()?;
    let state = status.current_state;
    Ok(state)
}

// 通知当前服务状态
pub fn notify_state() -> Result<(), ServiceError> {
    Application::notify_service_state(query_service_state()?)
}

// 启动服务的命令
#[tauri::command]
pub fn launch_service() -> Result<(), String> {
    request_elevation(Some("service launch")).map_err(|e| e.to_string())
}

// 获取服务状态的命令
#[tauri::command]
pub async fn get_service_state() -> Result<String, String> {
    let state = query_service_state().map_err(|e| e.to_string())?;
    let state = state.to_string();
    Ok(state.to_string())
}

// 获取服务状态文件的命令
#[tauri::command]
pub fn get_service_state_file() -> String {
    Application::get_service_state_file().to_string()
}
