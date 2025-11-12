//! # 热门Rust库使用案例模块
//!
//! 这个模块演示了Rust生态系统中最重要的热门库的实际使用案例。
//! 包括数据序列化、命令行解析、HTTP请求、错误处理、日志记录等。
//! 采用了现代化的Rust 2021/2024最佳实践。

use serde::{Deserialize, Serialize};
use serde_json::json;
use clap::Parser;
use reqwest;
use anyhow::{Result, Context, anyhow};
use thiserror::Error;
use tracing::{info, warn, error, debug, trace};
use tracing_subscriber;
use chrono::{DateTime, Utc, Duration};
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
    let json_string = serde_json::to_string_pretty(&user)
        .context("序列化用户数据失败")?;
    
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
    let user: User = serde_json::from_str(json_string)
        .context("反序列化JSON失败")?;
    
    println!("✅ 反序列化的用户数据:");
    println!("  ID: {}", user.id);
    println!("  姓名: {}", user.name);
    println!("  邮箱: {}", user.email);
    println!("  创建时间: {}", user.created_at);
    println!("  主题: {}", user.preferences.theme);
    println!("  语言: {}", user.preferences.language);
    println!("  通知: {}", if user.preferences.notifications { "启用" } else { "禁用" });
    
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
    
    let status = response.status();
    if status.is_success() {
        let data: serde_json::Value = response.json().await?;
        println!("✅ GET请求成功:");
        println!("  状态码: {}", status);
        println!("  响应: {}", serde_json::to_string_pretty(&data)?);
    } else {
        return Err(anyhow!("HTTP状态码错误: {}", status));
    }
    
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
    
    let post_status = post_response.status();
    if post_status.is_success() {
        let post_result: serde_json::Value = post_response.json().await?;
        println!("✅ POST请求成功:");
        println!("  状态码: {}", post_status);
        println!("  响应: {}", serde_json::to_string_pretty(&post_result)?);
    } else {
        return Err(anyhow!("POST请求状态码错误: {}", post_status));
    }
    
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
    let file_content = std::fs::read_to_string("不存在的文件.txt")
        .context("读取配置文件失败")?;
    
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

/// 演示现代化日志记录
pub fn demonstrate_tracing() {
    println!("📊 演示现代化日志记录:");
    
    // 不同级别的日志
    trace!("🔍 跟踪信息: 这是一条trace日志");
    debug!("🐛 调试信息: 这是一条debug日志");
    info!("ℹ️ 信息日志: 程序正在运行");
    warn!("⚠️ 警告日志: 发现潜在问题");
    error!("💥 错误日志: 发生了严重错误");
    
    // 带上下文的日志
    let user_id = 12345;
    let operation = "用户登录";
    
    info!(user_id, operation, "用户开始执行操作");
    
    // 模拟不同场景
    let scenarios = [
        ("用户注册", "成功"),
        ("密码验证", "成功"),
        ("权限检查", "警告: 权限不足"),
        ("数据保存", "错误: 数据库连接失败")
    ];
    
    for (action, result) in scenarios {
        match result {
            "成功" => info!(action, result, "操作完成"),
            "警告" => warn!(action, result, "需要关注"),
            "错误" => error!(action, result, "操作失败"),
            _ => debug!(action, result, "未知状态"),
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
    
    // 初始化日志
    if args.verbose {
        tracing_subscriber::fmt::init();
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
                println!("❌ HTTP操作需要URL参数");
            }
        }
        Operation::Cli => {
            demonstrate_cli_parsing(args);
        }
        Operation::Errors => {
            demonstrate_error_handling()?;
        }
        Operation::Tracing => {
            demonstrate_tracing();
            demonstrate_datetime_uuid();
        }
    }
    
    println!("\n✅ 热门库演示完成！");
    Ok(())
}

/// 带URL的HTTP请求演示
async fn demonstrate_http_requests_with_url(url: &str) -> Result<()> {
    println!("🌐 演示HTTP请求到: {}", url);
    
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    if response.status().is_success() {
        let text = response.text().await?;
        println!("✅ 响应内容 (前200字符):");
        println!("{}", &text.chars().take(200).collect::<String>());
        if text.len() > 200 {
            println!("... (内容已截断)");
        }
    } else {
        println!("❌ HTTP错误: {}", response.status());
    }
    
    Ok(())
}