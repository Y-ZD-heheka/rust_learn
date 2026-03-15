//! # 任务管理器项目实战示例
//!
//! 这是一个面向学习者的项目实战示例，演示如何把任务建模、JSON 持久化、统计汇总与终端交互
//! 组织成一个带有 CLI 思维的多文件案例。
//!
//! ## 示例内容
//!
//! - 添加、删除、完成任务
//! - 任务优先级管理
//! - 任务分类和标签
//! - 数据持久化（JSON 文件）
//! - 任务搜索和过滤
//! - 任务统计和报告
//!
//! ## 源码结构
//!
//! - `model`：任务实体、优先级与状态建模
//! - `manager`：任务集合管理、排序、过滤与持久化协调
//! - `storage`：JSON 存储，以及首次运行与真实加载错误的区分
//! - `stats`：统计汇总展示
//! - `demo`：终端演示流程
//! - `tests`：正常路径与异常路径覆盖
//!
//! ## 技术栈
//!
//! - `serde` - 数据序列化
//! - `chrono` - 日期时间处理
//! - `anyhow` / `thiserror` - 错误处理与错误分层
//! - `colored` - 终端颜色输出
//! - `dirs` - 默认存储路径解析

#![allow(dead_code)]

mod demo;
mod manager;
mod model;
mod stats;
mod storage;

pub use demo::run_task_manager_demo;
pub use manager::{TaskManager, TaskManagerLoadState};
pub use model::{Priority, Status, Task};
pub use stats::TaskStatistics;
pub use storage::{TaskLoadError, TaskLoadOutcome, TaskStorage, TaskStorageConfig};

#[cfg(test)]
mod tests;
