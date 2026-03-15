//! 教学主题一：被测对象、领域模型与基础断言。
//!
//! 这一层放置最常被其他测试风格复用的“被测对象”：
//! - 纯函数 `add_two`
//! - 输入校验 `validate_email`
//! - 领域模型 `User` 与 `UserManager`
//! - 面向集成测试入口的 `greeting`

/// 一个现代数学函数，用于测试。
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// 私有辅助函数，用于展示如何测试非公开实现细节。
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/// 邮箱校验失败的稳定错误类型。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EmailValidationError {
    #[error("邮箱长度至少为5个字符")]
    TooShort,
    #[error("邮箱必须且只能包含一个@符号")]
    InvalidAtCount,
    #[error("邮箱本地部分不能为空")]
    EmptyLocalPart,
    #[error("邮箱域名必须包含点号")]
    MissingDomainDot,
    #[error("邮箱域名不能以点号开头或结尾")]
    DomainStartsOrEndsWithDot,
    #[error("邮箱域名不能包含空标签")]
    EmptyDomainLabel,
}

/// 用户创建失败的稳定错误类型。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UserCreationError {
    #[error(transparent)]
    InvalidEmail(#[from] EmailValidationError),
    #[error("用户年龄必须大于等于13岁")]
    AgeTooYoung,
}

/// 用户管理器的稳定错误类型。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum UserManagerError {
    #[error("邮箱已存在")]
    DuplicateEmail,
}

/// 统一的邮箱校验规则。
///
/// 该函数保留布尔返回值，方便教学中的快速断言；
/// 如果需要查看失败原因，请使用 crate 内部共享的
/// `validate_email_with_error` 规则入口。
pub fn validate_email(email: &str) -> bool {
    validate_email_with_error(email).is_ok()
}

pub(crate) fn validate_email_with_error(email: &str) -> Result<(), EmailValidationError> {
    if email.len() < 5 {
        return Err(EmailValidationError::TooShort);
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err(EmailValidationError::InvalidAtCount);
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() {
        return Err(EmailValidationError::EmptyLocalPart);
    }

    if !domain.contains('.') {
        return Err(EmailValidationError::MissingDomainDot);
    }

    if domain.starts_with('.') || domain.ends_with('.') {
        return Err(EmailValidationError::DomainStartsOrEndsWithDot);
    }

    if domain.split('.').any(|label| label.is_empty()) {
        return Err(EmailValidationError::EmptyDomainLabel);
    }

    Ok(())
}

/// 现代化用户结构体。
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, email: String, age: u8) -> Result<Self, UserCreationError> {
        validate_email_with_error(&email)?;

        if age < 13 {
            return Err(UserCreationError::AgeTooYoung);
        }

        Ok(Self { name, email, age })
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn greet(&self) -> String {
        format!("你好，{}！", self.name)
    }
}

/// 现代化用户管理器。
#[derive(Debug, Default)]
pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserManagerError> {
        if self.users.iter().any(|existing| existing.email == user.email) {
            return Err(UserManagerError::DuplicateEmail);
        }

        self.users.push(user);
        Ok(())
    }

    pub fn find_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|user| user.email == email)
    }

    pub fn get_adult_users(&self) -> Vec<&User> {
        self.users.iter().filter(|user| user.is_adult()).collect()
    }

    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

/// 现代化集成测试辅助函数。
pub fn greeting(name: &str) -> String {
    format!("你好，{}！", name)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_add_two_positive() {
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(10), 12);
    }

    #[test]
    fn test_add_two_negative() {
        assert_eq!(add_two(-1), 1);
        assert_eq!(add_two(-5), -3);
    }

    #[test]
    fn test_internal_adder() {
        assert_eq!(internal_adder(2, 3), 5);
        assert_eq!(internal_adder(0, 0), 0);
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test@domain.org"));
        assert!(validate_email("user.name@sub.domain.com"));
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@domain"));
        assert!(!validate_email("user@.example.com"));
    }

    #[test]
    fn test_validate_email_with_error_details() {
        assert_eq!(
            validate_email_with_error("invalid"),
            Err(EmailValidationError::InvalidAtCount)
        );
        assert_eq!(
            validate_email_with_error("@domain.com"),
            Err(EmailValidationError::EmptyLocalPart)
        );
        assert_eq!(
            validate_email_with_error("user@domain"),
            Err(EmailValidationError::MissingDomainDot)
        );
    }

    #[test]
    fn test_user_creation() {
        let user = User::new("张三".to_string(), "zhangsan@example.com".to_string(), 25)
            .expect("Failed to create user for test");

        assert_eq!(user.name, "张三");
        assert_eq!(user.email, "zhangsan@example.com");
        assert_eq!(user.age, 25);
        assert!(user.is_adult());
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let result = User::new("李四".to_string(), "invalid-email".to_string(), 20);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            UserCreationError::InvalidEmail(EmailValidationError::InvalidAtCount)
        );
    }

    #[test]
    fn test_user_creation_minor() {
        let result = User::new("小明".to_string(), "xiaoming@example.com".to_string(), 12);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserCreationError::AgeTooYoung);
    }

    #[test]
    fn test_user_methods() {
        let user = User::new("王五".to_string(), "wangwu@example.com".to_string(), 30)
            .expect("Failed to create user for test");

        assert!(user.is_adult());
        assert_eq!(user.greet(), "你好，王五！");
    }

    #[test]
    #[should_panic]
    fn test_panic_case() {
        panic!("这个测试应该发生 panic");
    }

    #[test]
    fn test_result_handling() -> Result<(), &'static str> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("2+2 不等于 4")
        }
    }

    #[test]
    fn test_approximate_comparison() {
        let result: f64 = 0.1 + 0.2;
        assert!((result - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_string_operations() {
        let text = "Hello Rust 2024!";
        assert!(text.contains("Rust"));
        assert!(text.starts_with("Hello"));
        assert!(text.ends_with("2024!"));
    }

    #[test]
    fn test_option_handling() {
        let some_value = Some(42);
        let none_value: Option<i32> = None;

        assert_eq!(some_value.unwrap_or_default(), 42);
        assert_eq!(none_value.unwrap_or_default(), 0);
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_user_manager_performance() {
        let mut manager = UserManager::new();

        for i in 0..1000 {
            let user = User::new(
                format!("用户{}", i),
                format!("user{}@example.com", i),
                20 + (i % 50) as u8,
            )
            .expect("Failed to create user in performance test");

            manager.add_user(user).expect("Failed to add user");
        }

        assert_eq!(manager.user_count(), 1000);
        assert_eq!(manager.get_adult_users().len(), 1000);
    }
}
