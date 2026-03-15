//! 教学主题三：业务校验、自定义错误与恢复策略。
//!
//! 这一层强调“进入业务代码后，错误如何表达得更贴近领域”：
//! - 自定义错误枚举
//! - 校验失败时的用户友好提示
//! - 日志与重试等恢复策略

use std::fmt;

use super::fundamentals::{log_demo_error, log_demo_message, AppError};

/// 演示现代错误类型设计。
pub fn modern_error_types() {
    println!("🎨 现代错误类型设计：");

    fn process_data(input: &str) -> Result<i32, AppError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(AppError::missing_input("input"));
        }

        let number: i32 = trimmed
            .parse()
            .map_err(|source| AppError::parse_number("input", trimmed, source))?;
        if number < 0 {
            return Err(AppError::validation("input", "负数不被允许"));
        }

        Ok(number * 2)
    }

    let test_cases = ["42", "invalid", "", "-5"];

    for case in test_cases {
        match process_data(case) {
            Ok(result) => println!("✅ '{}' -> {}", case, result),
            Err(AppError::MissingInput { field }) => {
                println!("❌ '{}' -> 缺少输入字段: {}", case, field)
            }
            Err(AppError::ParseNumber { field, .. }) => {
                println!("❌ '{}' -> 字段 {} 解析失败", case, field)
            }
            Err(AppError::Validation { field, message }) => {
                println!("❌ '{}' -> 字段 {} 校验失败: {}", case, field, message)
            }
            Err(AppError::Io(_)) => println!("❌ '{}' -> IO 错误", case),
            Err(AppError::ExternalService {
                service,
                code,
                message,
            }) => {
                println!("❌ '{}' -> 外部服务 {} 错误 {}: {}", case, service, code, message)
            }
        }
    }
}

/// 演示现代错误恢复策略。
pub fn modern_error_recovery() {
    println!("🔧 现代错误恢复策略：");

    #[derive(Debug, Clone, Copy)]
    enum RetryError {
        TemporaryUnavailable,
        PermanentDenied,
    }

    impl fmt::Display for RetryError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::TemporaryUnavailable => write!(f, "临时失败"),
                Self::PermanentDenied => write!(f, "永久失败"),
            }
        }
    }

    fn unreliable_operation(id: u32) -> Result<String, RetryError> {
        match id {
            1..=3 => Ok(format!("操作 {} 成功", id)),
            4..=5 => Err(RetryError::TemporaryUnavailable),
            _ => Err(RetryError::PermanentDenied),
        }
    }

    fn with_retry<T, F>(max_retries: usize, operation_name: &str, operation: F) -> Result<T, RetryError>
    where
        F: Fn() -> Result<T, RetryError>,
    {
        let mut last_error = RetryError::PermanentDenied;

        for attempt in 1..=max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = error;
                    if attempt < max_retries {
                        log_demo_message(
                            operation_name,
                            &format!("准备第 {} 次尝试，上次失败: {}", attempt + 1, error),
                        );
                    }
                }
            }
        }

        Err(last_error)
    }

    for id in [1, 4, 6, 8] {
        let operation_name = format!("重试任务 #{id}");
        match with_retry(3, &operation_name, || unreliable_operation(id)) {
            Ok(result) => println!("✅ 操作结果: {}", result),
            Err(error) => {
                log_demo_error(&operation_name, &error);
                match error {
                    RetryError::TemporaryUnavailable => {
                        println!("💡 适合继续退避重试或切换到备用实例")
                    }
                    RetryError::PermanentDenied => {
                        println!("💡 应停止重试并将失败返回给调用方")
                    }
                }
            }
        }
    }
}

/// 演示现代错误日志记录。
pub fn modern_error_logging() {
    println!("📝 现代错误日志记录：");

    #[derive(Debug)]
    struct OperationFailure {
        operation: &'static str,
        error: AppError,
    }

    let failures = [
        OperationFailure {
            operation: "网络请求",
            error: AppError::external_service("profile-api", 504, "连接超时"),
        },
        OperationFailure {
            operation: "数据库操作",
            error: AppError::validation("user_id", "记录不存在，无法更新"),
        },
        OperationFailure {
            operation: "文件读取",
            error: AppError::missing_input("config_path"),
        },
    ];

    for failure in failures {
        log_demo_error(failure.operation, &failure.error);
        println!("⚠️ {} 操作失败，已使用统一示例日志口径输出", failure.operation);
    }
}

/// 演示业务逻辑验证错误处理。
pub fn business_validation_error_handling() {
    println!("🔍 业务逻辑验证错误处理：");

    #[derive(Debug)]
    enum ValidationError {
        EmptyField(String),
        InvalidFormat { field: String, format: String },
        ValueOutOfRange {
            field: String,
            min: i32,
            max: i32,
            actual: i32,
        },
        DuplicateEntry(String),
        ForbiddenAction(String),
    }

    impl fmt::Display for ValidationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::EmptyField(field) => write!(f, "字段不能为空: {}", field),
                Self::InvalidFormat { field, format } => {
                    write!(f, "字段 {} 格式不正确，期望: {}", field, format)
                }
                Self::ValueOutOfRange {
                    field,
                    min,
                    max,
                    actual,
                } => write!(
                    f,
                    "字段 {} 值超出范围 [{}, {}]，实际: {}",
                    field, min, max, actual
                ),
                Self::DuplicateEntry(entry) => write!(f, "重复条目: {}", entry),
                Self::ForbiddenAction(action) => write!(f, "禁止操作: {}", action),
            }
        }
    }

    fn validate_user_registration(
        username: &str,
        email: &str,
        age: i32,
        existing_users: &[&str],
        is_admin_action: bool,
    ) -> Result<(), ValidationError> {
        if username.trim().is_empty() {
            return Err(ValidationError::EmptyField("用户名".to_string()));
        }

        if username.len() < 3 || username.len() > 20 {
            return Err(ValidationError::ValueOutOfRange {
                field: "用户名长度".to_string(),
                min: 3,
                max: 20,
                actual: username.len() as i32,
            });
        }

        if existing_users.contains(&username) {
            return Err(ValidationError::DuplicateEntry("用户名".to_string()));
        }

        if email.trim().is_empty() {
            return Err(ValidationError::EmptyField("邮箱".to_string()));
        }

        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidFormat {
                field: "邮箱".to_string(),
                format: "必须包含 @ 和域名".to_string(),
            });
        }

        if age < 13 || age > 120 {
            return Err(ValidationError::ValueOutOfRange {
                field: "年龄".to_string(),
                min: 13,
                max: 120,
                actual: age,
            });
        }

        if is_admin_action {
            return Err(ValidationError::ForbiddenAction(
                "当前演示用户不能直接执行管理员审批".to_string(),
            ));
        }

        Ok(())
    }

    let existing_users = ["john_doe", "jane_smith"];
    let test_cases = [
        ("", "john@example.com", 25, false),
        ("jo", "jane@example.com", 25, false),
        ("john_doe", "john@example.com", 25, false),
        ("alice", "invalid-email", 25, false),
        ("alice", "alice@example.com", 10, false),
        ("alice", "alice@example.com", 25, true),
        ("alice", "alice@example.com", 25, false),
    ];

    for (index, (username, email, age, is_admin_action)) in test_cases.iter().enumerate() {
        println!(
            "测试用例 {}: ({}, {}, {}, admin={})",
            index + 1,
            username,
            email,
            age,
            is_admin_action
        );

        match validate_user_registration(username, email, *age, &existing_users, *is_admin_action) {
            Ok(()) => println!("✅ 注册验证通过"),
            Err(error) => {
                log_demo_error("注册验证", &error);
                match error {
                    ValidationError::EmptyField(_) => {
                        println!("💡 请填写所有必需字段");
                    }
                    ValidationError::ValueOutOfRange { field, .. } => {
                        println!("💡 请检查 {} 的取值范围", field);
                    }
                    ValidationError::DuplicateEntry(_) => {
                        println!("💡 该用户名已被使用，请选择其他用户名");
                    }
                    ValidationError::InvalidFormat { .. } => {
                        println!("💡 请输入有效的邮箱地址");
                    }
                    ValidationError::ForbiddenAction(_) => {
                        println!("💡 您没有权限执行此操作");
                    }
                }
            }
        }

        println!();
    }
}
