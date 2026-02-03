use std::{
    fs::{create_dir_all, exists, write},
    path::PathBuf,
};
const APP_NAME: &str = "daisyTools";
pub struct Application;

impl Application {
    pub fn get_data_path() -> PathBuf {
        Self::get_path("")
    }
    pub fn get_path(path: &str) -> PathBuf {
        let is_dir = path.ends_with(".d") || !path.contains(".");
        let path = PathBuf::from("C:\\ProgramData").join(APP_NAME).join(path);
        println!("{}", path.display());
        if !exists(&path).unwrap() {
            if is_dir {
                create_dir_all(&path).unwrap();
            } else {
                write(&path, "").unwrap();
            }
        }
        path
    }
}
