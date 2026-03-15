use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

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

    pub(crate) fn sort_order(&self) -> u8 {
        match self {
            Priority::Urgent => 0,
            Priority::High => 1,
            Priority::Medium => 2,
            Priority::Low => 3,
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
    pub fn as_str(&self) -> &'static str {
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

/// 面向外部的只读任务视图。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskView {
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

/// 任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    #[serde(flatten)]
    inner: TaskView,
}

impl Deref for Task {
    type Target = TaskView;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Task {
    /// 创建新任务
    pub fn new(id: u64, title: impl Into<String>, priority: Priority) -> Self {
        let now = Local::now();
        Self {
            inner: TaskView {
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
            },
        }
    }

    /// 任务 ID
    pub fn id(&self) -> u64 {
        self.inner.id
    }

    /// 任务标题
    pub fn title(&self) -> &str {
        &self.inner.title
    }

    /// 任务描述
    pub fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    /// 任务优先级
    pub fn priority(&self) -> Priority {
        self.inner.priority
    }

    /// 任务状态
    pub fn status(&self) -> Status {
        self.inner.status
    }

    /// 任务标签
    pub fn tags(&self) -> &[String] {
        &self.inner.tags
    }

    /// 创建时间
    pub fn created_at(&self) -> &DateTime<Local> {
        &self.inner.created_at
    }

    /// 更新时间
    pub fn updated_at(&self) -> &DateTime<Local> {
        &self.inner.updated_at
    }

    /// 完成时间
    pub fn completed_at(&self) -> Option<&DateTime<Local>> {
        self.inner.completed_at.as_ref()
    }

    /// 截止时间
    pub fn due_date(&self) -> Option<&DateTime<Local>> {
        self.inner.due_date.as_ref()
    }

    pub(crate) fn assign_id(&mut self, id: u64) {
        self.inner.id = id;
    }

    fn touch(&mut self) {
        self.inner.updated_at = Local::now();
    }

    /// 设置标题
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.inner.title = title.into();
        self.touch();
    }

    /// 设置描述
    pub fn set_description(&mut self, desc: Option<String>) {
        self.inner.description = desc;
        self.touch();
    }

    /// 设置优先级
    pub fn set_priority(&mut self, priority: Priority) {
        self.inner.priority = priority;
        self.touch();
    }

    /// 设置标签
    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.inner.tags = tags;
        self.touch();
    }

    /// 设置截止日期
    pub fn set_due_date(&mut self, due_date: Option<DateTime<Local>>) {
        self.inner.due_date = due_date;
        self.touch();
    }

    /// 设置描述
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.inner.description = Some(desc.into());
        self
    }

    /// 设置标签
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.inner.tags = tags;
        self
    }

    /// 设置截止日期
    pub fn with_due_date(mut self, due: DateTime<Local>) -> Self {
        self.inner.due_date = Some(due);
        self
    }

    /// 完成任务
    pub fn complete(&mut self) {
        self.inner.status = Status::Completed;
        self.inner.completed_at = Some(Local::now());
        self.touch();
    }

    /// 开始任务
    pub fn start(&mut self) {
        self.inner.status = Status::InProgress;
        self.inner.completed_at = None;
        self.touch();
    }

    /// 取消任务
    pub fn cancel(&mut self) {
        self.inner.status = Status::Cancelled;
        self.inner.completed_at = None;
        self.touch();
    }

    /// 格式化显示任务
    pub fn display(&self) -> String {
        let priority_str = format!("[{}]", self.inner.priority.as_str())
            .color(self.inner.priority.color())
            .bold();

        let status_symbol = self.inner.status.symbol();
        let title = if self.inner.status == Status::Completed {
            self.inner.title.strikethrough().to_string()
        } else {
            self.inner.title.clone()
        };

        let mut result = format!(
            "{} {} {} - {}",
            status_symbol,
            priority_str,
            self.inner.id.to_string().cyan(),
            title
        );

        if !self.inner.tags.is_empty() {
            let tags_str = self
                .inner
                .tags
                .iter()
                .map(|tag| format!("#{}", tag).dimmed().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            result.push_str(&format!(" {}", tags_str));
        }

        if let Some(ref due) = self.inner.due_date {
            let due_str = format!("📅 {}", due.format("%Y-%m-%d"));
            result.push_str(&format!(" {}", due_str.yellow()));
        }

        result
    }
}
