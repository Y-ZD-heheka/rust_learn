use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

use super::model::{Priority, Status, Task};
use super::stats::TaskStatistics;
use super::storage::{TaskLoadOutcome, TaskStorage, TaskStorageConfig};

/// 任务管理器初始化时的数据来源状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskManagerLoadState {
    InitializedEmpty,
    LoadedFromStorage,
}

/// 受控的任务更新句柄。
pub struct TaskUpdateHandle<'a> {
    manager: &'a mut TaskManager,
    id: u64,
}

impl<'a> TaskUpdateHandle<'a> {
    /// 将任务标记为进行中。
    pub fn start(&mut self) -> Result<()> {
        self.manager.start_task(self.id)
    }

    /// 将任务标记为已完成。
    pub fn complete(&mut self) -> Result<()> {
        self.manager.complete_task(self.id)
    }

    /// 将任务标记为已取消。
    pub fn cancel(&mut self) -> Result<()> {
        self.manager.cancel_task(self.id)
    }
}

/// 任务存储管理器
pub struct TaskManager {
    tasks: HashMap<u64, Task>,
    next_id: u64,
    storage: TaskStorage,
    load_state: TaskManagerLoadState,
}

impl TaskManager {
    /// 使用默认存储配置创建新的任务管理器。
    pub fn new() -> Result<Self> {
        Self::with_config(TaskStorageConfig::new()?)
    }

    /// 通过可注入配置创建任务管理器。
    pub fn with_config(config: TaskStorageConfig) -> Result<Self> {
        let storage = TaskStorage::new(config)?;
        Self::with_storage(storage)
    }

    /// 通过自定义文件路径创建任务管理器，适合测试或临时示例。
    pub fn with_storage_path(path: impl Into<std::path::PathBuf>) -> Result<Self> {
        Self::with_config(TaskStorageConfig::from_path(path))
    }

    /// 通过已构造的存储实例创建任务管理器。
    pub fn with_storage(storage: TaskStorage) -> Result<Self> {
        let (tasks, load_state) = match storage.load_tasks() {
            Ok(TaskLoadOutcome::NotFound) => (HashMap::new(), TaskManagerLoadState::InitializedEmpty),
            Ok(TaskLoadOutcome::Loaded(tasks)) => (tasks, TaskManagerLoadState::LoadedFromStorage),
            Err(error) => {
                return Err(anyhow::Error::new(error)
                    .context("Failed to initialize task manager from storage"));
            }
        };

        let next_id = next_task_id(&tasks);

        Ok(Self {
            tasks,
            next_id,
            storage,
            load_state,
        })
    }

    /// 当前初始化路径的数据来源状态。
    pub fn load_state(&self) -> TaskManagerLoadState {
        self.load_state
    }

    /// 当前存储文件路径。
    pub fn storage_path(&self) -> &Path {
        self.storage.path()
    }

    /// 将当前内存中的任务状态持久化到存储。
    pub fn persist(&self) -> Result<()> {
        self.save()
    }

    /// 添加任务
    pub fn add_task(&mut self, mut task: Task) -> Result<u64> {
        let task_id = self.next_id;
        task.assign_id(task_id);
        self.tasks.insert(task_id, task);
        self.next_id += 1;

        if let Err(error) = self.save() {
            self.tasks.remove(&task_id);
            self.next_id = task_id;
            return Err(error.context("Failed to persist newly added task"));
        }

        Ok(task_id)
    }

    /// 获取任务
    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    /// 获取受控的任务更新句柄。
    pub fn get_task_mut(&mut self, id: u64) -> Option<TaskUpdateHandle<'_>> {
        if self.tasks.contains_key(&id) {
            Some(TaskUpdateHandle { manager: self, id })
        } else {
            None
        }
    }

    /// 在保存与回滚保护下更新任务。
    pub fn update_task<F>(&mut self, id: u64, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Task),
    {
        let original_task = self.tasks.get(&id).cloned().context("Task not found")?;

        {
            let task = self.tasks.get_mut(&id).expect("task must exist after clone");
            updater(task);
        }

        if let Err(error) = self.save() {
            self.tasks.insert(id, original_task);
            return Err(error.context("Failed to persist task update"));
        }

        Ok(())
    }

    /// 将任务标记为进行中。
    pub fn start_task(&mut self, id: u64) -> Result<()> {
        self.update_task(id, Task::start)
    }

    /// 将任务标记为已完成。
    pub fn complete_task(&mut self, id: u64) -> Result<()> {
        self.update_task(id, Task::complete)
    }

    /// 将任务标记为已取消。
    pub fn cancel_task(&mut self, id: u64) -> Result<()> {
        self.update_task(id, Task::cancel)
    }

    /// 删除任务
    pub fn delete_task(&mut self, id: u64) -> Result<Task> {
        let task = self.tasks.remove(&id).context("Task not found")?;

        if let Err(error) = self.save() {
            self.tasks.insert(id, task.clone());
            return Err(error.context("Failed to persist task deletion"));
        }

        Ok(task)
    }

    /// 列出所有任务
    pub fn list_tasks(&self, filter: Option<Status>) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();

        if let Some(status) = filter {
            tasks.retain(|task| task.status() == status);
        }

        tasks.sort_by(|left, right| {
            left.priority()
                .sort_order()
                .cmp(&right.priority().sort_order())
                .then_with(|| right.created_at().cmp(left.created_at()))
        });

        tasks
    }

    /// 搜索任务
    pub fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        self.tasks
            .values()
            .filter(|task| {
                task.title().to_lowercase().contains(&query_lower)
                    || task
                        .description()
                        .map(|description| description.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || task
                        .tags()
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    /// 获取统计信息
    pub fn get_statistics(&self) -> TaskStatistics {
        let total = self.tasks.len();
        let completed = self
            .tasks
            .values()
            .filter(|task| task.status() == Status::Completed)
            .count();
        let pending = self
            .tasks
            .values()
            .filter(|task| task.status() == Status::Pending)
            .count();
        let in_progress = self
            .tasks
            .values()
            .filter(|task| task.status() == Status::InProgress)
            .count();
        let urgent = self
            .tasks
            .values()
            .filter(|task| {
                task.priority() == Priority::Urgent && task.status() != Status::Completed
            })
            .count();

        TaskStatistics {
            total,
            completed,
            pending,
            in_progress,
            urgent,
        }
    }

    fn save(&self) -> Result<()> {
        self.storage.save_tasks(&self.tasks)
    }
}

fn next_task_id(tasks: &HashMap<u64, Task>) -> u64 {
    tasks.keys().max().map(|max_id| max_id + 1).unwrap_or(1)
}
