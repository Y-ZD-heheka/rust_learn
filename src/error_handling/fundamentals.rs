//! 教学主题一：错误处理基础心智模型。
//!
//! 这一层聚焦学习者最先需要掌握的内容：
//! - `panic!` 与可恢复错误的区别
//! - `Result` 与模式匹配
//! - `?` 操作符与错误转换
//!
//! 这些示例会为后续的 IO、业务校验和外部服务错误处理打下基础。

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

/// 用于演示 `Result` 的基础数学错误。
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InvalidOperation(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "除数不能为零"),
            Self::InvalidOperation(message) => write!(f, "无效操作: {}", message),
        }
    }
}

/// 贯穿多个示例的应用层错误类型。
#[derive(Debug)]
pub(crate) enum AppError {
    Io(io::Error),
    ParseNumber {
        field: &'static str,
        input: String,
        source: std::num::ParseIntError,
    },
    MissingInput {
        field: &'static str,
    },
    Validation {
        field: &'static str,
        message: String,
    },
    ExternalService {
        service: &'static str,
        code: u16,
        message: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO错误: {}", error),
            Self::ParseNumber {
                field,
                input,
                source,
            } => write!(
                f,
                "字段 {} 的值 '{}' 无法解析为整数: {}",
                field, input, source
            ),
            Self::MissingInput { field } => write!(f, "缺少必填输入: {}", field),
            Self::Validation { field, message } => {
                write!(f, "字段 {} 未通过校验: {}", field, message)
            }
            Self::ExternalService {
                service,
                code,
                message,
            } => write!(f, "外部服务 {} 返回错误 {}: {}", service, code, message),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::ParseNumber { source, .. } => Some(source),
            Self::MissingInput { .. }
            | Self::Validation { .. }
            | Self::ExternalService { .. } => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl AppError {
    pub(crate) fn missing_input(field: &'static str) -> Self {
        Self::MissingInput { field }
    }

    pub(crate) fn parse_number(
        field: &'static str,
        input: impl Into<String>,
        source: std::num::ParseIntError,
    ) -> Self {
        Self::ParseNumber {
            field,
            input: input.into(),
            source,
        }
    }

    pub(crate) fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation {
            field,
            message: message.into(),
        }
    }

    pub(crate) fn external_service(
        service: &'static str,
        code: u16,
        message: impl Into<String>,
    ) -> Self {
        Self::ExternalService {
            service,
            code,
            message: message.into(),
        }
    }
}

pub(crate) fn log_demo_error(operation: &str, error: &dyn fmt::Display) {
    println!("⚠️ [{}] {}", operation, error);
}

pub(crate) fn log_demo_message(operation: &str, message: &str) {
    println!("📝 [{}] {}", operation, message);
}

fn unique_demo_path(label: &str) -> std::path::PathBuf {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    std::env::temp_dir().join(format!(
        "rust_learn_error_handling_{}_{}.txt",
        label, unique_id
    ))
}

pub(crate) fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// 演示现代 panic 处理（不可恢复错误）。
pub fn modern_panic_handling() {
    println!("💥 现代 Panic 处理：");

    let values = vec![1, 2, 3];
    let value_at_index = values.get(5).unwrap_or(&0);
    println!("安全获取索引 5 的值: {}", value_at_index);

    let config = std::env::var("APP_CONFIG").unwrap_or_else(|_| "default_config".to_string());
    println!("配置: {}", config);

    let positive_number = 5;
    debug_assert!(positive_number > 0, "数字必须为正数");
    println!("数字验证通过: {}", positive_number);
    println!("💡 提示: 只有真正不可恢复的问题才应考虑 panic");
}

/// 演示 `Result`、模式匹配与错误分支处理。
pub fn modern_result_handling() {
    println!("🔄 现代化 Result 处理：");

    fn divide(x: f64, y: f64) -> Result<f64, MathError> {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else if x < 0.0 && y < 0.0 {
            Err(MathError::InvalidOperation(
                "这里将负数除法视为演示性业务限制".to_string(),
            ))
        } else {
            Ok(x / y)
        }
    }

    match divide(10.0, 2.0) {
        Ok(value) => println!("✅ 除法结果: {}", value),
        Err(error) => println!("❌ 除法失败: {}", error),
    }

    let safe_result = match divide(10.0, 0.0) {
        Ok(value) => value,
        Err(error) => {
            log_demo_error("安全除法", &error);
            0.0
        }
    };

    println!("安全除法结果: {}", safe_result);
}

/// 演示 `?` 操作符与错误转换链。
pub fn modern_question_mark_patterns() {
    println!("🎯 现代化 `?` 操作符模式：");

    let test_path = unique_demo_path("question_mark");
    let test_content = "Test content for reading";

    if let Err(error) = std::fs::write(&test_path, test_content) {
        log_demo_error("准备示例文件", &error);
        println!("💡 当准备步骤本身失败时，应终止当前演示而不是使用 expect 直接崩溃");
        return;
    }

    log_demo_message(
        "准备示例文件",
        &format!("已创建临时文件: {}", test_path.display()),
    );

    match read_file_content(test_path.to_string_lossy().as_ref()) {
        Ok(content) => {
            println!("📖 文件内容: {}", content.trim());
            println!("✅ 基础文件读取流程完成");
        }
        Err(error) => {
            log_demo_error("读取临时文件", &error);
        }
    }

    if let Err(error) = std::fs::remove_file(&test_path) {
        log_demo_error("清理临时文件", &error);
    }

    fn complex_operation() -> Result<String, AppError> {
        let file_content = read_file_content("nonexistent.txt")?;
        let raw_number = file_content.trim();
        let number: i32 = raw_number
            .parse()
            .map_err(|source| AppError::parse_number("文件内容", raw_number, source))?;
        Ok(format!("解析得到数字: {}", number))
    }

    let complex_result = match complex_operation() {
        Ok(result) => result,
        Err(error) => {
            log_demo_error("复杂操作", &error);
            log_demo_message("复杂操作", "这里演示在边界处返回默认值，而不是在示例内部 panic");
            "默认值".to_string()
        }
    };

    println!("复杂操作结果: {}", complex_result);
}
