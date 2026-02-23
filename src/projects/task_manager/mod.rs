//! # 任务管理器项目
//!
//! 这是一个完整的实战项目示例，演示如何构建一个功能完善的任务管理器 CLI 工具。
//!
//! ## 功能特性
//!
//! - 添加、删除、完成任务
//! - 任务优先级管理
//! - 任务分类和标签
//! - 数据持久化（JSON文件）
//! - 任务搜索和过滤
//! - 任务统计和报告
//!
//! ## 技术栈
//!
//! - `clap` - 命令行参数解析
//! - `serde` - 数据序列化
//! - `chrono` - 日期时间处理
//! - `anyhow` - 错误处理
//! - `colored` - 终端颜色输出

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 任务优先级
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Priority {
    /// 获取优先级的颜色表示
    fn color(&self) -> colored::Color {
        match self {
            Priority::Low => colored::Color::Green,
            Priority::Medium => colored::Color::Yellow,
            Priority::High => colored::Color::Magenta,
            Priority::Urgent => colored::Color::Red,
        }
    }
    
    /// 获取优先级的字符串表示
    fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "LOW",
            Priority::Medium => "MED",
            Priority::High => "HIGH",
            Priority::Urgent => "URGENT",
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;
    
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            "urgent" | "u" => Ok(Priority::Urgent),
            _ => Err(format!("Unknown priority: {}", s)),
        }
    }
}

/// 任务状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Pending => "PENDING",
            Status::InProgress => "IN_PROGRESS",
            Status::Completed => "COMPLETED",
            Status::Cancelled => "CANCELLED",
        }
    }
    
    fn symbol(&self) -> &'static str {
        match self {
            Status::Pending => "⏳",
            Status::InProgress => "🔄",
            Status::Completed => "✅",
            Status::Cancelled => "❌",
        }
    }
}

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub completed_at: Option<DateTime<Local>>,
    pub due_date: Option<DateTime<Local>>,
}

impl Task {
    /// 创建新任务
    pub fn new(id: u64, title: impl Into<String>, priority: Priority) -> Self {
        let now = Local::now();
        Self {
            id,
            title: title.into(),
            description: None,
            priority,
            status: Status::Pending,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            completed_at: None,
            due_date: None,
        }
    }
    
    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    
    /// 设置标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    /// 设置截止日期
    pub fn with_due_date(mut self, due: DateTime<Local>) -> Self {
        self.due_date = Some(due);
        self
    }
    
    /// 完成任务
    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.completed_at = Some(Local::now());
        self.updated_at = Local::now();
    }
    
    /// 开始任务
    pub fn start(&mut self) {
        self.status = Status::InProgress;
        self.updated_at = Local::now();
    }
    
    /// 取消任务
    pub fn cancel(&mut self) {
        self.status = Status::Cancelled;
        self.updated_at = Local::now();
    }
    
    /// 格式化显示任务
    pub fn display(&self) -> String {
        let priority_str = format!("[{}]", self.priority.as_str())
            .color(self.priority.color())
            .bold();
        
        let status_symbol = self.status.symbol();
        let title = if self.status == Status::Completed {
            self.title.strikethrough().to_string()
        } else {
            self.title.clone()
        };
        
        let mut result = format!("{} {} {} - {}", 
            status_symbol,
            priority_str,
            self.id.to_string().cyan(),
            title
        );
        
        if !self.tags.is_empty() {
            let tags_str = self.tags
                .iter()
                .map(|t| format!("#{}", t).dimmed().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            result.push_str(&format!(" {}", tags_str));
        }
        
        if let Some(ref due) = self.due_date {
            let due_str = format!("📅 {}", due.format("%Y-%m-%d"));
            result.push_str(&format!(" {}", due_str.yellow()));
        }
        
        result
    }
}

/// 任务存储管理器
pub struct TaskManager {
    tasks: HashMap<u64, Task>,
    next_id: u64,
    storage_path: PathBuf,
}

impl TaskManager {
    /// 创建新的任务管理器
    pub fn new() -> Result<Self> {
        let storage_path = Self::get_storage_path()?;
        let mut manager = Self {
            tasks: HashMap::new(),
            next_id: 1,
            storage_path,
        };
        
        // 尝试加载已有数据
        if let Err(e) = manager.load() {
            println!("{}: {}", "Info: No existing data found".yellow(), e);
        }
        
        Ok(manager)
    }
    
    /// 获取存储路径
    fn get_storage_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not find home directory")?;
        let app_dir = home.join(".task_manager");
        
        // 确保目录存在
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)
                .context("Failed to create app directory")?;
        }
        
        Ok(app_dir.join("tasks.json"))
    }
    
    /// 添加任务
    pub fn add_task(&mut self, mut task: Task) -> u64 {
        task.id = self.next_id;
        self.tasks.insert(task.id, task);
        self.next_id += 1;
        self.save().expect("Failed to save tasks");
        self.next_id - 1
    }
    
    /// 获取任务
    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }
    
    /// 获取可变任务
    pub fn get_task_mut(&mut self, id: u64) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }
    
    /// 删除任务
    pub fn delete_task(&mut self, id: u64) -> Result<Task> {
        let task = self.tasks.remove(&id)
            .context("Task not found")?;
        self.save()?;
        Ok(task)
    }
    
    /// 列出所有任务
    pub fn list_tasks(&self, filter: Option<Status>) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        
        if let Some(status) = filter {
            tasks.retain(|t| t.status == status);
        }
        
        // 按优先级和创建时间排序
        tasks.sort_by(|a, b| {
            let priority_order = |p: &Priority| match p {
                Priority::Urgent => 0,
                Priority::High => 1,
                Priority::Medium => 2,
                Priority::Low => 3,
            };
            
            priority_order(&a.priority)
                .cmp(&priority_order(&b.priority))
                .then_with(|| b.created_at.cmp(&a.created_at))
        });
        
        tasks
    }
    
    /// 搜索任务
    pub fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let query_lower = query.to_lowercase();
        self.tasks
            .values()
            .filter(|t| {
                t.title.to_lowercase().contains(&query_lower) ||
                t.description.as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false) ||
                t.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
    
    /// 获取统计信息
    pub fn get_statistics(&self) -> TaskStatistics {
        let total = self.tasks.len();
        let completed = self.tasks.values()
            .filter(|t| t.status == Status::Completed)
            .count();
        let pending = self.tasks.values()
            .filter(|t| t.status == Status::Pending)
            .count();
        let in_progress = self.tasks.values()
            .filter(|t| t.status == Status::InProgress)
            .count();
        
        let urgent = self.tasks.values()
            .filter(|t| t.priority == Priority::Urgent && t.status != Status::Completed)
            .count();
        
        TaskStatistics {
            total,
            completed,
            pending,
            in_progress,
            urgent,
        }
    }
    
    /// 保存到文件
    fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.tasks)
            .context("Failed to serialize tasks")?;
        fs::write(&self.storage_path, data)
            .context("Failed to write tasks file")?;
        Ok(())
    }
    
    /// 从文件加载
    fn load(&mut self) -> Result<()> {
        let data = fs::read_to_string(&self.storage_path)
            .context("Failed to read tasks file")?;
        self.tasks = serde_json::from_str(&data)
            .context("Failed to parse tasks file")?;
        
        // 更新 next_id
        if let Some(&max_id) = self.tasks.keys().max() {
            self.next_id = max_id + 1;
        }
        
        Ok(())
    }
}

/// 任务统计信息
#[derive(Debug)]
pub struct TaskStatistics {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub in_progress: usize,
    pub urgent: usize,
}

impl TaskStatistics {
    /// 显示统计信息
    pub fn display(&self) {
        println!("\n{}", "📊 Task Statistics".bold().underline());
        println!("  {} {}", "Total:".bold(), self.total);
        println!("  {} {} ({:.1}%)", 
            "Completed:".green().bold(), 
            self.completed,
            if self.total > 0 {
                (self.completed as f64 / self.total as f64) * 100.0
            } else {
                0.0
            }
        );
        println!("  {} {}", "Pending:".yellow().bold(), self.pending);
        println!("  {} {}", "In Progress:".blue().bold(), self.in_progress);
        
        if self.urgent > 0 {
            println!("  {} {}", "⚠️  Urgent:".red().bold(), self.urgent);
        }
    }
}

/// 运行任务管理器演示
pub fn run_task_manager_demo() {
    println!("{}", "🎯 Task Manager Demo".bold().green());
    println!("{}", "═══════════════════════════════════════".dimmed());
    
    let mut manager = TaskManager::new().expect("Failed to create task manager");
    
    // 添加示例任务
    println!("\n{}", "Adding sample tasks...".cyan());
    
    let task1 = Task::new(0, "Complete Rust project", Priority::High)
        .with_description("Finish the task manager implementation")
        .with_tags(vec!["rust".to_string(), "project".to_string()]);
    let id1 = manager.add_task(task1);
    println!("  Added task #{}: Complete Rust project", id1);
    
    let task2 = Task::new(0, "Review pull requests", Priority::Medium)
        .with_tags(vec!["code-review".to_string()]);
    let id2 = manager.add_task(task2);
    println!("  Added task #{}: Review pull requests", id2);
    
    let task3 = Task::new(0, "Fix critical bug in production", Priority::Urgent)
        .with_description("Users are reporting crashes")
        .with_tags(vec!["bug".to_string(), "production".to_string()]);
    let id3 = manager.add_task(task3);
    println!("  Added task #{}: Fix critical bug in production", id3);
    
    let task4 = Task::new(0, "Update documentation", Priority::Low)
        .with_tags(vec!["docs".to_string()]);
    let id4 = manager.add_task(task4);
    println!("  Added task #{}: Update documentation", id4);
    
    // 完成任务
    println!("\n{}", "Completing a task...".cyan());
    if let Some(task) = manager.get_task_mut(id2) {
        task.complete();
        println!("  ✅ Completed task #{}: Review pull requests", id2);
    }
    
    // 列出所有任务
    println!("\n{}", "All Tasks:".bold().underline());
    for task in manager.list_tasks(None) {
        println!("  {}", task.display());
    }
    
    // 显示统计
    manager.get_statistics().display();
    
    // 搜索任务
    println!("\n{}", "Search results for 'rust':".bold().underline());
    for task in manager.search_tasks("rust") {
        println!("  {}", task.display());
    }
    
    // 清理
    println!("\n{}", "Cleaning up demo tasks...".cyan());
    let _ = manager.delete_task(id1);
    let _ = manager.delete_task(id2);
    let _ = manager.delete_task(id3);
    let _ = manager.delete_task(id4);
    println!("  ✅ Demo completed!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test task", Priority::High);
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test task");
        assert_eq!(task.priority, Priority::High);
        assert_eq!(task.status, Status::Pending);
    }
    
    #[test]
    fn test_task_completion() {
        let mut task = Task::new(1, "Test task", Priority::Medium);
        assert_eq!(task.status, Status::Pending);
        assert!(task.completed_at.is_none());
        
        task.complete();
        assert_eq!(task.status, Status::Completed);
        assert!(task.completed_at.is_some());
    }
    
    #[test]
    fn test_priority_from_str() {
        assert_eq!("high".parse::<Priority>().unwrap(), Priority::High);
        assert_eq!("LOW".parse::<Priority>().unwrap(), Priority::Low);
        assert_eq!("u".parse::<Priority>().unwrap(), Priority::Urgent);
        assert!("unknown".parse::<Priority>().is_err());
    }
}
