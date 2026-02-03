use std::{
    collections::HashMap,
    fs::{exists, read_to_string, write},
    path::PathBuf,
};

use aster_loader::ActionProvider;
use common::{
    action::{entry::ActionEntry, error::ActionError, Action},
    application::Application,
    ty::{CardResult, Data},
    utils::get_uid,
};
use error::TaskError;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::trigger::Trigger;
pub mod error;
pub mod scheduler;

pub trait TaskManager {
    fn get_task_file() -> PathBuf;
    fn get_task_list() -> Result<Vec<Task>, TaskError>;
    fn update_task_list(task_list: &Vec<Task>) -> Result<(), TaskError>;
    fn add_task(
        task_info: TaskInfo,
        workflow: HashMap<String, ActionEntry>,
    ) -> Result<String, TaskError>;
}

pub trait SetupManager {
    fn get_setup_file() -> PathBuf;
    fn get_setup_list() -> Result<Vec<Setup>, TaskError>;
    fn update_setup_list(setup_list: &Vec<Setup>) -> Result<(), TaskError>;
    fn add_setup(setup: Setup) -> Result<(), TaskError>;
}

impl TaskManager for Application {
    fn get_task_file() -> PathBuf {
        Self::get_path("task.json")
    }
    fn get_task_list() -> Result<Vec<Task>, TaskError> {
        let path = Self::get_task_file();
        debug!("task file: {:?}", &path);
        if !exists(&path).unwrap() {
            write(path.clone(), "[]").unwrap();
            return Ok(vec![]);
        }

        let result = read_to_string(path.clone())
            .map_err(|e| TaskError::ReadTaskFileError(path.clone(), e.to_string()))?;

        let result: Vec<Task> = serde_json::from_str(&result)
            .map_err(move |e| TaskError::ParseTaskFileError(path.clone(), e.to_string()))?;
        Ok(result)
    }
    fn update_task_list(task_list: &Vec<Task>) -> Result<(), TaskError> {
        let path = Self::get_task_file();
        let content = serde_json::to_string(&task_list)
            .map_err(|e| TaskError::UpdateTaskListError(e.to_string()))?;
        write(path, content).map_err(|e| TaskError::UpdateTaskListError(e.to_string()))?;
        Ok(())
    }

    fn add_task(
        task_info: TaskInfo,
        workflow: HashMap<String, ActionEntry>,
    ) -> Result<String, TaskError> {
        let mut task_list = Self::get_task_list()?;
        let task_id = get_uid();

        log::info!(
            "Creating new task with name: {}, id: {}",
            &task_info.name,
            &task_id
        );

        task_list.push(Task {
            id: task_id.clone(),
            info: task_info.clone(),
            workflow,
        });

        Self::update_task_list(&task_list)?;
        log::info!("Task list updated with new task: {}", task_info.name);
        Ok(task_id)
    }
}

impl SetupManager for Application {
    fn get_setup_file() -> PathBuf {
        Self::get_path("setup.json")
    }

    fn get_setup_list() -> Result<Vec<Setup>, TaskError> {
        let path = Self::get_setup_file();
        debug!("setup file: {:?}", &path);
        if !exists(&path).unwrap() {
            write(path.clone(), "[]").unwrap();
            return Ok(vec![]);
        }

        let result = read_to_string(path.clone())
            .map_err(|e| TaskError::ReadTaskFileError(path.clone(), e.to_string()))?;

        let result: Vec<Setup> = serde_json::from_str(&result)
            .map_err(move |e| TaskError::ParseTaskFileError(path.clone(), e.to_string()))?;
        Ok(result)
    }

    fn update_setup_list(setup_list: &Vec<Setup>) -> Result<(), TaskError> {
        let path = Self::get_setup_file();
        let content = serde_json::to_string(&setup_list)
            .map_err(|e| TaskError::UpdateTaskListError(e.to_string()))?;
        write(path, content).map_err(|e| TaskError::UpdateTaskListError(e.to_string()))?;
        Ok(())
    }

    fn add_setup(setup: Setup) -> Result<(), TaskError> {
        let mut setup_list = Self::get_setup_list()?;
        setup_list.push(setup);
        Self::update_setup_list(&setup_list)?;
        Ok(())
    }
}

#[tauri::command]
pub fn create_task(
    task_info: TaskInfo,
    workflow: HashMap<String, ActionEntry>,
) -> Result<String, String> {
    let id = Application::add_task(task_info, workflow).map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub fn remove_task(task_id: String) -> Result<(), String> {
    let mut task_list = Application::get_task_list().map_err(|e| e.to_string())?;
    task_list.retain(|t| &t.id != &task_id);
    Application::update_task_list(&task_list).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskInfo {
    pub tag: Vec<String>,
    pub name: String,
    pub setup: Setup,
    pub trigger: Vec<String>,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Setup {
    pub trigger: String,
    pub task: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub info: TaskInfo,
    pub workflow: HashMap<String, ActionEntry>,
}

pub struct TaskInstance {
    id: String,
    name: String,
    context: HashMap<String, Data>,
    workflow: HashMap<String, Action>,
}

impl TaskInstance {
    pub fn run(mut self) -> Result<(), TaskError> {
        info!(
            "Run workflow {{{}}}({}): {:?}",
            &self.name, &self.id, &self.workflow
        );

        let mut current = self.workflow.get("trigger");

        while let Some(action) = current {
            // 执行action，传入context
            let CardResult { variant, data } = match action.run(&self.context) {
                Ok(data) => data,
                Err(e) => {
                    let ActionError::RunActionCardError(e) = e else {
                        log::error!("unknow error {}", &e);
                        break;
                    };
                    self.context
                        .insert(action.id.clone(), Data::Any(json!({ "\0error": e })));
                    break;
                }
            };

            info!("Run action successfully, result: {}", &data);

            info!("Insert context");
            // 将结果存入context
            self.context.insert(action.id.clone(), data);

            debug!("workflow context: {:?}", &self.context);

            current = self.workflow.get(&format!("{}:{}", &self.id, variant))
        }

        Ok(())
    }
}

impl Task {
    pub fn init_task_instance(task_id: String) -> Result<Vec<TaskInstance>, TaskError> {
        let task = Self::find_from_id(&task_id);
        match task {
            Some(task) => {
                if task.info.enabled {
                    let workflow = Action::create_workflow(&task.workflow).unwrap();
                    let task_instance = TaskInstance {
                        id: task_id.clone(),
                        name: task.info.name.clone(),
                        context: HashMap::new(),
                        workflow,
                    };
                    Ok(vec![task_instance])
                } else {
                    Ok(vec![])
                }
            }
            None => Err(TaskError::TaskNotFoundError(task_id.clone())),
        }
    }
    pub async fn setup(&self) -> Result<(), TaskError> {
        let task_id = self.id.clone();

        // 为每个trigger设置任务
        for trigger_id in &self.info.trigger {
            let trigger = Trigger::from_id(trigger_id)
                .map_err(|e| TaskError::SetupTaskError(self.id.clone(), e.to_string()))?;
            trigger
                .setup(task_id.clone())
                .await
                .map_err(|e| TaskError::SetupTaskError(self.id.clone(), e.to_string()))?;
        }

        Ok(())
    }
    pub fn find_from_trigger(id: &str) -> Option<Task> {
        let task_list = Application::get_task_list().ok()?;
        let task = task_list
            .iter()
            .find(|task| task.info.trigger.contains(&id.to_string()));
        task.cloned()
    }
    pub fn find_from_id(id: &str) -> Option<Task> {
        let task_list = Application::get_task_list().ok()?;
        task_list.iter().find(|task| task.id == id).cloned()
    }
}
