#[macro_export]
macro_rules! this_file {
    () => {
        std::path::PathBuf::from(file!())
    };
}

use std::path::PathBuf;

pub fn get_current_binary() -> PathBuf {
    std::env::current_exe().unwrap()
}


pub fn get_uid() -> String {
    (rand::random_range(0.0..1e9) as u64).to_string()
}
pub fn to_upper_camel_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty()) // 过滤空字符串，避免连续下划线影响
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    std::string::ToString::to_string(&first.to_ascii_uppercase()) + chars.as_str()
                }
                None => String::new(),
            }
        })
        .collect::<String>()
}
