use common::application::Application;
use common::tokio::runtime::{Builder as RuntimeBuilder, Runtime};
use common::tokio::task::JoinHandle;
use log::debug;
use num_cpus;
use std::time::Duration;

use crate::service::trigger::Trigger;

use super::{Setup, SetupManager, Task, TaskManager};

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("Runtime creation failed: {0}")]
    RuntimeError(String),

    #[error("Task setup failed: {0}")]
    TaskSetupError(String),
}

pub struct TaskScheduler {
    runtime: Runtime,
    worker_count: usize,
    task_triggers: Vec<String>,
}

impl TaskScheduler {
    pub fn new() -> Result<Self, SchedulerError> {
        // 获取CPU核心数，留出一个核心给其他系统进程
        let worker_count = std::cmp::max(1, num_cpus::get() - 1);

        // 创建多线程运行时
        let runtime = RuntimeBuilder::new_multi_thread()
            .worker_threads(worker_count)
            .enable_all()
            .build()
            .map_err(|e| SchedulerError::RuntimeError(e.to_string()))?;

        Ok(Self {
            runtime: runtime,
            worker_count,
            task_triggers: vec![],
        })
    }

    pub fn setup_tasks(&mut self, setup_list: Vec<Setup>) -> Result<(), SchedulerError> {
        log::info!(
            "Starting task scheduler with {} worker threads",
            self.worker_count
        );

        // 收集所有需要加载的任务ID
        let mut all_task_ids = Vec::new();
        for setup in &setup_list {
            self.task_triggers.push(setup.trigger.clone());
            all_task_ids.extend(setup.task.clone());
        }

        // 从task.json加载所有需要的任务
        let all_tasks = Application::get_task_list()
            .map_err(|e| SchedulerError::TaskSetupError(e.to_string()))?;

        let mut tasks_to_setup = Vec::new();
        for task_id in all_task_ids {
            if let Some(task) = all_tasks.iter().find(|t| t.id == task_id) {
                tasks_to_setup.push(task.clone());
            } else {
                log::warn!("Task with ID {} not found in task.json", task_id);
            }
        }

        // 将任务分组，每个工作线程处理一组任务
        let tasks_per_worker = (tasks_to_setup.len() + self.worker_count - 1) / self.worker_count;
        let task_groups: Vec<Vec<Task>> =
            tasks_to_setup
                .into_iter()
                .fold(Vec::new(), |mut acc, task| {
                    if acc.is_empty() || acc.last().unwrap().len() >= tasks_per_worker {
                        acc.push(vec![task]);
                    } else {
                        acc.last_mut().unwrap().push(task);
                    }
                    acc
                });

        // 为每组任务创建一个异步任务
        let handles: Vec<JoinHandle<Result<(), SchedulerError>>> = task_groups
            .into_iter()
            .enumerate()
            .map(|(worker_id, tasks)| {
                let runtime = &self.runtime;
                runtime.spawn(async move {
                    log::info!("Worker {} starting with {} tasks", worker_id, tasks.len());
                    for task in tasks {
                        if let Err(e) = task.setup().await {
                            log::error!("Task setup failed on worker {}: {}", worker_id, e);
                            return Err(SchedulerError::TaskSetupError(e.to_string()));
                        }
                    }
                    Ok(())
                })
            })
            .collect();

        // 等待所有任务组完成初始化
        self.runtime.block_on(async {
            for handle in handles {
                match handle.await {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => {
                        log::error!("Worker returned error: {}", e);
                        return Err(e);
                    }
                    Err(e) => {
                        log::error!("Worker thread panicked: {}", e);
                        return Err(SchedulerError::RuntimeError(e.to_string()));
                    }
                }
            }
            Ok::<(), SchedulerError>(())
        })?;

        Ok(())
    }
    pub async fn shutdown(self) {
        for trigger in self.task_triggers {
            match Trigger::from_id(&trigger) {
                Ok(trigger) => {
                    log::info!("Shutdown trigger: {}", &trigger.r#type);
                    trigger.shutdown().await;
                }
                Err(e) => {
                    log::warn!("Trigger not found for task {}: {}", &trigger, e);
                }
            }
        }
        self.runtime.shutdown_timeout(Duration::from_secs(3));
    }
}

// 修改应用程序的任务设置函数
pub fn setup_task() -> Result<TaskScheduler, SchedulerError> {
    debug!("try to get setup list");
    // 获取启动项列表
    let setup_list =
        Application::get_setup_list().map_err(|e| SchedulerError::TaskSetupError(e.to_string()))?;

    debug!("setup list: {:?}", &setup_list);

    // 创建调度器
    let mut scheduler = TaskScheduler::new()?;

    // 启动所有任务
    scheduler.setup_tasks(setup_list)?;

    Ok(scheduler)
}
