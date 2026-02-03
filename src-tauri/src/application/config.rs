use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use common::{application::Application, ty::error::TypeConvertError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaxToken {
    Default,
    Value(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiConfig {
    pub api_key: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: MaxToken,
    pub top_p: f32,
    pub frequency_penalty: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {}

impl Default for AiConfig {
    fn default() -> Self {
        AiConfig {
            api_key: "".to_string(),
            model: "".to_string(),
            temperature: 0.7,
            max_tokens: MaxToken::Default,
            top_p: 1.0,
            frequency_penalty: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub ai_config: AiConfig,
    #[serde(default)]
    pub app_config: AppConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai_config: AiConfig::default(),
            app_config: AppConfig::default(),
        }
    }
}
pub trait ConfigManager {
    fn get_config_file() -> PathBuf;
    fn get_config() -> Config;
    fn save_config(config: &Config) -> Result<(), TypeConvertError>;
}

impl ConfigManager for Application {
    fn get_config_file() -> PathBuf {
        Self::get_path("config.json")
    }

    fn get_config() -> Config {
        let config_file = Self::get_config_file();

        let result = read_to_string(config_file).unwrap();
        if result.is_empty() {
            return Config::default();
        }
        let config = serde_json::from_str(&result).unwrap();
        config
    }
    fn save_config(config: &Config) -> Result<(), TypeConvertError> {
        let config_file = Self::get_config_file();
        let config_str = serde_json::to_string(config).unwrap();
        fs::write(config_file, config_str).unwrap();
        Ok(())
    }
}
