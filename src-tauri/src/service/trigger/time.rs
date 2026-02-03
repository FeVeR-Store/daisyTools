use std::{str::FromStr, time::Duration};

use chrono::Local;
use cron::Schedule;
use crossbeam_channel::tick;
use log::info;
use common::tokio::time::sleep;

use crate::service::task::Task;

use super::{error::TriggerError, Trigger, TriggerTrait};
use common::ty::Data;

pub struct CronTrigger {}

impl TriggerTrait for CronTrigger {
    fn get_trigger(&self, name: String, args: Data) -> Trigger {
        self.new_trigger("cron_trigger", name, args)
    }
    async fn setup(&self, args: Data, task_id: String) -> Result<(), super::error::TriggerError> {
        match args.as_string() {
            Ok(expression) => cron_trigger(&expression, task_id).await,
            Err(e) => Err(TriggerError::SetupTriggerError(e.to_string())),
        }
    }
}

pub async fn cron_trigger(expression: &str, task_id: String) -> Result<(), TriggerError> {
    let cron = Schedule::from_str(expression).unwrap();
    info!("cron trigger setup");
    loop {
        // 检查是否到达执行时间
        if cron.includes(Local::now()) {
            info!("cron trigger activate");
            let res = Task::init_task_instance(task_id.clone());
            match res {
                Ok(task_instance_list) => {
                    info!("task number: {}", task_instance_list.len());
                    for task_instance in task_instance_list {
                        task_instance.run().map_err(|e| {
                            TriggerError::RunTaskError(task_id.clone(), e.to_string())
                        })?;
                    }
                }
                Err(e) => log::error!("cron trigger error: {}", e),
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}

#[tauri::command]
pub fn is_cron_expression_vaild(expression: String) -> bool {
    Schedule::from_str(&expression).is_ok()
}

pub struct TickerTrigger {}

impl TriggerTrait for TickerTrigger {
    fn get_trigger(&self, name: String, args: Data) -> Trigger {
        self.new_trigger("ticker_trigger", name, args)
    }
    async fn setup(&self, args: Data, task_id: String) -> Result<(), super::error::TriggerError> {
        match args.as_int() {
            Ok(duration) => ticker_trigger(duration.abs().try_into().unwrap(), task_id),
            Err(e) => Err(TriggerError::SetupTriggerError(e.to_string())),
        }
    }
}

pub fn ticker_trigger(duration: u64, task_id: String) -> Result<(), TriggerError> {
    let ticker = tick(Duration::from_millis(duration));
    loop {
        ticker.recv().unwrap();
        match Task::init_task_instance(task_id.clone()) {
            Ok(task_instance_list) => {
                for task_instance in task_instance_list {
                    task_instance
                        .run()
                        .map_err(|e| TriggerError::RunTaskError(task_id.clone(), e.to_string()))?;
                }
            }
            Err(e) => log::error!("ticker trigger error: {}", e),
        }
    }
}
