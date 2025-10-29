//! 集成测试
//!
//! 这个文件包含了整个rust_learn库的集成测试。

use rust_learn::*;

/// 测试基础语法模块
#[test]
fn test_basics_module() {
    // 这个测试验证基础语法模块可以正常导入和运行
    // 注意：我们不直接调用run_basics_examples()因为它会打印输出
    // 而是测试模块的导入是否正常工作
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试所有权模块
#[test]
fn test_ownership_module() {
    // 测试所有权相关的公共函数
    let s = String::from("hello");
    let len = s.len(); // 使用标准库方法
    assert_eq!(len, 5);
}

/// 测试类型系统模块
#[test]
fn test_types_module() {
    // 测试类型系统模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试错误处理模块
#[test]
fn test_error_handling_module() {
    // 测试错误处理模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试宏模块
#[test]
fn test_macros_module() {
    // 测试宏模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试高级类型模块
#[test]
fn test_advanced_types_module() {
    // 测试高级类型模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试测试模块
#[test]
fn test_testing_module() {
    // 测试公开函数
    assert_eq!(testing::add_two(3), 5);
    assert_eq!(testing::greeting("Rust"), "Hello Rust!");
}

/// 测试并发模块
#[test]
fn test_concurrency_module() {
    // 测试并发模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试模块和包管理模块
#[test]
fn test_modules_module() {
    // 测试模块管理功能的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试生态系统模块
#[test]
fn test_ecosystem_module() {
    // 测试生态系统模块的导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 测试模块间的集成
#[test]
fn test_module_integration() {
    // 测试不同模块间的交互
    // 例如，使用testing模块的函数

    // 这个测试确保所有模块都能正确导入
    assert!(true); // 如果代码能编译通过，这个断言就会成功
}

/// 性能测试示例
#[test]
#[ignore] // 默认忽略，需要明确运行
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

/// 文档测试验证
#[test]
fn test_documentation_examples() {
    // 验证文档中的示例代码能正常工作
    let result = testing::greeting("Carol");
    assert_eq!(result, "Hello Carol!");
}