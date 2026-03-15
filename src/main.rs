//! # Rust学习项目主程序
//!
//! 这个程序演示了如何运行当前已接入 CLI 的学习模块。
//! crate 使用 Rust 2024 Edition；CLI 注册表覆盖所有稳定接入的学习主题，
//! `projects` 保持为独立实战项目入口，`database` 已作为库模块开放，
//! 但仍暂不纳入主学习 CLI，避免将数据库运行环境要求引入默认主路径。
//! `ecosystem` 目前保留为说明性导览模块：仍可从 CLI 运行与从库侧兼容访问，
//! 但不应被视为与其它可验证源码等价的稳定工程 API。

use std::env;
use std::time::{Duration, Instant};

/// CLI 学习阶段，用于组织帮助输出和推荐学习路径。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LearningStage {
    Foundation,
    LanguageAndEngineering,
    AdvancedTopics,
    PracticeTracks,
}

impl LearningStage {
    fn title(self) -> &'static str {
        match self {
            Self::Foundation => "基础主线",
            Self::LanguageAndEngineering => "语言与工程进阶",
            Self::AdvancedTopics => "高级主题",
            Self::PracticeTracks => "实践专题",
        }
    }

    fn focus(self) -> &'static str {
        match self {
            Self::Foundation => "先建立语法、所有权、类型系统与错误处理的核心心智模型。",
            Self::LanguageAndEngineering => "再补齐模块化、测试、生态工具与工程规范，形成可维护代码习惯。",
            Self::AdvancedTopics => "随后进入并发、宏、高级类型与设计模式，理解更复杂的 Rust 抽象能力。",
            Self::PracticeTracks => "最后结合库生态、安全与常见陷阱做实践复盘，把知识串成可落地经验。",
        }
    }
}

const LEARNING_STAGES: [LearningStage; 4] = [
    LearningStage::Foundation,
    LearningStage::LanguageAndEngineering,
    LearningStage::AdvancedTopics,
    LearningStage::PracticeTracks,
];

/// CLI 中可直接运行的模块信息。
#[derive(Debug, Clone, Copy)]
struct ModuleInfo {
    name: &'static str,
    description: &'static str,
    stage: LearningStage,
    run_function: fn(),
}

/// 已在仓库中存在、但当前不纳入主学习 CLI 的模块状态说明。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModuleStatus {
    name: &'static str,
    summary: &'static str,
    reading_path: &'static str,
    suggestion: &'static str,
}

/// 当前已接入 CLI 的学习模块注册表。
const MODULE_REGISTRY: &[ModuleInfo] = &[
    ModuleInfo {
        name: "basics",
        description: "基础语法和核心概念",
        stage: LearningStage::Foundation,
        run_function: rust_learn::basics::run_basics_examples,
    },
    ModuleInfo {
        name: "ownership",
        description: "所有权、借用和生命周期",
        stage: LearningStage::Foundation,
        run_function: rust_learn::ownership::run_ownership_examples,
    },
    ModuleInfo {
        name: "types",
        description: "类型系统、结构体、枚举和特征",
        stage: LearningStage::Foundation,
        run_function: rust_learn::types::run_types_examples,
    },
    ModuleInfo {
        name: "error_handling",
        description: "错误处理和 Result 类型",
        stage: LearningStage::Foundation,
        run_function: rust_learn::error_handling::run_error_handling_examples,
    },
    ModuleInfo {
        name: "modules",
        description: "模块系统和包管理",
        stage: LearningStage::LanguageAndEngineering,
        run_function: rust_learn::modules::run_modules_examples,
    },
    ModuleInfo {
        name: "testing",
        description: "测试策略和质量保证",
        stage: LearningStage::LanguageAndEngineering,
        run_function: rust_learn::testing::run_testing_examples,
    },
    ModuleInfo {
        name: "ecosystem",
        description: "生态系统与工具导览（说明性内容，非稳定工程 API）",
        stage: LearningStage::LanguageAndEngineering,
        run_function: rust_learn::ecosystem::run_ecosystem_examples,
    },
    ModuleInfo {
        name: "best_practices",
        description: "工程最佳实践与编码规范",
        stage: LearningStage::LanguageAndEngineering,
        run_function: rust_learn::best_practices::run_best_practices_examples,
    },
    ModuleInfo {
        name: "concurrency",
        description: "并发编程和异步处理",
        stage: LearningStage::AdvancedTopics,
        run_function: rust_learn::concurrency::run_concurrency_examples,
    },
    ModuleInfo {
        name: "macros",
        description: "宏系统和元编程",
        stage: LearningStage::AdvancedTopics,
        run_function: rust_learn::macros::run_macros_examples,
    },
    ModuleInfo {
        name: "advanced_types",
        description: "高级类型系统和生命周期",
        stage: LearningStage::AdvancedTopics,
        run_function: rust_learn::advanced_types::run_advanced_types_examples,
    },
    ModuleInfo {
        name: "advanced_patterns",
        description: "进阶设计模式和架构模式",
        stage: LearningStage::AdvancedTopics,
        run_function: rust_learn::advanced_patterns::run_all_patterns,
    },
    ModuleInfo {
        name: "popular_libraries",
        description: "热门 Rust 库聚合演示（Serde、Clap、Reqwest 等）",
        stage: LearningStage::PracticeTracks,
        run_function: rust_learn::popular_libraries::run_popular_libraries_examples,
    },
    ModuleInfo {
        name: "security",
        description: "安全编程实践与输入验证",
        stage: LearningStage::PracticeTracks,
        run_function: rust_learn::security::run_security_examples,
    },
    ModuleInfo {
        name: "pitfalls",
        description: "常见陷阱、反模式与规避方式",
        stage: LearningStage::PracticeTracks,
        run_function: rust_learn::pitfalls::run_pitfalls_examples,
    },
];

/// 仓库中存在但当前不纳入主学习 CLI 的模块状态。
const NON_CLI_MODULES: &[ModuleStatus] = &[
    ModuleStatus {
        name: "projects",
        summary: "独立实战项目集合，保留为库中的专题入口，不并入主学习 CLI。",
        reading_path: "src/projects/mod.rs -> src/projects/task_manager/mod.rs",
        suggestion: "如果你想继续主线学习，建议改为运行 `cargo run -- popular_libraries` 或 `cargo run -- best_practices`；如果想看项目化示例，请直接阅读对应源码。",
    },
    ModuleStatus {
        name: "database",
        summary: "数据库模块已统一为 SQLite + SQLx 教学示例，并作为库模块开放；但当前仍不纳入主学习 CLI。",
        reading_path: "src/database.rs",
        suggestion: "如需阅读数据库主题，可直接查看源码；若需要稳定 CLI 主线学习，建议继续使用 `cargo run -- ecosystem` 或 `cargo run -- popular_libraries`。",
    },
];

/// CLI 错误类型。
#[derive(Debug, PartialEq, Eq)]
enum AppError {
    UnknownModule(String),
    UnavailableModule(&'static ModuleStatus),
    TooManyArguments,
    ModuleExecutionFailed(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownModule(module) => write!(f, "未知模块: {}", module),
            Self::UnavailableModule(module) => {
                write!(f, "模块 '{}' 当前不通过主学习 CLI 运行", module.name)
            }
            Self::TooManyArguments => {
                write!(f, "参数过多；仅支持一个模块名，或使用 help / --help / --performance")
            }
            Self::ModuleExecutionFailed(module) => {
                write!(f, "模块执行失败: {}", module)
            }
        }
    }
}

impl std::error::Error for AppError {}

fn find_module(name: &str) -> Option<&'static ModuleInfo> {
    MODULE_REGISTRY.iter().find(|module| module.name == name)
}

fn find_non_cli_module(name: &str) -> Option<&'static ModuleStatus> {
    NON_CLI_MODULES.iter().find(|module| module.name == name)
}

fn modules_in_stage(stage: LearningStage) -> impl Iterator<Item = &'static ModuleInfo> {
    MODULE_REGISTRY
        .iter()
        .filter(move |module| module.stage == stage)
}

fn count_modules_in_stage(stage: LearningStage) -> usize {
    modules_in_stage(stage).count()
}

fn next_module_after(name: &str) -> Option<&'static ModuleInfo> {
    MODULE_REGISTRY
        .iter()
        .position(|module| module.name == name)
        .and_then(|index| MODULE_REGISTRY.get(index + 1))
}

fn suggested_modules(input: &str) -> Vec<&'static str> {
    let query = input.to_ascii_lowercase();
    let mut matches: Vec<&'static str> = MODULE_REGISTRY
        .iter()
        .filter(|module| {
            module.name.starts_with(&query)
                || module.name.contains(&query)
                || query.starts_with(module.name)
        })
        .map(|module| module.name)
        .take(3)
        .collect();

    if matches.is_empty() {
        matches.extend(["basics", "ownership", "types"]);
    }

    matches
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs_f64() >= 1.0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else {
        format!("{:.2}ms", duration.as_secs_f64() * 1_000.0)
    }
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "非字符串 panic 载荷".to_string(),
        },
    }
}

fn execute_module(module: &ModuleInfo) -> Result<Duration, AppError> {
    let start_time = Instant::now();
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(module.run_function)).map_err(
        |payload| {
            let message = panic_message(payload);
            AppError::ModuleExecutionFailed(format!("{}（panic: {}）", module.name, message))
        },
    )?;
    Ok(start_time.elapsed())
}

fn print_learning_path_overview() {
    println!("🗺️ 推荐学习顺序:");
    for (index, stage) in LEARNING_STAGES.iter().enumerate() {
        println!("  {}. {}：{}", index + 1, stage.title(), stage.focus());
    }
    println!();
}

fn print_stage_modules(stage: LearningStage) {
    let modules: Vec<&ModuleInfo> = modules_in_stage(stage).collect();
    println!("📚 {}（{} 个）", stage.title(), modules.len());
    println!("   {}", stage.focus());
    for module in modules {
        println!("   {:<18} - {}", module.name, module.description);
    }
    println!();
}

fn print_non_cli_modules() {
    println!("📌 仓库中存在但当前不纳入主学习 CLI 的模块:");
    for module in NON_CLI_MODULES {
        println!("  {:<18} - {}", module.name, module.summary);
        println!("  {:<18}   阅读路径: {}", "", module.reading_path);
        println!("  {:<18}   替代建议: {}", "", module.suggestion);
    }
    println!();
}

fn print_post_module_guidance(module: &ModuleInfo) {
    if let Some(next_module) = next_module_after(module.name) {
        if next_module.stage == module.stage {
            println!(
                "📌 下一步建议: 继续同阶段学习，运行 `cargo run -- {}`（{}）",
                next_module.name, next_module.description
            );
        } else {
            println!(
                "📌 下一步建议: 当前阶段完成后进入“{}”，可运行 `cargo run -- {}`（{}）",
                next_module.stage.title(),
                next_module.name,
                next_module.description
            );
        }
    } else {
        println!(
            "📌 下一步建议: 可以运行 `cargo run` 复盘完整学习路径，或使用 `cargo run -- --help` 查看全部分组。"
        );
    }
}

fn print_error_guidance(error: &AppError) {
    match error {
        AppError::UnknownModule(module) => {
            let suggestions = suggested_modules(module).join("、");
            eprintln!("💡 下一步建议:");
            eprintln!("   - 使用 `cargo run -- --help` 查看按学习阶段分组的模块列表");
            eprintln!("   - 可以先从这些模块开始: {}", suggestions);
            eprintln!(
                "   - 如果你想看独立实战项目，请阅读 `src/projects/mod.rs` 与 `src/projects/task_manager/mod.rs`"
            );
        }
        AppError::UnavailableModule(module) => {
            eprintln!("💡 模块说明:");
            eprintln!("   - {}", module.summary);
            eprintln!("   - 阅读路径: {}", module.reading_path);
            eprintln!("   - 下一步建议: {}", module.suggestion);
            eprintln!("   - 使用 `cargo run -- --help` 可查看主学习 CLI 当前支持的模块");
        }
        AppError::TooManyArguments => {
            eprintln!("💡 支持的调用方式:");
            eprintln!("   - `cargo run`");
            eprintln!("   - `cargo run -- <module>`");
            eprintln!("   - `cargo run -- help` 或 `cargo run -- --help`");
            eprintln!("   - `cargo run -- --performance`");
        }
        AppError::ModuleExecutionFailed(module) => {
            eprintln!("💡 下一步建议:");
            eprintln!("   - 可先单独重试 `cargo run -- {}`，聚焦查看该模块输出", module);
            eprintln!("   - 若只是查看学习路径，可使用 `cargo run -- --help`");
        }
    }
}

/// 运行所有已接入 CLI 的模块示例。
fn run_all_examples() -> Result<(), AppError> {
    println!("🚀 启动 Rust 学习项目");
    println!("📦 Rust Edition: 2024");
    println!(
        "🧭 当前 CLI 已接入 {} 个学习主题，覆盖 {} 个学习阶段",
        MODULE_REGISTRY.len(),
        LEARNING_STAGES.len()
    );
    println!("📚 执行顺序将遵循推荐学习路径：基础主线 → 语言与工程进阶 → 高级主题 → 实践专题");
    println!();

    let start_time = Instant::now();
    let total_modules = MODULE_REGISTRY.len();
    let mut success_count = 0;
    let mut failed_modules = Vec::new();

    for (index, module) in MODULE_REGISTRY.iter().enumerate() {
        print!(
            "[{}/{}] 执行模块: {} [{}] - {} ...",
            index + 1,
            total_modules,
            module.name,
            module.stage.title(),
            module.description
        );

        match execute_module(module) {
            Ok(duration) => {
                println!(" ✅ 完成 ({})", format_duration(duration));
                success_count += 1;
            }
            Err(AppError::ModuleExecutionFailed(_)) => {
                println!(" ❌ 失败");
                eprintln!("⚠️ 模块 '{}' 执行失败", module.name);
                failed_modules.push(module.name);
            }
            Err(other) => return Err(other),
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    let total_duration = start_time.elapsed();
    let average_duration = Duration::from_secs_f64(total_duration.as_secs_f64() / total_modules as f64);

    println!();
    println!("📊 执行统计:");
    println!("   ✅ 成功模块: {}/{}", success_count, total_modules);
    println!("   ⏱️ 总执行时间: {}", format_duration(total_duration));
    println!("   📈 平均模块时间: {}", format_duration(average_duration));

    if failed_modules.is_empty() {
        println!("\n🎉 所有已接入模块执行成功，CLI 演示完成！");
        println!("💡 可继续使用 `cargo run -- <module>` 单独复习某一主题。");
        Ok(())
    } else {
        let failed = failed_modules.join(", ");
        eprintln!("\n⚠️ 以下模块执行失败: {}", failed);
        Err(AppError::ModuleExecutionFailed(failed))
    }
}

fn resolve_module_request(module_name: &str) -> Result<&'static ModuleInfo, AppError> {
    if let Some(module) = find_module(module_name) {
        return Ok(module);
    }

    if let Some(module_status) = find_non_cli_module(module_name) {
        return Err(AppError::UnavailableModule(module_status));
    }

    Err(AppError::UnknownModule(module_name.to_string()))
}

/// 运行指定模块示例。
fn run_specific_example(module_name: &str) -> Result<(), AppError> {
    let module = resolve_module_request(module_name)?;

    println!("🎯 执行模块: {} - {}", module.name, module.description);
    println!("🧭 所属阶段: {}", module.stage.title());
    println!("📖 学习目标: {}", module.stage.focus());
    println!();

    let duration = execute_module(module)?;
    println!("\n✅ {} 模块执行完成！耗时: {}", module.name, format_duration(duration));
    print_post_module_guidance(module);

    Ok(())
}

/// 输出 CLI 帮助信息。
fn print_usage() {
    println!("📖 Rust 学习项目 CLI 使用说明");
    println!();
    println!("🔧 基本用法:");
    println!("  cargo run                      - 按推荐学习路径运行所有已接入 CLI 的学习模块");
    println!("  cargo run -- <module>          - 运行指定学习模块");
    println!("  cargo run -- help              - 显示此帮助信息（等价于 --help）");
    println!("  cargo run -- --performance     - 显示性能与学习阶段概览");
    println!();

    print_learning_path_overview();
    println!("📚 当前已接入 CLI 的学习模块（{} 个）:", MODULE_REGISTRY.len());
    println!();

    for stage in LEARNING_STAGES {
        print_stage_modules(stage);
    }

    print_non_cli_modules();

    println!("💡 学习建议:");
    println!("  - 初学者优先从 `basics -> ownership -> types -> error_handling` 开始");
    println!("  - 想补工程能力，可继续看 `modules / testing / ecosystem / best_practices`");
    println!("  - 想冲刺难点，再进入 `concurrency / macros / advanced_types / advanced_patterns`");
    println!("  - 想做实践复盘，可查看 `popular_libraries / security / pitfalls`");
}

/// 输出性能和模块概览信息。
fn show_performance_info() {
    println!("🚀 Rust 学习项目性能信息:");
    println!("   🖥️ Rust Edition: 2024");
    println!("   📚 已接入 CLI 的学习模块: {}", MODULE_REGISTRY.len());
    println!("   📦 未纳入主学习 CLI 的模块: {}", NON_CLI_MODULES.len());
    println!("   🧭 学习阶段分布:");
    for stage in LEARNING_STAGES {
        println!("      - {}: {} 个模块", stage.title(), count_modules_in_stage(stage));
    }
    println!(
        "   📅 时间戳: {}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    );
}

/// 命令行参数。
#[derive(Debug, PartialEq, Eq)]
struct Args {
    module: Option<String>,
    show_help: bool,
    show_performance: bool,
}

fn parse_args_from<I, S>(args: I) -> Result<Args, AppError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();

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

    match args[1].as_str() {
        "--help" | "-h" | "help" => Ok(Args {
            module: None,
            show_help: true,
            show_performance: false,
        }),
        "--performance" | "-p" | "performance" => Ok(Args {
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

fn parse_args() -> Result<Args, AppError> {
    parse_args_from(env::args())
}

/// 程序主入口。
fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("❌ 参数解析错误: {}", error);
            print_error_guidance(&error);
            println!();
            print_usage();
            std::process::exit(1);
        }
    };

    if args.show_help {
        print_usage();
        return;
    }

    if args.show_performance {
        show_performance_info();
        return;
    }

    let result = match args.module {
        Some(ref module) => run_specific_example(module),
        None => run_all_examples(),
    };

    match result {
        Ok(()) => {
            println!("✅ 程序执行成功");
            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("❌ 程序执行失败: {}", error);
            print_error_guidance(&error);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli_args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parse_args_defaults_to_running_all_modules() {
        let args = parse_args_from(cli_args(&["rust_learn"])).expect("no args should parse");

        assert_eq!(
            args,
            Args {
                module: None,
                show_help: false,
                show_performance: false,
            },
            "empty invocation should keep CLI in run-all mode"
        );
    }

    #[test]
    fn parse_args_supports_help_aliases() {
        for alias in ["--help", "-h", "help"] {
            let args = parse_args_from(cli_args(&["rust_learn", alias]))
                .unwrap_or_else(|_| panic!("help alias {alias} should parse"));

            assert_eq!(
                args,
                Args {
                    module: None,
                    show_help: true,
                    show_performance: false,
                },
                "alias {alias} should route to help output"
            );
        }
    }

    #[test]
    fn parse_args_supports_performance_aliases() {
        for alias in ["--performance", "-p", "performance"] {
            let args = parse_args_from(cli_args(&["rust_learn", alias]))
                .unwrap_or_else(|_| panic!("performance alias {alias} should parse"));

            assert_eq!(
                args,
                Args {
                    module: None,
                    show_help: false,
                    show_performance: true,
                },
                "alias {alias} should route to performance overview"
            );
        }
    }

    #[test]
    fn parse_args_treats_other_values_as_module_names() {
        let args = parse_args_from(cli_args(&["rust_learn", "basics"]))
            .expect("module name should parse as positional argument");

        assert_eq!(
            args,
            Args {
                module: Some("basics".to_string()),
                show_help: false,
                show_performance: false,
            },
            "non-flag arguments should be preserved as module names"
        );
    }

    #[test]
    fn parse_args_rejects_extra_arguments() {
        let error = parse_args_from(cli_args(&["rust_learn", "basics", "extra"]))
            .expect_err("more than one user argument should be rejected");

        assert_eq!(
            error,
            AppError::TooManyArguments,
            "CLI should fail fast when users provide too many positional arguments"
        );
    }

    #[test]
    fn resolve_module_request_distinguishes_non_cli_modules() {
        let projects_status = find_non_cli_module("projects").expect("projects should be registered");
        let database_status = find_non_cli_module("database").expect("database should be registered");

        let projects_error =
            resolve_module_request("projects").expect_err("projects should be blocked from main CLI");
        let database_error =
            resolve_module_request("database").expect_err("database should be blocked from main CLI");

        assert_eq!(
            projects_error,
            AppError::UnavailableModule(projects_status),
            "projects should be reported as unavailable instead of unknown"
        );
        assert_eq!(
            database_error,
            AppError::UnavailableModule(database_status),
            "database should be reported as unavailable instead of unknown"
        );
    }

    #[test]
    fn resolve_module_request_reports_unknown_modules() {
        let error = resolve_module_request("not_a_real_module")
            .expect_err("unknown module names should return an error");

        assert_eq!(
            error,
            AppError::UnknownModule("not_a_real_module".to_string()),
            "unknown modules should remain distinguishable from disabled modules"
        );
    }
}
