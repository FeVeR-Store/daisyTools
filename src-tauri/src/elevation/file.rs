use log::info;
use windows::Win32::Security::Authorization::{
    BuildExplicitAccessWithNameW, GetNamedSecurityInfoW, SetEntriesInAclW, SetNamedSecurityInfoW,
    EXPLICIT_ACCESS_W, SET_ACCESS, SE_FILE_OBJECT,
};
use windows::{
    core::PCWSTR, Win32::Foundation::*, Win32::Security::*, Win32::Storage::FileSystem::*,
};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

fn to_pcwstr(s: &str) -> PCWSTR {
    let wide: Vec<u16> = OsStr::new(s).encode_wide().chain(Some(0)).collect();
    PCWSTR(wide.as_ptr())
}

pub fn grant_read_access_to_users(file_path: &str) -> windows::core::Result<()> {
    unsafe {
        let pw = to_pcwstr(file_path);

        let mut p_old_dacl: *mut ACL = std::ptr::null_mut();
        let mut p_sacl: *mut ACL = std::ptr::null_mut();
        let mut p_sd: PSECURITY_DESCRIPTOR = PSECURITY_DESCRIPTOR(null_mut() as *mut _);
        GetNamedSecurityInfoW(
            to_pcwstr(file_path),      // 文件路径
            SE_FILE_OBJECT,            // 对象类型是文件
            DACL_SECURITY_INFORMATION, // 只获取 DACL
            None,                      // 不获取所有者
            None,                      // 不获取用户组
            Some(&mut p_old_dacl),     // 输出指针 to old DACL
            Some(&mut p_sacl),         // 输出指向 SACL，虽然不需要但必须传入
            &mut p_sd as *mut _,       // 输出 Security Descriptor
        )
        .ok()?;

        // Step 1: Build access entry for "Users" group
        let mut ea = EXPLICIT_ACCESS_W::default();

        let trustee_name = to_pcwstr("Users");

        BuildExplicitAccessWithNameW(
            &mut ea,
            trustee_name,
            FILE_GENERIC_READ.0,
            SET_ACCESS,
            SUB_CONTAINERS_AND_OBJECTS_INHERIT, // SUB_OBJECTS_AND_SELF_INHERIT,
        );

        // Step 2: Create new ACL
        info!("Create ACL");

        let new_acl: *mut ACL = null_mut();

        // Step 3: 合并旧 ACL 和新的 ACE
        let mut p_new_dacl: *mut ACL = null_mut();
        SetEntriesInAclW(Some(&[ea]), Some(p_old_dacl), &mut p_new_dacl).ok()?;

        // Step 3: Apply new ACL to file
        info!("Apply ACL to file {}", file_path);
        SetNamedSecurityInfoW(
            pw,
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            None,
            None,
            Some(new_acl),
            None,
        )
        .ok()?;

        info!("Free ACL memory");
        // Step 4: Free ACL memory
        let hmem: Option<HLOCAL> = Some(HLOCAL(new_acl as _)); // new_acl 是 *mut ACL 转成 HLOCAL
        let result: HLOCAL = LocalFree(hmem);
        if !result.is_invalid() {
            return Err(windows::core::Error::from_win32());
        }
    }

    Ok(())
}
