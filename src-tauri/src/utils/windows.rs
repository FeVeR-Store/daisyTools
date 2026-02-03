use std::io::{self};
use windows::Win32::Security::{
    InitializeSecurityDescriptor, SetSecurityDescriptorDacl, PSECURITY_DESCRIPTOR,
    SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
};
use windows::Win32::Storage::FileSystem::{
    CreateFileA, ReadFile, WriteFile, FILE_FLAGS_AND_ATTRIBUTES, FILE_FLAG_WRITE_THROUGH,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING, PIPE_ACCESS_DUPLEX,
};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeA, DisconnectNamedPipe, PIPE_READMODE_MESSAGE,
    PIPE_TYPE_MESSAGE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};
use windows::{
    core::PCSTR,
    Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE, INVALID_HANDLE_VALUE},
};

/// 命名管道
pub struct NamedPipeServer {
    handle: HANDLE,
    pub pipe_name: String,
}

impl NamedPipeServer {
    pub fn new(name: &str) -> io::Result<Self> {
        // 确保管道名称格式正确
        let pipe_name = if !name.starts_with("\\\\.\\pipe\\") {
            format!("\\\\.\\pipe\\{}", name)
        } else {
            name.to_string()
        };

        let sd = SECURITY_DESCRIPTOR::default();

        unsafe {
            let psd = PSECURITY_DESCRIPTOR(&sd as *const _ as *mut _);
            InitializeSecurityDescriptor(psd, 1)?;
            SetSecurityDescriptorDacl(psd, true, None, false)?;
        }

        let sa = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: &sd as *const _ as *mut _,
            bInheritHandle: false.into(),
        };

        // 创建命名管道
        let handle = unsafe {
            CreateNamedPipeA(
                PCSTR(pipe_name.as_ptr()),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                4096,
                4096,
                FILE_FLAG_WRITE_THROUGH.0,
                Some(&sa as *const _),
            )?
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { handle, pipe_name })
    }

    pub fn wait_for_connection(&self) -> io::Result<()> {
        let result = unsafe { ConnectNamedPipe(self.handle, None) };
        if !result.is_ok() {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0;
        let result = unsafe { WriteFile(self.handle, Some(data), Some(&mut bytes_written), None) };
        if result.is_err() {
            return Err(io::Error::last_os_error());
        }
        Ok(bytes_written as usize)
    }

    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;
        let result = unsafe { ReadFile(self.handle, Some(buffer), Some(&mut bytes_read), None) };

        if !result.is_err() {
            return Err(io::Error::last_os_error());
        }
        Ok(bytes_read as usize)
    }

    pub fn disconnect(&self) -> io::Result<()> {
        let result = unsafe { DisconnectNamedPipe(self.handle) };
        if !result.is_err() {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for NamedPipeServer {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle).unwrap() };
    }
}

pub struct NamedPipeClient {
    handle: HANDLE,
}

impl NamedPipeClient {
    pub fn connect(name: &str) -> io::Result<Self> {
        let pipe_name = if !name.starts_with("\\\\.\\pipe\\") {
            format!("\\\\.\\pipe\\{}", name)
        } else {
            name.to_string()
        };

        let handle = unsafe {
            CreateFileA(
                PCSTR(pipe_name.as_ptr()),
                GENERIC_READ.0 | GENERIC_WRITE.0, // dwDesiredAccess
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_FLAGS_AND_ATTRIBUTES(0), // dwFlagsAndAttributes
                None,
            )
        };

        let Ok(handle) = handle else {
            return Err(io::Error::last_os_error());
        };

        Ok(Self { handle })
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        let mut bytes_written = 0;
        let result = unsafe { WriteFile(self.handle, Some(data), Some(&mut bytes_written), None) };

        if result.is_err() {
            return Err(io::Error::last_os_error());
        }
        Ok(bytes_written as usize)
    }

    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut bytes_read = 0;
        let result = unsafe { ReadFile(self.handle, Some(buffer), Some(&mut bytes_read), None) };

        if result.is_err() {
            return Err(io::Error::last_os_error());
        }
        Ok(bytes_read as usize)
    }
}

impl Drop for NamedPipeClient {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle).unwrap() };
    }
}
/// 捕获来自windows的错误信息
pub fn get_error() -> String {
    windows::core::Error::from_win32().to_string()
}
