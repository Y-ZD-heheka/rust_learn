//! # 测试和文档模块
//!
//! 这个模块现在按“学习主题”拆分，帮助学习者从被测对象、文档测试、
//! 测试策略到性能与集成场景逐层理解 Rust 测试体系。
//!
//! 门面层仅 re-export 当前希望对外稳定暴露的教学入口；
//! 目录内的其它辅助实现继续保留在子模块中，避免把示例内部细节误导为稳定 API。

mod domain;
mod documentation;
mod performance;
mod strategies;

pub use domain::{
    EmailValidationError, User, UserCreationError, UserManager, UserManagerError, add_two,
    greeting, validate_email,
};
pub use documentation::{documented_function, run_testing_examples, timed_operations_demo};
pub use performance::{integration_testing_scenarios, performance_testing_examples};
pub use strategies::{
    boundary_and_error_testing, enterprise_testing_strategies, property_thinking_basics,
    test_driven_development_example,
};

