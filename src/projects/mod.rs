//! # 实战项目模块
//!
//! 这个模块包含项目实战示例，帮助学习者理解如何将 Rust 知识组合到一个更接近真实场景的终端案例中。
//! 它保留为库中的专题入口，供源码阅读、集成测试与定向演示使用，但不纳入 [`crate::main`](src/main.rs) 的主学习 CLI。
//!
//! ## 项目列表
//!
//! - **Task Manager** - 任务管理器项目实战示例
//!   - 定位：主学习路径后的教学型案例，带有 CLI 思维，但不作为主学习 CLI 的内建工具
//!   - 内容：任务建模、状态流转、搜索过滤、JSON 持久化、统计展示
//!   - 结构：按 `model`、`manager`、`storage`、`stats`、`demo`、`tests` 拆分
//!   - 难度：初级到中级
//!
//! ## 学习目标
//!
//! 1. 理解如何将多个 crate 组合使用
//! 2. 学习项目结构和多文件代码组织
//! 3. 掌握错误处理、持久化加载语义与终端交互
//! 4. 实践文件操作、数据序列化与异常路径测试

pub mod task_manager;

use task_manager::run_task_manager_demo;

/// 运行项目专题演示。
///
/// 该入口用于定向体验实战案例，不属于主学习 CLI 的稳定模块注册表。
pub fn run_projects_demo() {
    println!("\n{}", "🚀 Rust Projects Demo".bold().green());
    println!("{}", "═══════════════════════════════════════".dimmed());
    
    let _ = run_task_manager_demo();
}

/// 项目专题对外暴露的常用类型集合。
pub mod prelude {
    pub use super::task_manager::{Priority, Status, Task, TaskManager};
}

use colored::Colorize;

