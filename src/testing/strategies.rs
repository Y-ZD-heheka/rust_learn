//! 教学主题三：属性思维、边界测试与测试设计策略。
//!
//! 这一层把“如何设计测试”聚合在一起，方便学习者从 API 测试
//! 逐步过渡到：
//! - 属性思维演示 `property_thinking_basics`
//! - 边界与错误处理 `boundary_and_error_testing`
//! - 企业级策略与 TDD 示例

use super::domain::{EmailValidationError, validate_email_with_error};

/// 演示企业级测试策略。
pub fn enterprise_testing_strategies() {
    println!("🏢 企业级测试策略：");

    let mut calc = TeachingCalculator::new();

    println!("📊 基础运算测试:");
    assert_eq!(calc.add(2.0, 3.0), 5.0);
    assert_eq!(calc.subtract(10.0, 4.0), 6.0);
    assert_eq!(calc.multiply(3.0, 4.0), 12.0);
    assert_eq!(
        calc.divide(15.0, 3.0).expect("Division should succeed"),
        5.0
    );

    println!("✅ 基础运算测试通过");

    assert!(matches!(
        calc.divide(10.0, 0.0),
        Err(CalculatorError::DivisionByZero)
    ));
    println!("✅ 错误处理测试通过");

    assert_eq!(calc.get_history().len(), 4);
    println!("✅ 历史记录功能测试通过");

    println!("📊 企业级测试策略演示完成");
}

/// 演示“属性思维”而非真正的 property-based testing 框架。
///
/// 这里使用确定性样本覆盖多个输入区间，帮助学习者理解
/// “围绕不变量设计测试”的思路；若要做真正的属性测试，
/// 应进一步引入 `proptest` 或 `quickcheck` 等专用框架。
pub fn property_thinking_basics() {
    println!("🎯 属性思维基础：");
    println!("ℹ️ 这是确定性样本驱动的属性演示，不等同于真实 property-based testing 框架。");

    println!("\n🔍 反转两次保持不变:");
    for len in 0..=5 {
        let items = generate_sequence(len);
        let result = reverse_twice(&items);
        println!(
            "  长度 {:>2} 的序列 {:?}: {}",
            len,
            items,
            if result { "✅ 通过" } else { "❌ 失败" }
        );
    }

    println!("\n🔍 加法交换律（确定性枚举样本）:");
    for a in -3..=3 {
        for b in -3..=3 {
            assert!(addition_commutative(a, b));
        }
    }
    println!("  ✅ 在 [-3, 3] × [-3, 3] 样本上成立");

    println!("\n🔍 加法结合律（确定性枚举样本）:");
    for a in -2..=2 {
        for b in -2..=2 {
            for c in -2..=2 {
                assert!(addition_associative(a, b, c));
            }
        }
    }
    println!("  ✅ 在 [-2, 2]^3 样本上成立");

    println!("\n🔍 乘法对加法分配律（确定性枚举样本）:");
    for a in -2..=2 {
        for b in -2..=2 {
            for c in -2..=2 {
                assert!(multiplication_distributive(a, b, c));
            }
        }
    }
    println!("  ✅ 在 [-2, 2]^3 样本上成立");

    println!("\n🔍 质数判定的示例样本:");
    for (number, expected) in [
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (9, false),
        (11, true),
        (21, false),
        (29, true),
    ] {
        let result = is_prime(number);
        println!(
            "  {:>2} -> 期望 {}, 实际 {}",
            number,
            expected,
            if result { "质数" } else { "非质数" }
        );
        assert_eq!(result, expected);
    }

    println!("\n🔍 阶乘的单调性样本:");
    for n in 0..10 {
        let result = factorial_growth_property(n);
        println!(
            "  阶乘({}) 单调性: {}",
            n,
            if result { "✅ 通过" } else { "❌ 失败" }
        );
    }

    println!("📊 属性思维演示完成");
}

/// 演示测试驱动开发（TDD）示例。
pub fn test_driven_development_example() {
    println!("🔄 测试驱动开发（TDD）示例：");
    println!("🔬 TDD测试用例:");

    let mut calc = TeachingCalculator::new();

    assert_eq!(calc.add(2.0, 3.0), 5.0);
    println!("✅ 加法测试通过");

    assert_eq!(calc.subtract(10.0, 4.0), 6.0);
    println!("✅ 减法测试通过");

    assert_eq!(calc.multiply(3.0, 4.0), 12.0);
    println!("✅ 乘法测试通过");

    assert_eq!(
        calc.divide(15.0, 3.0).expect("Division should succeed"),
        5.0
    );
    println!("✅ 除法测试通过");

    assert!(matches!(
        calc.divide(10.0, 0.0),
        Err(CalculatorError::DivisionByZero)
    ));
    println!("✅ 除零错误处理测试通过");

    assert_eq!(calc.get_history().len(), 4);
    println!("✅ 历史记录功能测试通过");

    calc.clear_history();
    assert_eq!(calc.get_history().len(), 0);
    println!("✅ 清除历史记录测试通过");

    let result = calc.add(0.1, 0.2);
    assert!((result - 0.3).abs() < f64::EPSILON);
    println!("✅ 浮点数精度测试通过");

    println!("📊 TDD示例完成");
}

/// 演示边界条件和错误处理测试。
pub fn boundary_and_error_testing() {
    println!("🎯 边界条件和错误处理测试：");

    println!("🔍 年龄验证边界测试:");
    for (age, desc) in [
        (-1, "负数年龄"),
        (0, "零岁"),
        (1, "一岁"),
        (18, "成年年龄"),
        (65, "退休年龄"),
        (120, "高龄"),
        (150, "极限年龄"),
        (151, "超出上限"),
    ] {
        match validate_age(age) {
            Ok(()) => println!("  ✅ {}: 有效", desc),
            Err(error) => println!("  ❌ {}: {}", desc, error),
        }
    }

    println!("\n🔍 用户名验证边界测试:");
    for (username, desc) in [
        ("", "空字符串"),
        ("  ", "纯空格"),
        ("ab", "太短"),
        ("abc", "最小有效长度"),
        ("user_name", "包含下划线"),
        ("UserName", "包含大写"),
    ] {
        match validate_username(username) {
            Ok(()) => println!("  ✅ {}: 有效", desc),
            Err(error) => println!("  ❌ {}: {}", desc, error),
        }
    }

    println!("\n🔍 邮箱验证边界测试（与 domain 模块共用同一规则）:");
    for (email, desc) in [
        ("", "空字符串"),
        ("@", "只有@符号"),
        ("user@", "缺少域名"),
        ("user@domain", "缺少顶级域名"),
        ("user@domain.com", "有效邮箱"),
        ("user.name@domain.com", "包含点的用户名"),
        ("user@.example.com", "域名以点开头"),
    ] {
        match boundary_validate_email(email) {
            Ok(()) => println!("  ✅ {}: 有效", desc),
            Err(error) => println!("  ❌ {}: {}", desc, error),
        }
    }

    println!("📊 边界条件测试完成");
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
enum CalculatorError {
    #[error("除数不能为零")]
    DivisionByZero,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
enum BoundaryValidationError {
    #[error("年龄不能为负数")]
    NegativeAge,
    #[error("年龄超出合理范围")]
    AgeTooLarge,
    #[error("用户名不能为空")]
    EmptyUsername,
    #[error("用户名长度至少3个字符")]
    UsernameTooShort,
    #[error("用户名长度不能超过20个字符")]
    UsernameTooLong,
}

#[derive(Debug, Default)]
struct TeachingCalculator {
    history: Vec<f64>,
}

impl TeachingCalculator {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.history.push(result);
        result
    }

    fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let result = a - b;
        self.history.push(result);
        result
    }

    fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = a * b;
        self.history.push(result);
        result
    }

    fn divide(&mut self, a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }

        let result = a / b;
        self.history.push(result);
        Ok(result)
    }

    fn get_history(&self) -> &[f64] {
        &self.history
    }

    fn clear_history(&mut self) {
        self.history.clear();
    }
}

fn generate_sequence(len: usize) -> Vec<i32> {
    (0..len as i32).collect()
}

fn reverse_twice<T: Clone + PartialEq>(items: &[T]) -> bool {
    let reversed: Vec<_> = items.iter().cloned().rev().collect();
    let reversed_twice: Vec<_> = reversed.iter().cloned().rev().collect();
    items.iter().eq(reversed_twice.iter())
}

#[allow(clippy::eq_op)]
fn addition_commutative(a: i32, b: i32) -> bool {
    a + b == b + a
}

fn addition_associative(a: i32, b: i32, c: i32) -> bool {
    (a + b) + c == a + (b + c)
}

fn multiplication_distributive(a: i32, b: i32, c: i32) -> bool {
    a * (b + c) == a * b + a * c
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn factorial_growth_property(n: u32) -> bool {
    let current = factorial(n);
    let next = factorial(n + 1);
    next >= current
}

fn factorial(n: u32) -> u64 {
    (1..=n as u64).product::<u64>()
}

fn validate_age(age: i32) -> Result<(), BoundaryValidationError> {
    if age < 0 {
        return Err(BoundaryValidationError::NegativeAge);
    }
    if age > 150 {
        return Err(BoundaryValidationError::AgeTooLarge);
    }
    Ok(())
}

fn validate_username(username: &str) -> Result<(), BoundaryValidationError> {
    if username.trim().is_empty() {
        return Err(BoundaryValidationError::EmptyUsername);
    }
    if username.len() < 3 {
        return Err(BoundaryValidationError::UsernameTooShort);
    }
    if username.len() > 20 {
        return Err(BoundaryValidationError::UsernameTooLong);
    }
    Ok(())
}

fn boundary_validate_email(email: &str) -> Result<(), EmailValidationError> {
    validate_email_with_error(email)
}
