//! # Rust学习项目主程序
//!
//! 这个程序演示了如何运行各个学习模块的示例代码。
//! 每个模块对应一个Rust核心概念的学习主题。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::env;
use std::time::Instant;

/// 现代化模块信息结构体
#[derive(Debug, Clone)]
struct ModuleInfo {
    name: &'static str,
    description: &'static str,
    run_function: fn(),
}

/// 现代化模块注册表
const MODULE_REGISTRY: &[ModuleInfo] = &[
    ModuleInfo {
        name: "basics",
        description: "基础语法和核心概念",
        run_function: rust_learn::basics::run_basics_examples,
    },
    ModuleInfo {
        name: "ownership",
        description: "所有权、借用和生命周期",
        run_function: rust_learn::ownership::run_ownership_examples,
    },
    ModuleInfo {
        name: "types",
        description: "类型系统、结构体、枚举和特征",
        run_function: rust_learn::types::run_types_examples,
    },
    ModuleInfo {
        name: "error_handling",
        description: "错误处理和Result类型",
        run_function: rust_learn::error_handling::run_error_handling_examples,
    },
    ModuleInfo {
        name: "concurrency",
        description: "并发编程和异步处理",
        run_function: rust_learn::concurrency::run_concurrency_examples,
    },
    ModuleInfo {
        name: "modules",
        description: "模块系统和包管理",
        run_function: rust_learn::modules::run_modules_examples,
    },
    ModuleInfo {
        name: "macros",
        description: "宏系统和元编程",
        run_function: rust_learn::macros::run_macros_examples,
    },
    ModuleInfo {
        name: "advanced_types",
        description: "高级类型系统和生命周期",
        run_function: rust_learn::advanced_types::run_advanced_types_examples,
    },
    ModuleInfo {
        name: "testing",
        description: "测试策略和质量保证",
        run_function: rust_learn::testing::run_testing_examples,
    },
    ModuleInfo {
        name: "ecosystem",
        description: "生态系统、工具和最佳实践",
        run_function: rust_learn::ecosystem::run_ecosystem_examples,
    },
    ModuleInfo {
        name: "popular_libraries",
        description: "热门Rust库使用案例(Serde、Clap、Reqwest等)",
        run_function: || {
            println!("运行热门库演示，使用命令: cargo run --popular_libraries serialize");
        },
    },
];

/// 现代化错误处理类型
#[derive(Debug)]
enum AppError {
    UnknownModule(String),
    TooManyArguments,
    IoError(std::io::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownModule(module) => write!(f, "未知模块: {}", module),
            Self::TooManyArguments => write!(f, "参数过多"),
            Self::IoError(e) => write!(f, "IO错误: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

/// 现代化运行所有模块的示例
fn run_all_examples() -> Result<(), AppError> {
    println!("🚀 启动现代化Rust学习项目");
    println!("📦 Rust版本: 2021 Edition");
    println!("🏗️ 项目状态: Rust 2021/2024现代化特性");
    println!();
    
    let start_time = Instant::now();
    let mut success_count = 0;
    let total_modules = MODULE_REGISTRY.len();

    for (index, module) in MODULE_REGISTRY.iter().enumerate() {
        print!("[{}/{}] 执行模块: {} - {} ...",
               index + 1, total_modules, module.name, module.description);
        
        // 现代化的模块执行（带错误处理）
        let module_start = Instant::now();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(module.run_function)) {
            Ok(_) => {
                let duration = module_start.elapsed();
                println!(" ✅ 完成 ({:.2}ms)", duration.as_millis());
                success_count += 1;
            }
            Err(_e) => {
                println!(" ❌ 失败");
                eprintln!("⚠️ 模块 '{}' 执行失败", module.name);
            }
        }
        
        // 模块间的间隔
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!();
    println!("📊 执行统计:");
    println!("   ✅ 成功模块: {}/{}", success_count, total_modules);
    println!("   ⏱️ 总执行时间: {:.2}s", start_time.elapsed().as_secs_f64());
    println!("   📈 平均模块时间: {:.2}ms",
             start_time.elapsed().as_millis() / total_modules as u128);

    if success_count == total_modules {
        println!("\n🎉 所有模块执行成功！现代化Rust学习项目运行完成！");
        Ok(())
    } else {
        eprintln!("\n⚠️ 部分模块执行失败，请检查上述错误信息");
        Err(AppError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} 个模块执行失败", total_modules - success_count)
        )))
    }
}

/// 现代化运行指定模块的示例
fn run_specific_example(module_name: &str) -> Result<(), AppError> {
    let module = MODULE_REGISTRY.iter()
        .find(|m| m.name == module_name)
        .ok_or_else(|| AppError::UnknownModule(module_name.to_string()))?;

    println!("🎯 执行模块: {} - {}", module.name, module.description);
    println!();

    let start_time = Instant::now();
    
    // 现代化的模块执行（带错误处理）
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(module.run_function))
        .map_err(|e| {
            eprintln!("⚠️ 模块 '{}' 执行失败: {:?}", module.name, e);
            AppError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "模块执行失败"))
        })?;

    let duration = start_time.elapsed();
    println!("\n✅ {} 模块执行完成！耗时: {:.2}ms", module.name, duration.as_millis());
    
    Ok(())
}

/// 现代化使用说明
fn print_usage() {
    println!("📖 现代化Rust学习项目使用指南");
    println!();
    println!("🔧 基本用法:");
    println!("  cargo run                    - 运行所有现代化学习示例");
    println!("  cargo run <module>           - 运行指定模块示例");
    println!("  cargo run --help             - 显示此帮助信息");
    println!();
    
    println!("📚 可用学习模块:");
    for module in MODULE_REGISTRY {
        println!("  {:<15} - {}", module.name, module.description);
    }
    println!();
    
    println!("🧪 测试和质量保证:");
    println!("  cargo test                   - 运行所有测试");
    println!("  cargo test --doc            - 运行文档测试");
    println!("  cargo clippy                - 代码质量检查");
    println!("  cargo fmt                   - 代码格式化");
    println!("  cargo audit                 - 安全审计");
    println!();
    
    println!("📖 文档生成:");
    println!("  cargo doc --open            - 生成并打开HTML文档");
    println!("  cargo doc --no-deps        - 只生成项目文档");
    println!();
    
    println!("🔨 高级用法:");
    println!("  cargo build --release       - 优化构建");
    println!("  cargo run --release        - 运行优化版本");
    println!("  cargo check                - 快速类型检查");
    println!();
    
    println!("📊 现代化特性:");
    println!("  ✅ Rust 2021 Edition 支持");
    println!("  ✅ 异步/等待语法");
    println!("  ✅ 现代化错误处理");
    println!("  ✅ 改进的生命周期管理");
    println!("  ✅ 泛型和特征系统");
    println!("  ✅ 现代化并发编程");
    println!();
    
    println!("💡 学习提示:");
    println!("  - 从基础模块开始，逐步进阶");
    println!("  - 关注代码中的现代化特性");
    println!("  - 尝试修改示例代码加深理解");
    println!("  - 使用 cargo test 验证你的修改");
    println!("  - 阅读生成的文档了解API细节");
}

/// 现代化性能监控
fn show_performance_info() {
    println!("🚀 性能信息:");
    println!("   🖥️ Rust版本: 2021 Edition");
    println!("   ⚙️ 现代化特性: Rust 2021/2024");
    println!("   📅 编译时间: {}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs());
}

/// 现代化命令行解析
#[derive(Debug)]
struct Args {
    module: Option<String>,
    show_help: bool,
    show_performance: bool,
}

fn parse_args() -> Result<Args, AppError> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        return Err(AppError::TooManyArguments);
    }
    
    if args.len() == 1 {
        return Ok(Args {
            module: None,
            show_help: false,
            show_performance: false,
        });
    }
    
    let arg = &args[1];
    match arg.as_str() {
        "--help" | "-h" => Ok(Args {
            module: None,
            show_help: true,
            show_performance: false,
        }),
        "--performance" | "-p" => Ok(Args {
            module: None,
            show_help: false,
            show_performance: true,
        }),
        other => Ok(Args {
            module: Some(other.to_string()),
            show_help: false,
            show_performance: false,
        }),
    }
}

/// 现代化主函数
fn main() {
    println!("🚀 启动现代化Rust学习项目");
    
    // 解析命令行参数
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("❌ 参数解析错误: {}", e);
            print_usage();
            std::process::exit(1);
        }
    };
    
    // 显示帮助信息
    if args.show_help {
        print_usage();
        return;
    }
    
    // 显示性能信息
    if args.show_performance {
        show_performance_info();
        return;
    }
    
    // 执行主逻辑
    let result = match args.module {
        Some(ref module) => run_specific_example(module),
        None => run_all_examples(),
    };
    
    // 现代化错误处理
    match result {
        Ok(_) => {
            println!("✅ 程序执行成功");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("❌ 程序执行失败: {}", e);
            std::process::exit(1);
        }
    }
}