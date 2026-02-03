pub mod error;
pub mod file;

use common::{ty::type_convert::ToString, utils::get_current_binary};
use error::ElevationError;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE},
        Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        System::{
            Com::{CoInitializeEx, COINIT_APARTMENTTHREADED},
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
        UI::{
            Shell::ShellExecuteW,
            WindowsAndMessaging::{HWND_DESKTOP, SW_HIDE},
        },
    },
};

use crate::utils::windows::get_error;

pub fn request_elevation(args: Option<&str>) -> Result<(), ElevationError> {
    unsafe {
        // 初始化 COM
        if CoInitializeEx(None, COINIT_APARTMENTTHREADED).is_err() {
            return Err(ElevationError::RequestElevationError(get_error()));
        };

        // 转换参数为宽字符串
        let operation = windows::core::w!("runas");

        let program = get_current_binary().to_string();
        let file = windows::core::HSTRING::from(program);

        let parameters = args.map(|s| windows::core::HSTRING::from(s));

        // 执行 ShellExecute
        let result: windows::Win32::Foundation::HINSTANCE = ShellExecuteW(
            Some(HWND_DESKTOP),
            PCWSTR(operation.as_ptr()),
            PCWSTR(file.as_ptr()),
            parameters
                .as_ref()
                .map_or(PCWSTR::null(), |s| PCWSTR(s.as_ptr())),
            PCWSTR::null(),
            SW_HIDE,
        );
        // 检查结果

        let result_value = result.0 as isize;
        if result_value <= 32 {
            let msg = match result_value {
                0 => "The operating system is out of memory or resources.",
                2 => "The specified file was not found.",
                3 => "The specified path was not found.",
                5 => "Access denied.",
                27 => "Association incomplete.",
                31 => "No application is associated with the specified file.",
                _ => &format!("ShellExecuteW failed with error code: {}", result_value),
            };
            return Err(ElevationError::RequestElevationError(msg.to_string()));
        }

        Ok(())
    }
}
pub fn is_elevation() -> Result<bool, ElevationError> {
    unsafe {
        // 获取当前进程句柄
        let process_handle = GetCurrentProcess();

        // 打开进程令牌
        let mut token_handle: HANDLE = HANDLE::default();
        if OpenProcessToken(process_handle, TOKEN_QUERY, &mut token_handle).is_err() {
            return Err(ElevationError::CheckElevationError(get_error()));
        }

        // 查询令牌的提升信息
        let mut elevation = TOKEN_ELEVATION::default();
        let mut return_length = 0u32;
        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );

        // 关闭令牌句柄
        CloseHandle(token_handle).map_err(|e| ElevationError::CheckElevationError(e.message()))?;

        // 检查是否成功获取令牌信息
        if result.is_err() {
            return Err(ElevationError::CheckElevationError(get_error()));
        }

        // 判断是否具有管理员权限
        Ok(elevation.TokenIsElevated != 0)
    }
}
