//! # Rust 学习项目库
//!
//! 该库公开“可复用或可验证”的学习主题模块，供外部调用、测试与文档示例复用。
//! 偏说明性的 CLI 导览内容会保留在主程序侧运行，而不作为稳定库 API 导出，
//! 以避免与其它可验证源码处于同一公开层级。
//!
//! 其中：
//! - `projects` 是独立实战项目专题入口，保留为库模块，但不纳入主学习 CLI。
//! - `database` 是带运行环境要求的 SQLite + SQLx 教学示例，保留为库模块，
//!   但同样不纳入默认 CLI 主路径。

pub mod basics;
pub mod ownership;
pub mod types;
pub mod error_handling;
pub mod concurrency;
pub mod modules;
pub mod macros;
pub mod advanced_types;
pub mod testing;
pub mod popular_libraries;
#[doc(hidden)]
pub mod ecosystem;
pub mod advanced_patterns;
pub mod security;
pub mod best_practices;
pub mod pitfalls;
pub mod projects;
pub mod database;
