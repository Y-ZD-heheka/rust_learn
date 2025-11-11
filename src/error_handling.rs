//! # 错误处理模块
//!
//! 这个模块演示了Rust的错误处理机制，包括可恢复错误和不可恢复错误。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fs::File;
use std::io::{self, Read};
use std::fmt;

/// 演示现代panic处理（不可恢复错误）
pub fn modern_panic_handling() {
    println!("💥 现代Panic处理：");
    
    // 使用unwrap_or_else进行安全unwrap
    let v = vec![1, 2, 3];
    let value_at_index = v.get(5).unwrap_or(&0); // 安全获取
    println!("安全获取索引5的值: {}", value_at_index);
    
    // 使用unwrap_or替代expect
    let config = std::env::var("APP_CONFIG").unwrap_or_else(|_| "default_config".to_string());
    println!("配置: {}", config);
    
    // 现代panic处理：debug_assert用于开发时
    let positive_number = -5;
    debug_assert!(positive_number >= 0, "数字必须为正数");
    println!("数字验证通过: {}", positive_number);
}

/// 现代化数学错误类型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    InvalidOperation(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "除数不能为零"),
            Self::InvalidOperation(msg) => write!(f, "无效操作: {}", msg),
        }
    }
}

/// 现代化应用错误类型
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    Custom { message: String },
    Network { code: u16, message: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO错误: {}", err),
            Self::Parse(err) => write!(f, "解析错误: {}", err),
            Self::Custom { message } => write!(f, "自定义错误: {}", message),
            Self::Network { code, message } => write!(f, "网络错误 {}: {}", code, message),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::Parse(err)
    }
}

impl AppError {
    fn network_error(code: u16, message: &str) -> Self {
        Self::Network { code, message: message.to_string() }
    }
    
    fn custom_error(message: &str) -> Self {
        Self::Custom { message: message.to_string() }
    }
}

/// 演示现代化Result类型和模式匹配
pub fn modern_result_handling() {
    println!("🔄 现代化Result处理：");
    
    fn divide(x: f64, y: f64) -> Result<f64, MathError> {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else if x < 0.0 && y < 0.0 {
            Err(MathError::InvalidOperation("负数除法".to_string()))
        } else {
            Ok(x / y)
        }
    }
    
    // 使用let_else进行现代化Result处理
    let result = divide(10.0, 2.0);
    if let Ok(value) = result {
        println!("✅ 除法结果: {}", value);
    } else {
        println!("❌ 除法失败");
    }
    
    // 使用现代化map_err和unwrap_or
    let safe_result = divide(10.0, 0.0)
        .map_err(|e| {
            println!("❌ 错误: {:?}", e);
            e
        })
        .unwrap_or(0.0); // 提供默认值
    
    println!("安全除法结果: {}", safe_result);
}

/// 演示现代化?操作符使用模式
pub fn modern_question_mark_patterns() {
    println!("🎯 现代化?操作符模式：");
    
    // 创建测试文件
    let test_content = "Test content for reading";
    std::fs::write("test_file.txt", test_content).unwrap();
    
    fn read_file_content(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
    
    // 现代化错误处理链
    let content = read_file_content("test_file.txt")
        .map(|c| {
            println!("📖 文件内容: {}", c.trim());
            c
        })
        .map_err(|e| {
            eprintln!("⚠️ 文件读取错误: {}", e);
            e
        });
    
    // 清理测试文件
    let _ = std::fs::remove_file("test_file.txt");
    
    // 现代化的链式错误处理
    fn complex_operation() -> Result<String, AppError> {
        let file_content = read_file_content("nonexistent.txt")?;
        let number: i32 = file_content.trim().parse()
            .map_err(|_| AppError::custom_error("数字解析失败"))?;
        Ok(format!("解析得到数字: {}", number))
    }
    
    let complex_result = complex_operation()
        .unwrap_or_else(|e| {
            eprintln!("复杂操作失败: {}", e);
            "默认值".to_string()
        });
    
    println!("复杂操作结果: {}", complex_result);
}

/// 演示现代错误类型设计
pub fn modern_error_types() {
    println!("🎨 现代错误类型设计：");
    
    // 演示错误处理
    fn process_data(input: &str) -> Result<i32, AppError> {
        if input.is_empty() {
            return Err(AppError::custom_error("输入为空"));
        }
        
        let number: i32 = input.parse()?;
        if number < 0 {
            return Err(AppError::network_error(400, "负数不被允许"));
        }
        
        Ok(number * 2)
    }
    
    // 现代化错误处理链
    let test_cases = vec!["42", "invalid", "", "-5"];
    
    for case in test_cases {
        match process_data(case) {
            Ok(result) => println!("✅ '{}' -> {}", case, result),
            Err(AppError::Network { code, message }) =>
                println!("❌ '{}' -> 网络错误 {}: {}", case, code, message),
            Err(AppError::Parse { .. }) =>
                println!("❌ '{}' -> 解析错误", case),
            Err(AppError::Custom { message }) =>
                println!("❌ '{}' -> 自定义错误: {}", case, message),
            Err(AppError::Io { .. }) =>
                println!("❌ '{}' -> IO错误", case),
        }
    }
}

/// 演示现代错误恢复策略
pub fn modern_error_recovery() {
    println!("🔧 现代错误恢复策略：");
    
    // 模拟多个可能失败的操作
    fn unreliable_operation(id: u32) -> Result<String, &'static str> {
        match id {
            1..=3 => Ok(format!("操作{}成功", id)),
            4..=5 => Err("临时失败"),
            _ => Err("永久失败"),
        }
    }
    
    // 现代化重试机制
    fn with_retry<T, F>(max_retries: usize, operation: F) -> Result<T, String>
    where
        F: Fn() -> Result<T, &'static str>,
    {
        let mut last_error = String::new();
        
        for attempt in 1..=max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = format!("{}", e);
                    if attempt < max_retries {
                        println!("重试第{}次，上次失败: {}", attempt + 1, e);
                    }
                }
            }
        }
        
        Err(format!("所有重试都失败了: {}", last_error))
    }
    
    // 演示重试机制
    for id in [1, 4, 6, 8] {
        match with_retry(3, || unreliable_operation(id)) {
            Ok(result) => println!("✅ 操作结果: {}", result),
            Err(e) => println!("❌ 最终失败: {}", e),
        }
    }
}

/// 演示现代错误日志记录
pub fn modern_error_logging() {
    println!("📝 现代错误日志记录：");
    
    // 使用现代日志记录（模拟实现）
    fn log_error(operation: &str, error: &dyn fmt::Display) {
        eprintln!("[ERROR] {} failed: {}", operation, error);
    }
    
    // 模拟错误日志记录
    let operations = ["网络请求", "数据库操作", "文件读取"];
    
    for op in operations {
        log_error(op, &"连接超时");
        println!("⚠️ {} 操作失败，已记录错误日志", op);
    }
}

/// 运行错误处理示例
pub fn run_error_handling_examples() {
    println!("🎯 === 现代化错误处理示例 ===");
    println!();
    
    modern_panic_handling();
    println!();
    
    modern_result_handling();
    println!();
    
    modern_question_mark_patterns();
    println!();
    
    modern_error_types();
    println!();
    
    modern_error_recovery();
    println!();
    
    modern_error_logging();
    
    println!("\n✅ 所有错误处理示例运行完成！");
}