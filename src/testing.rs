//! # 测试和文档模块
//!
//! 这个模块演示了Rust的测试功能和文档生成。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fmt;

/// 一个现代数学函数，用于测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// 另一个现代函数，用于测试私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/// 现代化验证函数
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.len() > 5
}

/// 现代化用户结构体
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u8,
}

impl User {
    pub fn new(name: String, email: String, age: u8) -> Result<Self, String> {
        if !validate_email(&email) {
            return Err("邮箱格式不正确".to_string());
        }
        if age < 13 {
            return Err("用户年龄必须大于等于13岁".to_string());
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

/// 现代化单元测试
#[cfg(test)]
mod tests {
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
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@domain.com"));
    }

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "张三".to_string(),
            "zhangsan@example.com".to_string(),
            25
        ).unwrap();
        
        assert_eq!(user.name, "张三");
        assert_eq!(user.email, "zhangsan@example.com");
        assert_eq!(user.age, 25);
        assert!(user.is_adult());
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let result = User::new(
            "李四".to_string(),
            "invalid-email".to_string(),
            20
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "邮箱格式不正确");
    }

    #[test]
    fn test_user_creation_minor() {
        let result = User::new(
            "小明".to_string(),
            "xiaoming@example.com".to_string(),
            12
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户年龄必须大于等于13岁");
    }

    #[test]
    fn test_user_methods() {
        let user = User::new(
            "王五".to_string(),
            "wangwu@example.com".to_string(),
            30
        ).unwrap();
        
        assert!(user.is_adult());
        assert_eq!(user.greet(), "你好，王五！");
    }

    #[test]
    #[should_panic]
    fn test_panic_case() {
        panic!("这个测试应该发生 panic");
    }

    #[test]
    fn test_result_handling() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("2+2 不等于 4".to_string())
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

/// 现代化集成测试辅助函数
pub fn greeting(name: &str) -> String {
    format!("你好，{}！", name)
}

/// 现代化用户管理器
pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
    
    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        // 检查邮箱是否已存在
        if self.users.iter().any(|u| u.email == user.email) {
            return Err("邮箱已存在".to_string());
        }
        
        self.users.push(user);
        Ok(())
    }
    
    pub fn find_user_by_email(&self, email: &str) -> Option<&User> {
        self.users.iter().find(|u| u.email == email)
    }
    
    pub fn get_adult_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.is_adult()).collect()
    }
    
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

/// 现代化文档测试
///
/// # 示例
///
/// ```
/// let result = rust_learn::testing::greeting("小王");
/// assert_eq!(result, "你好，小王！");
///
/// let user = rust_learn::testing::User::new(
///     "小李".to_string(),
///     "xiaoli@example.com".to_string(),
///     25
/// ).unwrap();
///
/// assert_eq!(user.name, "小李");
/// assert!(user.is_adult());
/// assert_eq!(user.greet(), "你好，小李！");
/// ```
pub fn documented_function() {
    println!("这个函数包含文档测试示例");
}

/// 现代化基准测试示例
pub fn benchmark_operations() {
    println!("🧪 现代化基准测试操作：");
    
    // 大量数据处理示例
    let large_data: Vec<i32> = (1..10000).collect();
    
    // 过滤器基准
    let even_count = large_data.iter().filter(|&&x| x % 2 == 0).count();
    println!("偶数数量: {}", even_count);
    
    // 映射基准
    let squared: Vec<i32> = large_data.iter().map(|&x| x * x).take(10).collect();
    println!("前10个平方数: {:?}", squared);
    
    // 查找基准
    let target = 5000;
    let found = large_data.iter().find(|&&x| x == target);
    println!("查找 {} 结果: {:?}", target, found.is_some());
}

/// 现代化性能测试
#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_user_manager_performance() {
        let mut manager = UserManager::new();
        
        // 创建大量用户进行性能测试
        for i in 0..1000 {
            let user = User::new(
                format!("用户{}", i),
                format!("user{}@example.com", i),
                20 + (i % 50) as u8
            ).unwrap();
            
            manager.add_user(user).unwrap();
        }
        
        assert_eq!(manager.user_count(), 1000);
        assert_eq!(manager.get_adult_users().len(), 1000);
    }
}

/// 现代化条件编译测试
#[cfg(test)]
mod conditional_tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_platform_windows() {
        println!("在 Windows 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_platform_linux() {
        println!("在 Linux 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_platform_macos() {
        println!("在 macOS 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn test_platform_other() {
        println!("在未知平台运行测试");
        assert!(true);
    }
}

/// 现代化属性测试（如果启用proptest feature）
#[cfg(feature = "proptest")]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_add_two_properties(a in -1000i32..1000i32) {
            let result = add_two(a);
            prop_assert!(result > a); // 加2应该大于原值
            prop_assert_eq!(result - 2, a); // 减2应该等于原值
        }
    }
}

/// 运行测试和文档示例
pub fn run_testing_examples() {
    println!("🧪 === 现代化测试和文档示例 ===");
    println!();
    
    // 基本函数测试
    println!("数学运算:");
    println!("5 + 2 = {}", add_two(5));
    println!("-3 + 2 = {}", add_two(-3));
    
    println!();
    
    // 用户管理测试
    println!("用户管理:");
    let mut manager = UserManager::new();
    
    match User::new(
        "张三".to_string(),
        "zhangsan@example.com".to_string(),
        25
    ) {
        Ok(user) => {
            manager.add_user(user).unwrap();
            println!("✅ 用户添加成功");
        }
        Err(e) => println!("❌ 用户创建失败: {}", e),
    }
    
    match User::new(
        "李四".to_string(),
        "invalid-email".to_string(),
        25
    ) {
        Ok(user) => {
            manager.add_user(user).unwrap();
            println!("✅ 用户添加成功");
        }
        Err(e) => println!("❌ 用户创建失败: {}", e),
    }
    
    println!("当前用户数量: {}", manager.user_count());
    
    println!();
    
    // 问候测试
    println!("问候测试:");
    println!("问候: {}", greeting("世界"));
    println!("问候: {}", greeting("Rust 爱好者"));
    
    println!();
    
    // 文档函数
    documented_function();
    
    println!();
    
    // 基准测试
    benchmark_operations();
    
    println!();
    
    // 运行单元测试提示
    #[cfg(test)]
    {
        println!("💡 提示：使用 'cargo test' 运行所有测试");
        println!("💡 提示：使用 'cargo test --doc' 运行文档测试");
    }
    
    println!("\n✅ 所有测试和文档示例运行完成！");
}