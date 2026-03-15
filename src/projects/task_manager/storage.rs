use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use super::model::Task;

#[derive(Debug, Clone)]
pub struct TaskStorageConfig {
    path: PathBuf,
}

impl TaskStorageConfig {
    /// 使用默认存储位置创建配置。
    pub fn new() -> Result<Self> {
        Ok(Self {
            path: default_storage_path()?,
        })
    }

    /// 直接注入指定文件路径，便于测试或示例隔离。
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Default for TaskStorageConfig {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self::from_path(fallback_storage_path()))
    }
}

#[derive(Debug, Error)]
pub enum TaskLoadError {
    #[error("Failed to read tasks file at {0}", .path.display())]
    Read { path: PathBuf, #[source] source: io::Error },
    #[error("Failed to parse tasks file at {0}", .path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug)]
pub enum TaskLoadOutcome {
    NotFound,
    Loaded(HashMap<u64, Task>),
}

#[derive(Debug, Clone)]
pub struct TaskStorage {
    path: PathBuf,
}

impl TaskStorage {
    pub fn new(config: TaskStorageConfig) -> Result<Self> {
        ensure_parent_dir(config.path())?;
        Ok(Self {
            path: config.path,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load_tasks(&self) -> std::result::Result<TaskLoadOutcome, TaskLoadError> {
        let data = match fs::read_to_string(&self.path) {
            Ok(data) => data,
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                return Ok(TaskLoadOutcome::NotFound);
            }
            Err(source) => {
                return Err(TaskLoadError::Read {
                    path: self.path.clone(),
                    source,
                });
            }
        };

        let tasks = serde_json::from_str(&data).map_err(|source| TaskLoadError::Parse {
            path: self.path.clone(),
            source,
        })?;

        Ok(TaskLoadOutcome::Loaded(tasks))
    }

    pub fn save_tasks(&self, tasks: &HashMap<u64, Task>) -> Result<()> {
        ensure_parent_dir(&self.path)?;
        let data = serde_json::to_vec_pretty(tasks).context("Failed to serialize tasks")?;
        let temp_path = temporary_path_for(&self.path);

        fs::write(&temp_path, &data).context("Failed to write temporary tasks file")?;

        match fs::rename(&temp_path, &self.path) {
            Ok(()) => Ok(()),
            Err(rename_error) => {
                if rename_error.kind() == io::ErrorKind::AlreadyExists {
                    fs::remove_file(&self.path)
                        .context("Failed to remove existing tasks file before replacement")?;
                    fs::rename(&temp_path, &self.path)
                        .context("Failed to replace tasks file with temporary file")?;
                    Ok(())
                } else {
                    let _ = fs::remove_file(&temp_path);
                    Err(rename_error).context("Failed to atomically replace tasks file")
                }
            }
        }
    }
}

fn default_storage_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".task_manager").join("tasks.json"))
}

fn fallback_storage_path() -> PathBuf {
    PathBuf::from(".task_manager").join("tasks.json")
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create app directory")?;
    }
    Ok(())
}

fn temporary_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("tasks.json");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    let temp_file_name = format!(".{file_name}.{timestamp}.tmp");

    match path.parent() {
        Some(parent) => parent.join(temp_file_name),
        None => PathBuf::from(temp_file_name),
    }
}
