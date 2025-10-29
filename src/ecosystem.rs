//! # 生态系统和工具模块
//!
//! 这个模块演示了Rust生态系统的重要工具和概念。

/// 演示Cargo的使用
pub fn cargo_usage() {
    println!("Cargo是Rust的包管理器和构建工具");
    println!("常用命令:");
    println!("  cargo new <project>     - 创建新项目");
    println!("  cargo build             - 构建项目");
    println!("  cargo run               - 运行项目");
    println!("  cargo test              - 运行测试");
    println!("  cargo doc               - 生成文档");
    println!("  cargo check             - 检查代码但不构建");
    println!("  cargo update            - 更新依赖");
    println!("  cargo publish           - 发布crate到crates.io");
}

/// 演示crates.io生态系统
pub fn crates_io() {
    println!("crates.io是Rust的官方包注册表");
    println!("常用crate:");
    println!("  serde - 序列化/反序列化");
    println!("  tokio - 异步运行时");
    println!("  clap - 命令行参数解析");
    println!("  reqwest - HTTP客户端");
    println!("  diesel - ORM和查询构建器");
    println!("  actix-web - Web框架");
}

/// 演示Rustup工具链管理器
pub fn rustup() {
    println!("Rustup是Rust工具链安装器");
    println!("常用命令:");
    println!("  rustup update           - 更新Rust");
    println!("  rustup install <version> - 安装特定版本");
    println!("  rustup default <version> - 设置默认版本");
    println!("  rustup target add <target> - 添加目标平台");
    println!("  rustup component add <component> - 添加组件");
}

/// 演示Clippy代码检查工具
pub fn clippy() {
    println!("Clippy是Rust的代码检查工具");
    println!("运行方式:");
    println!("  cargo clippy");
    println!("它可以检测常见的错误和代码异味");
}

/// 演示Rustfmt代码格式化工具
pub fn rustfmt() {
    println!("Rustfmt是Rust的代码格式化工具");
    println!("运行方式:");
    println!("  cargo fmt");
    println!("它确保代码风格一致");
}

/// 演示Rust文档工具
pub fn rustdoc() {
    println!("Rustdoc从代码注释生成HTML文档");
    println!("运行方式:");
    println!("  cargo doc --open");
    println!("生成的文档包含API参考和示例");
}

/// 演示基准测试
pub fn benchmarking() {
    println!("Rust支持性能基准测试");
    println!("使用criterion crate进行基准测试:");
    println!("  cargo add criterion --dev");
    println!("  cargo bench");
}

/// 演示交叉编译
pub fn cross_compilation() {
    println!("Rust支持交叉编译到多个平台");
    println!("添加目标:");
    println!("  rustup target add x86_64-unknown-linux-gnu");
    println!("构建:");
    println!("  cargo build --target x86_64-unknown-linux-gnu");
}

/// 演示WebAssembly支持
pub fn webassembly() {
    println!("Rust可以编译到WebAssembly");
    println!("安装目标:");
    println!("  rustup target add wasm32-unknown-unknown");
    println!("构建:");
    println!("  cargo build --target wasm32-unknown-unknown");
}

/// 演示嵌入式开发
pub fn embedded() {
    println!("Rust支持嵌入式开发");
    println!("常用crate:");
    println!("  cortex-m - ARM Cortex-M微控制器");
    println!("  stm32f4xx-hal - STM32F4 HAL");
    println!("  arduino-hal - Arduino HAL");
}

/// 演示游戏开发
pub fn gamedev() {
    println!("Rust在游戏开发中越来越受欢迎");
    println!("常用crate:");
    println!("  ggez - 简单2D游戏框架");
    println!("  amethyst - 数据驱动游戏引擎");
    println!("  bevy - ECS游戏引擎");
}

/// 运行生态系统和工具示例
pub fn run_ecosystem_examples() {
    println!("=== 生态系统和工具示例 ===");
    cargo_usage();
    println!();
    crates_io();
    println!();
    rustup();
    println!();
    clippy();
    println!();
    rustfmt();
    println!();
    rustdoc();
    println!();
    benchmarking();
    println!();
    cross_compilation();
    println!();
    webassembly();
    println!();
    embedded();
    println!();
    gamedev();
}