//! # 错误处理模块
//!
//! 这个模块已按“从基础到工程场景”的学习路径拆分：
//! - [`fundamentals`](src/error_handling/fundamentals.rs)：`panic!`、`Result`、`?` 操作符，以及统一的演示日志口径
//! - [`io_config`](src/error_handling/io_config.rs)：文件 IO 与配置错误
//! - [`domain`](src/error_handling/domain.rs)：业务校验、自定义错误与恢复策略
//! - [`infrastructure`](src/error_handling/infrastructure.rs)：资源加载与外部服务错误
//!
//! 公开入口仍保持兼容，方便 [`crate::main`](src/main.rs:526) 与测试继续通过。
//! [`run_error_handling_examples()`](src/error_handling.rs:36) 统一按学习路径运行所有演示，
//! 各子模块内部避免使用 `expect`/`eprintln!` 制造与“错误传播/恢复”主题相冲突的示范。
//! 门面层只公开当前需要的教学入口，避免用全局 `dead_code` 豁免掩盖未使用项。

mod domain;
mod fundamentals;
mod infrastructure;
mod io_config;

pub use domain::{
    business_validation_error_handling,
    modern_error_logging,
    modern_error_recovery,
    modern_error_types,
};
pub use fundamentals::{
    modern_panic_handling,
    modern_question_mark_patterns,
    modern_result_handling,
};
pub use infrastructure::{
    external_service_error_handling,
    network_error_handling,
    resource_loading_error_handling,
};
pub use io_config::{configuration_error_handling, file_system_error_handling};


/// 运行错误处理示例。
pub fn run_error_handling_examples() {
    println!("🎯 === 现代化错误处理示例 ===");
    println!("📚 学习路径：基础心智模型 → IO/配置 → 业务校验 → 系统边界错误");
    println!("🧭 说明：示例统一采用可观测输出，不在教学流程中使用 `expect` 直接崩溃");
    println!();

    println!("--- 第一阶段：错误处理基础 ---");
    modern_panic_handling();
    println!();

    modern_result_handling();
    println!();

    modern_question_mark_patterns();
    println!();

    println!("--- 第二阶段：业务中的错误表达 ---");
    modern_error_types();
    println!();

    modern_error_recovery();
    println!();

    modern_error_logging();
    println!();

    business_validation_error_handling();
    println!();

    println!("--- 第三阶段：工程场景中的错误处理 ---");
    file_system_error_handling();
    println!();

    configuration_error_handling();
    println!();

    network_error_handling();
    println!();

    resource_loading_error_handling();
    println!();

    external_service_error_handling();

    println!("\n✅ 所有错误处理示例运行完成！");
}
