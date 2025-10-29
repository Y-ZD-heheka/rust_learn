//! # Rust学习项目主程序
//!
//! 这个程序演示了如何运行各个学习模块的示例代码。
//! 每个模块对应一个Rust核心概念的学习主题。

use std::env;
use std::io;

/// 运行所有模块的示例
fn run_all_examples() {
    println!("🚀 运行所有Rust学习示例\n");

    rust_learn::basics::run_basics_examples();
    println!();

    rust_learn::ownership::run_ownership_examples();
    println!();

    rust_learn::types::run_types_examples();
    println!();

    rust_learn::error_handling::run_error_handling_examples();
    println!();

    rust_learn::concurrency::run_concurrency_examples();
    println!();

    rust_learn::modules::run_modules_examples();
    println!();

    rust_learn::macros::run_macros_examples();
    println!();

    rust_learn::advanced_types::run_advanced_types_examples();
    println!();

    rust_learn::testing::run_testing_examples();
    println!();

    rust_learn::ecosystem::run_ecosystem_examples();
    println!();

    println!("✅ 所有示例运行完成！");
}

/// 运行指定模块的示例
fn run_specific_example(module: &str) {
    println!("🎯 运行 {} 模块示例\n", module);

    match module {
        "basics" => rust_learn::basics::run_basics_examples(),
        "ownership" => rust_learn::ownership::run_ownership_examples(),
        "types" => rust_learn::types::run_types_examples(),
        "error_handling" => rust_learn::error_handling::run_error_handling_examples(),
        "concurrency" => rust_learn::concurrency::run_concurrency_examples(),
        "modules" => rust_learn::modules::run_modules_examples(),
        "macros" => rust_learn::macros::run_macros_examples(),
        "advanced_types" => rust_learn::advanced_types::run_advanced_types_examples(),
        "testing" => rust_learn::testing::run_testing_examples(),
        "ecosystem" => rust_learn::ecosystem::run_ecosystem_examples(),
        _ => {
            println!("❌ 未知模块: {}", module);
            print_usage();
            return;
        }
    }

    println!("\n✅ {} 模块示例运行完成！", module);
}

/// 打印使用说明
fn print_usage() {
    println!("📖 Rust学习项目使用方法:");
    println!("  cargo run                    - 运行所有示例");
    println!("  cargo run <module>           - 运行指定模块示例");
    println!();
    println!("📚 可用模块:");
    println!("  basics          - 基础语法");
    println!("  ownership       - 所有权系统");
    println!("  types           - 类型系统");
    println!("  error_handling  - 错误处理");
    println!("  concurrency     - 并发编程");
    println!("  modules         - 模块和包管理");
    println!("  macros          - 宏和元编程");
    println!("  advanced_types  - 高级类型和生命周期");
    println!("  testing         - 测试和文档");
    println!("  ecosystem       - 生态系统和工具");
    println!();
    println!("🧪 运行测试:");
    println!("  cargo test                   - 运行所有测试");
    println!("  cargo test --doc            - 运行文档测试");
    println!();
    println!("📖 生成文档:");
    println!("  cargo doc --open            - 生成并打开文档");
}

/// 主函数
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // 没有参数，运行所有示例
            run_all_examples();
        }
        2 => {
            // 一个参数，运行指定模块
            let module = &args[1];
            if module == "--help" || module == "-h" {
                print_usage();
            } else {
                run_specific_example(module);
            }
        }
        _ => {
            println!("❌ 参数过多");
            print_usage();
        }
    }
}