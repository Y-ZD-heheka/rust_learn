//! 集成测试
//!
//! 这个文件包含了整个rust_learn库的集成测试。

use rust_learn::*;

// ==================== 基础模块测试 ====================

/// 测试基础语法模块
#[test]
fn test_basics_module() {
    // 测试基础模块可以正常导入和运行
    basics::run_basics_examples();
}

/// 测试所有权模块
#[test]
fn test_ownership_module() {
    ownership::run_ownership_examples();
}

/// 测试类型系统模块
#[test]
fn test_types_module() {
    types::run_types_examples();
}

/// 测试错误处理模块
#[test]
fn test_error_handling_module() {
    error_handling::run_error_handling_examples();
}

// ==================== 核心功能测试 ====================

/// 测试邮箱验证功能
#[test]
fn test_email_validation() {
    // 有效邮箱
    assert!(testing::validate_email("user@example.com"));
    assert!(testing::validate_email("test@domain.org"));
    assert!(testing::validate_email("user.name@sub.domain.com"));

    // 无效邮箱
    assert!(!testing::validate_email("invalid"));
    assert!(!testing::validate_email("@domain.com"));
    assert!(!testing::validate_email("user@"));
    assert!(!testing::validate_email("user@domain"));
    assert!(!testing::validate_email(""));
}

/// 测试用户创建功能
#[test]
fn test_user_creation() {
    let user = testing::User::new("Alice".to_string(), "alice@example.com".to_string(), 25)
        .expect("Failed to create user for test");

    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, 25);
    assert!(user.is_adult());
}

/// 测试无效邮箱的用户创建
#[test]
fn test_user_creation_invalid_email() {
    let result = testing::User::new("Bob".to_string(), "invalid-email".to_string(), 20);

    assert!(result.is_err());
}

/// 测试未成年用户创建
#[test]
fn test_user_creation_minor() {
    let result = testing::User::new("Charlie".to_string(), "charlie@example.com".to_string(), 12);

    assert!(result.is_err());
}

// ==================== 模块集成测试 ====================

/// 测试测试模块的核心功能
#[test]
fn test_testing_module_core() {
    assert_eq!(testing::add_two(3), 5);
    assert_eq!(testing::add_two(-1), 1);
    assert_eq!(testing::greeting("Rust"), "你好，Rust！");
}

/// 测试并发模块
#[test]
fn test_concurrency_module() {
    // 运行并发示例，确保不会panic
    concurrency::run_concurrency_examples();
}

/// 测试模块系统
#[test]
fn test_modules_module() {
    modules::run_modules_examples();
}

/// 测试宏模块
#[test]
fn test_macros_module() {
    macros::run_macros_examples();
}

/// 测试高级类型模块
#[test]
fn test_advanced_types_module() {
    advanced_types::run_advanced_types_examples();
}

/// 测试生态系统模块
#[test]
fn test_ecosystem_module() {
    ecosystem::run_ecosystem_examples();
}

/// 测试高级设计模式模块
#[test]
fn test_advanced_patterns_module() {
    advanced_patterns::run_all_patterns();
}

/// 测试最佳实践模块
#[test]
fn test_best_practices_module() {
    best_practices::run_best_practices_examples();
}

/// 测试常见陷阱模块
#[test]
fn test_pitfalls_module() {
    pitfalls::run_pitfalls_examples();
}

/// 测试安全编程模块
#[test]
fn test_security_module() {
    security::run_security_examples();
}

/// 测试实战项目模块
#[test]
fn test_projects_module() {
    projects::run_projects_demo();
}

// ==================== 端到端测试 ====================

/// 测试完整的用户工作流
#[test]
fn test_complete_user_workflow() {
    // 1. 创建用户
    let user = testing::User::new("TestUser".to_string(), "test@example.com".to_string(), 25)
        .expect("Should create user");

    // 2. 验证用户信息
    assert_eq!(user.name, "TestUser");
    assert!(user.is_adult());

    // 3. 测试问候功能
    let greeting = user.greet();
    assert!(greeting.contains("TestUser"));
}

/// 测试模块间的数据传递
#[test]
fn test_module_integration() {
    // 使用testing模块的函数
    let result = testing::add_two(10);
    assert_eq!(result, 12);

    // 验证结果可以用于其他操作
    let greeting = testing::greeting(&result.to_string());
    assert!(greeting.contains("12"));
}

// ==================== 性能测试 ====================

/// 性能测试示例
#[test]
#[ignore] // 默认忽略，需要明确运行: cargo test -- --ignored
fn test_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // 执行一些计算
    let mut sum: i64 = 0;
    for i in 0..100000 {
        sum += i as i64;
    }

    let duration = start.elapsed();
    println!("计算耗时: {:?}", duration);

    assert_eq!(sum, 4999950000);
    assert!(duration.as_millis() < 100); // 确保性能在合理范围内
}

/// 测试大数据量处理
#[test]
fn test_large_data_processing() {
    let data: Vec<i32> = (1..=10000).collect();

    // 使用迭代器处理
    let sum: i64 = data.iter().map(|&x| x as i64).sum();
    let expected_sum: i64 = (1..=10000).map(|x| x as i64).sum();

    assert_eq!(sum, expected_sum);
}

// ==================== 错误处理测试 ====================

/// 测试错误传播
#[test]
fn test_error_propagation() {
    fn may_fail(success: bool) -> Result<String, String> {
        if success {
            Ok("Success".to_string())
        } else {
            Err("Failed".to_string())
        }
    }

    fn process() -> Result<String, String> {
        let result = may_fail(true)?;
        Ok(format!("Processed: {}", result))
    }

    assert!(process().is_ok());
}

// ==================== 文档测试验证 ====================

/// 验证文档中的示例代码能正常工作
#[test]
fn test_documentation_examples() {
    // 验证testing模块的文档示例
    let result = testing::greeting("World");
    assert_eq!(result, "你好，World！");

    // 验证add_two函数
    assert_eq!(testing::add_two(5), 7);
}

// ==================== 边界条件测试 ====================

/// 测试边界条件
#[test]
fn test_edge_cases() {
    // 测试空字符串
    assert!(!testing::validate_email(""));

    // 测试极大年龄
    let result = testing::User::new("Old".to_string(), "old@example.com".to_string(), 255);
    assert!(result.is_ok());

    // 测试边界年龄
    let result = testing::User::new("Teen".to_string(), "teen@example.com".to_string(), 13);
    assert!(result.is_ok());
}
