use std::{
    fs::{exists, read_to_string, write},
    path::PathBuf,
};

use error::TriggerError;
use serde::{Deserialize, Serialize};
use time::{CronTrigger, TickerTrigger};

use common::{application::Application, ty::Data, utils::get_uid};

pub mod command;
pub mod error;
pub mod time;

pub enum TriggerType {
    Cron(CronTrigger),
    Ticker(TickerTrigger),
}

impl TriggerType {
    pub async fn setup(&self, args: Data, task_id: String) -> Result<(), TriggerError> {
        match self {
            TriggerType::Cron(trigger) => trigger.setup(args, task_id).await,
            TriggerType::Ticker(trigger) => trigger.setup(args, task_id).await,
        }
    }
    pub fn lit(&self, name: String, args: Data) -> Result<String, TriggerError> {
        match self {
            TriggerType::Cron(trigger) => trigger.lit(name, args),
            TriggerType::Ticker(trigger) => trigger.lit(name, args),
        }
    }
    pub async fn shutdown(&self) {
        match self {
            TriggerType::Cron(trigger) => trigger.shutdown().await,
            TriggerType::Ticker(trigger) => trigger.shutdown().await,
        }
    }
}

pub trait TriggerManager {
    fn get_trigger_file() -> PathBuf;
    fn get_trigger_list() -> Vec<Trigger>;
    fn update_trigger_list(trigger_list: &Vec<Trigger>) -> Result<(), TriggerError>;
    fn lit(trigger: Trigger) -> Result<(), TriggerError>;
}

impl TriggerManager for Application {
    fn get_trigger_file() -> PathBuf {
        Self::get_data_path().join("trigger.json")
    }
    /// 获取点亮的触发器卡片
    fn get_trigger_list() -> Vec<Trigger> {
        let path = Self::get_trigger_file();
        if !exists(path.clone()).unwrap() {
            write(path, "[]").unwrap();
            return vec![];
        }
        let result = read_to_string(path).unwrap();
        let result: Vec<Trigger> = serde_json::from_str(&result).unwrap();
        result
    }
    fn update_trigger_list(trigger_list: &Vec<Trigger>) -> Result<(), TriggerError> {
        let path = Self::get_trigger_file();
        let content = serde_json::to_string(&trigger_list)
            .map_err(|e| TriggerError::RegisrterTriggerError(e.to_string()))?;
        write(path, content).map_err(|e| TriggerError::RegisrterTriggerError(e.to_string()))?;
        Ok(())
    }
    /// 点亮触发器卡片
    fn lit(trigger: Trigger) -> Result<(), TriggerError> {
        let mut trigger_list = Self::get_trigger_list();
        trigger_list.push(trigger);
        Self::update_trigger_list(&trigger_list)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Trigger {
    pub id: String,
    pub label: String,
    pub r#type: String,
    pub data: Data,
}

impl Trigger {
    pub fn from_id(id: &str) -> Result<Self, TriggerError> {
        let trigger_list = Application::get_trigger_list();
        let result = trigger_list.iter().find(|trigger| trigger.id == id);
        match result {
            Some(res) => Ok(res.clone()),
            None => Err(TriggerError::FindTriggerError(id.to_string())),
        }
    }
    pub async fn setup(&self, task_id: String) -> Result<(), TriggerError> {
        let data = self.data.clone();
        let trigger = Self::find(&self.r#type);
        trigger.setup(data, task_id).await?;
        Ok(())
    }
    pub fn find(trigger_type: &str) -> TriggerType {
        let trigger: TriggerType = match trigger_type {
            "cron_trigger" => TriggerType::Cron(CronTrigger {}),
            "ticker_trigger" => TriggerType::Ticker(TickerTrigger {}),
            _ => panic!(),
        };
        trigger
    }
    pub fn remove(id: &str) -> Result<(), TriggerError> {
        let mut trigger_list = Application::get_trigger_list();
        let index = trigger_list.iter().position(|trigger| trigger.id == id);
        if index.is_none() {
            return Err(TriggerError::RemoveTriggerError(
                id.to_string(),
                "not found".to_string(),
            ));
        }
        let index = index.unwrap();
        trigger_list.remove(index);
        let content = serde_json::to_string(&trigger_list)
            .map_err(|e| TriggerError::RemoveTriggerError(id.to_string(), e.to_string()))?;
        let path = Application::get_trigger_file();
        write(path, content)
            .map_err(|e| TriggerError::RemoveTriggerError(id.to_string(), e.to_string()))?;
        Ok(())
    }
    pub async fn shutdown(&self) {
        let trigger = Self::find(&self.r#type);
        trigger.shutdown().await;
    }
}

pub trait TriggerTrait {
    fn new_trigger(&self, r#type: &str, name: String, args: Data) -> Trigger {
        let id = get_uid();
        Trigger {
            id,
            label: name,
            r#type: r#type.to_string(),
            data: args,
        }
    }
    fn setup(
        &self,
        args: Data,
        task_id: String,
    ) -> impl std::future::Future<Output = Result<(), TriggerError>> + Send;
    fn get_trigger(&self, name: String, args: Data) -> Trigger;
    fn lit(&self, name: String, args: Data) -> Result<String, TriggerError> {
        let trigger = self.get_trigger(name, args);
        Application::lit(trigger.clone())?;
        Ok(trigger.id)
    }
    fn shutdown(&self) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}
