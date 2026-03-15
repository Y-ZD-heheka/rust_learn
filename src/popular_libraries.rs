//! # 热门Rust库使用案例模块
//!
//! 这个模块演示了Rust生态系统中最重要的热门库的实际使用案例。
//! 包括数据序列化、命令行解析、HTTP请求、错误处理、日志记录等。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{self, EnvFilter};
use uuid::Uuid;

/// 现代化结构体定义（使用Serde特性）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub preferences: UserPreferences,
}

/// 用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications: bool,
}

/// 命令行参数结构体
#[derive(Parser, Debug)]
#[command(name = "rust-popular-libs")]
#[command(about = "演示Rust热门库的使用案例")]
#[command(version = "1.0")]
pub struct CliArgs {
    /// 要执行的操作 (serialize, deserialize, http, cli, errors, tracing)
    #[arg(value_enum)]
    pub operation: Operation,

    /// 启用详细输出
    #[arg(short, long)]
    pub verbose: bool,

    /// HTTP请求的URL（用于http操作）
    #[arg(short, long)]
    pub url: Option<String>,
}

/// 操作类型枚举
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Operation {
    /// 演示序列化操作
    Serialize,
    /// 演示反序列化操作
    Deserialize,
    /// 演示HTTP请求操作
    Http,
    /// 演示命令行解析
    Cli,
    /// 演示错误处理
    Errors,
    /// 演示日志记录
    Tracing,
}

/// 自定义错误类型
#[derive(Error, Debug)]
pub enum LibraryError {
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("网络请求错误: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("无效的JSON: {0}")]
    InvalidJson(String),

    #[error("API返回错误状态码: {0}")]
    HttpStatus(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TracingInitState {
    Initialized { verbose_profile: bool },
    ExternalSubscriber,
}

static TRACING_INIT_STATE: OnceLock<TracingInitState> = OnceLock::new();

fn ensure_tracing_initialized(verbose: bool) -> TracingInitState {
    let state = *TRACING_INIT_STATE.get_or_init(|| {
        let default_filter = if verbose { "trace" } else { "info" };
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

        match tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(false)
            .try_init()
        {
            Ok(()) => TracingInitState::Initialized {
                verbose_profile: verbose,
            },
            Err(_) => TracingInitState::ExternalSubscriber,
        }
    });

    match state {
        TracingInitState::Initialized {
            verbose_profile: false,
        } if verbose => println!(
            "ℹ️ tracing 已按普通模式初始化；本次不会重复初始化，因此 debug/trace 日志可能仍被过滤。"
        ),
        TracingInitState::ExternalSubscriber => println!(
            "ℹ️ tracing 已由外部订阅器初始化；以下日志输出级别取决于当前全局配置。"
        ),
        _ => {}
    }

    state
}

fn truncate_for_output(content: &str, max_chars: usize) -> String {
    let trimmed = content.trim();
    let mut truncated = trimmed.chars().take(max_chars).collect::<String>();
    if trimmed.chars().count() > max_chars {
        truncated.push_str("...");
    }
    truncated
}

async fn ensure_success_response(
    response: reqwest::Response,
    operation: &str,
) -> Result<reqwest::Response> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let body = response
        .text()
        .await
        .unwrap_or_else(|error| format!("<读取错误响应体失败: {}>", error));
    let body_snippet = truncate_for_output(&body, 200);
    let visible_body = if body_snippet.is_empty() {
        "<空响应体>".to_string()
    } else {
        body_snippet
    };

    error!(operation, status = %status, body = %visible_body, "HTTP 请求失败");
    Err(anyhow!(
        "{} 返回非成功状态 {}，响应片段: {}",
        operation,
        status,
        visible_body
    ))
}

/// 演示现代化Serde序列化
pub fn demonstrate_serde_serialization() -> Result<()> {
    println!("🔄 演示现代化Serde序列化:");

    // 创建用户数据
    let user = User {
        id: 1,
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        created_at: Utc::now(),
        preferences: UserPreferences {
            theme: "dark".to_string(),
            language: "zh-CN".to_string(),
            notifications: true,
        },
    };

    // 序列化为JSON
    let json_string = serde_json::to_string_pretty(&user).context("序列化用户数据失败")?;

    println!("📄 序列化的JSON:");
    println!("{}", json_string);

    // 创建复杂的嵌套结构
    let user_data = json!({
        "users": [
            user,
            {
                "id": 2,
                "name": "李四",
                "email": "lisi@example.com",
                "created_at": Utc::now(),
                "preferences": {
                    "theme": "light",
                    "language": "en-US",
                    "notifications": false
                }
            }
        ],
        "metadata": {
            "version": "1.0",
            "timestamp": Utc::now(),
            "total_users": 2
        }
    });

    println!("\n📊 复杂结构序列化:");
    println!("{}", serde_json::to_string_pretty(&user_data)?);

    Ok(())
}

/// 演示现代化Serde反序列化
pub fn demonstrate_serde_deserialization() -> Result<()> {
    println!("🔄 演示现代化Serde反序列化:");

    // JSON字符串
    let json_string = r#"{
        "id": 42,
        "name": "王五",
        "email": "wangwu@example.com",
        "created_at": "2024-01-15T10:30:00Z",
        "preferences": {
            "theme": "auto",
            "language": "zh-CN",
            "notifications": true
        }
    }"#;

    // 反序列化
    let user: User = serde_json::from_str(json_string).context("反序列化JSON失败")?;

    println!("✅ 反序列化的用户数据:");
    println!("  ID: {}", user.id);
    println!("  姓名: {}", user.name);
    println!("  邮箱: {}", user.email);
    println!("  创建时间: {}", user.created_at);
    println!("  主题: {}", user.preferences.theme);
    println!("  语言: {}", user.preferences.language);
    println!(
        "  通知: {}",
        if user.preferences.notifications {
            "启用"
        } else {
            "禁用"
        }
    );

    Ok(())
}

/// 演示现代化HTTP请求
pub async fn demonstrate_http_requests() -> Result<()> {
    println!("🌐 演示现代化HTTP请求:");

    let client = reqwest::Client::new();

    // GET请求示例
    println!("📡 发送GET请求...");
    let response = client
        .get("https://httpbin.org/get")
        .header("User-Agent", "Rust Popular Libraries Demo")
        .send()
        .await
        .context("发送GET请求失败")?;
    let response = ensure_success_response(response, "GET https://httpbin.org/get").await?;
    let status = response.status();
    let data: serde_json::Value = response
        .json()
        .await
        .context("解析GET响应JSON失败")?;
    println!("✅ GET请求成功:");
    println!("  状态码: {}", status);
    println!("  响应: {}", serde_json::to_string_pretty(&data)?);

    // POST请求示例
    println!("\n📤 发送POST请求...");
    let post_data = json!({
        "name": "测试用户",
        "email": "test@example.com",
        "timestamp": Utc::now()
    });

    let post_response = client
        .post("https://httpbin.org/post")
        .json(&post_data)
        .send()
        .await
        .context("发送POST请求失败")?;
    let post_response =
        ensure_success_response(post_response, "POST https://httpbin.org/post").await?;
    let post_status = post_response.status();
    let post_result: serde_json::Value = post_response
        .json()
        .await
        .context("解析POST响应JSON失败")?;
    println!("✅ POST请求成功:");
    println!("  状态码: {}", post_status);
    println!("  响应: {}", serde_json::to_string_pretty(&post_result)?);

    Ok(())
}

/// 演示现代化错误处理
pub fn demonstrate_error_handling() -> Result<()> {
    println!("🚨 演示现代化错误处理:");

    // 使用Anyhow进行错误处理
    fn risky_operation(use_value: bool) -> Result<String> {
        if use_value {
            Ok("操作成功".to_string())
        } else {
            Err(anyhow!("操作失败: 参数无效"))
        }
    }

    // 成功情况
    match risky_operation(true) {
        Ok(result) => println!("✅ 成功结果: {}", result),
        Err(e) => println!("❌ 错误: {}", e),
    }

    // 失败情况
    match risky_operation(false) {
        Ok(result) => println!("✅ 成功结果: {}", result),
        Err(e) => println!("❌ 错误: {}", e),
    }

    // 使用context添加额外信息
    let file_content = std::fs::read_to_string("不存在的文件.txt").context("读取配置文件失败")?;

    println!("📄 文件内容: {}", file_content);

    Ok(())
}

/// 演示现代化命令行解析
pub fn demonstrate_cli_parsing(args: &CliArgs) {
    println!("⚡ 演示现代化命令行解析:");
    println!("  操作: {:?}", args.operation);
    println!("  详细模式: {}", args.verbose);

    if let Some(ref url) = args.url {
        println!("  目标URL: {}", url);
    }

    match args.operation {
        Operation::Cli => {
            println!("💡 CLI操作已选择");
            println!("  这是一个CLI示例");
        }
        Operation::Http => {
            if let Some(ref url) = args.url {
                println!("🌐 HTTP操作: {}", url);
            } else {
                println!("❌ HTTP操作需要URL参数");
            }
        }
        _ => println!("📋 其他操作"),
    }
}

/// 演示现代化日志记录。
///
/// 该示例会确保 tracing 只初始化一次，并明确说明当前采用的是普通日志配置、详细日志配置，
/// 还是沿用外部已存在的全局订阅器。
pub fn demonstrate_tracing(verbose: bool) {
    println!("📊 演示现代化日志记录:");

    let init_state = ensure_tracing_initialized(verbose);
    match init_state {
        TracingInitState::Initialized {
            verbose_profile: true,
        } => println!("📝 当前日志配置: 详细模式（默认显示 trace/debug/info/warn/error）"),
        TracingInitState::Initialized {
            verbose_profile: false,
        } => println!("📝 当前日志配置: 普通模式（默认显示 info/warn/error）"),
        TracingInitState::ExternalSubscriber => {
            println!("📝 当前日志配置: 由外部订阅器决定，示例仅负责产生日志事件")
        }
    }

    // 不同级别的日志
    trace!("🔍 跟踪信息: 这是一条 trace 日志");
    debug!("🐛 调试信息: 这是一条 debug 日志");
    info!("ℹ️ 信息日志: 程序正在运行");
    warn!("⚠️ 警告日志: 发现潜在问题");
    error!("💥 错误日志: 发生了严重错误");

    // 带上下文的日志
    let user_id = 12345;
    let operation = "用户登录";

    info!(user_id, operation, "用户开始执行操作");

    #[derive(Clone, Copy)]
    enum ScenarioOutcome {
        Success,
        Warning,
        Error,
    }

    // 模拟不同场景
    let scenarios = [
        ("用户注册", "注册成功", ScenarioOutcome::Success),
        ("密码验证", "密码验证通过", ScenarioOutcome::Success),
        ("权限检查", "权限不足", ScenarioOutcome::Warning),
        ("数据保存", "数据库连接失败", ScenarioOutcome::Error),
    ];

    for (action, detail, outcome) in scenarios {
        match outcome {
            ScenarioOutcome::Success => info!(action, detail, "操作完成"),
            ScenarioOutcome::Warning => warn!(action, detail, "需要关注"),
            ScenarioOutcome::Error => error!(action, detail, "操作失败"),
        }
    }
}

/// 演示UUID和日期时间操作
pub fn demonstrate_datetime_uuid() {
    println!("🕐 演示日期时间和UUID:");

    // UUID生成
    let user_uuid = Uuid::new_v4();
    let session_uuid = Uuid::new_v4();

    println!("🔑 用户UUID: {}", user_uuid);
    println!("🔑 会话UUID: {}", session_uuid);

    // 日期时间操作
    let now = Utc::now();
    let yesterday = now - Duration::days(1);
    let tomorrow = now + Duration::days(1);

    println!("📅 现在: {}", now);
    println!("📅 昨天: {}", yesterday);
    println!("📅 明天: {}", tomorrow);

    // 格式化
    let formatted_date = now.format("%Y-%m-%d %H:%M:%S");
    let iso_date = now.to_rfc3339();

    println!("📝 格式化日期: {}", formatted_date);
    println!("📝 ISO格式: {}", iso_date);

    // 时间计算
    let duration = tomorrow - yesterday;
    println!("⏰ 时间间隔: {} 小时", duration.num_hours());
}

/// 运行热门库演示
pub async fn run_popular_libraries_demo(args: &CliArgs) -> Result<()> {
    println!("🎯 === Rust热门库演示 ===");
    println!();

    if args.verbose || matches!(args.operation, Operation::Tracing) {
        let _ = ensure_tracing_initialized(args.verbose);
    }

    match args.operation {
        Operation::Serialize => {
            demonstrate_serde_serialization()?;
        }
        Operation::Deserialize => {
            demonstrate_serde_deserialization()?;
        }
        Operation::Http => {
            if let Some(ref url) = args.url {
                demonstrate_http_requests_with_url(url).await?;
            } else {
                return Err(anyhow!("HTTP操作需要URL参数"));
            }
        }
        Operation::Cli => {
            demonstrate_cli_parsing(args);
        }
        Operation::Errors => {
            demonstrate_error_handling()?;
        }
        Operation::Tracing => {
            demonstrate_tracing(args.verbose);
            demonstrate_datetime_uuid();
        }
    }

    println!("\n✅ 热门库演示完成！");
    Ok(())
}

/// 带URL的HTTP请求演示。
///
/// 对非 2xx 响应会返回错误，而不是仅打印后继续伪装成成功。
async fn demonstrate_http_requests_with_url(url: &str) -> Result<()> {
    println!("🌐 演示HTTP请求到: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("发送请求失败: {}", url))?;
    let response = ensure_success_response(response, &format!("GET {}", url)).await?;

    let text = response
        .text()
        .await
        .with_context(|| format!("读取响应体失败: {}", url))?;
    println!("✅ 响应内容 (前200字符):");
    println!("{}", truncate_for_output(&text, 200));
    if text.chars().count() > 200 {
        println!("... (内容已截断)");
    }

    Ok(())
}

/// 演示所有库的完整示例
pub fn run_popular_libraries_examples() {
    println!("🎯 === Rust热门库完整演示 ===");
    println!();

    // 1. Serde序列化/反序列化
    println!("════════════════════════════════════════");
    println!("1️⃣ Serde - 数据序列化");
    println!("════════════════════════════════════════");
    if let Err(e) = demonstrate_serde_serialization() {
        eprintln!("❌ Serde演示错误: {}", e);
    }

    println!("\n");

    // 2. 错误处理示例
    println!("════════════════════════════════════════");
    println!("2️⃣ Anyhow & Thiserror - 错误处理");
    println!("════════════════════════════════════════");
    if let Err(e) = demonstrate_error_handling() {
        println!("📝 捕获的错误: {}", e);
    }

    println!("\n");

    // 3. 日期时间和UUID
    println!("════════════════════════════════════════");
    println!("3️⃣ Chrono & UUID - 时间和标识符");
    println!("════════════════════════════════════════");
    demonstrate_datetime_uuid();

    println!("\n");

    // 4. JSON处理
    println!("════════════════════════════════════════");
    println!("4️⃣ Serde_json - JSON处理示例");
    println!("════════════════════════════════════════");
    demonstrate_json_processing();

    println!("\n");

    // 5. CLI参数示例
    println!("════════════════════════════════════════");
    println!("5️⃣ Clap - 命令行参数解析");
    println!("════════════════════════════════════════");
    let example_args = CliArgs {
        operation: Operation::Serialize,
        verbose: true,
        url: Some("https://example.com".to_string()),
    };
    demonstrate_cli_parsing(&example_args);

    println!("\n");

    // 6. 日志系统
    println!("════════════════════════════════════════");
    println!("6️⃣ Tracing - 日志和追踪");
    println!("════════════════════════════════════════");
    demonstrate_tracing(true);

    println!("\n✅ 所有热门库示例演示完成！");
}

/// 演示JSON处理
fn demonstrate_json_processing() {
    println!("📋 演示JSON处理:");

    // 创建复杂JSON对象
    let user_list = json!([
        {
            "id": 1,
            "name": "张三",
            "age": 28,
            "skills": ["Rust", "Go", "Python"],
            "active": true
        },
        {
            "id": 2,
            "name": "李四",
            "age": 32,
            "skills": ["Java", "C++", "C#"],
            "active": false
        }
    ]);

    println!("📝 原始JSON:");
    println!("{}", serde_json::to_string_pretty(&user_list).unwrap());

    // 访问JSON数据
    if let Some(arr) = user_list.as_array() {
        println!("\n👥 用户列表访问:");
        for (idx, user) in arr.iter().enumerate() {
            if let Some(name) = user.get("name").and_then(|v| v.as_str()) {
                if let Some(age) = user.get("age").and_then(|v| v.as_u64()) {
                    println!("  [{}] {} - 年龄: {}", idx + 1, name, age);
                }
            }
        }
    }

    // JSON修改
    let mut modified_json = user_list.clone();
    if let Some(obj) = modified_json.get_mut(0).and_then(|v| v.as_object_mut()) {
        obj.insert("status".to_string(), json!("升级"));
    }

    println!("\n✏️ 修改后的JSON:");
    println!("{}", serde_json::to_string_pretty(&modified_json).unwrap());
}
