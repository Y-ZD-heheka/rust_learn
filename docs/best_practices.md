# 最佳实践模块 (best_practices)

## 📖 模块概述

本模块总结了 Rust 编程的最佳实践，帮助开发者编写更加优雅、高效、可维护的代码。

## 🎯 学习目标

- 掌握 Rust 代码风格规范
- 理解错误处理的最佳实践
- 学会性能优化技巧
- 掌握 API 设计原则
- 理解文档编写规范

## 📚 内容目录

### 1. 命名规范

```rust
// 类型命名：大驼峰
struct UserService { }
enum HttpStatus { }
type UserId = u64;

// 函数和变量：蛇形命名
fn calculate_total_price() { }
let user_count = 0;

// 常量：全大写蛇形
const MAX_CONNECTIONS: usize = 100;
static DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

// 生命周期：短小精悍
fn parse<'a>(input: &'a str) -> &'a str { }

// 泛型类型：简短大驼峰
fn process<T, E, U>() { }
```

### 2. 错误处理最佳实践

```rust
// 使用 thiserror 定义错误
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("用户不存在: {0}")]
    UserNotFound(u64),
}

// 使用 anyhow 处理应用错误
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("无法读取配置文件")?;
    
    let config: Config = toml::from_str(&content)
        .context("配置文件格式错误")?;
    
    Ok(config)
}

// 提供有意义的错误信息
fn validate_age(age: i32) -> Result<(), String> {
    if age < 0 {
        Err(format!("年龄不能为负数，当前值: {}", age))
    } else if age > 150 {
        Err(format!("年龄超出合理范围，当前值: {}", age))
    } else {
        Ok(())
    }
}
```

### 3. 类型设计最佳实践

```rust
// 使用 newtype 提供类型安全
#[derive(Debug, Clone, PartialEq, Eq)]
struct Email(String);

impl Email {
    fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') && email.contains('.') {
            Ok(Self(email.to_string()))
        } else {
            Err("无效的邮箱格式".to_string())
        }
    }
    
    fn as_str(&self) -> &str {
        &self.0
    }
}

// 使用枚举代替布尔参数
// ❌ 不推荐
fn process(data: &str, validate: bool) { }

// ✅ 推荐
enum ProcessingMode {
    WithValidation,
    WithoutValidation,
}

fn process(data: &str, mode: ProcessingMode) { }

// 使用 Option 明确表示可能缺失的值
struct User {
    name: String,
    email: Option<String>,  // 明确表示邮箱可能不存在
}
```

### 4. 函数设计最佳实践

```rust
// 单一职责原则
// ❌ 不推荐：一个函数做太多事
fn process_user(user: User) -> Result<ProcessedUser, Error> {
    let validated = validate_user(user)?;
    let normalized = normalize_user(validated);
    let saved = save_user(normalized)?;
    Ok(saved)
}

// ✅ 推荐：拆分为多个函数
fn process_user(user: User) -> Result<ProcessedUser, Error> {
    validate_user(&user)?;
    let normalized = normalize_user(user);
    save_user(normalized)
}

// 使用构建器处理复杂参数
let request = RequestBuilder::new()
    .url("https://api.example.com")
    .method(Method::POST)
    .header("Content-Type", "application/json")
    .body(json)
    .build()?;

// 提供合理的默认值
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            timeout: Duration::from_secs(30),
        }
    }
}
```

### 5. 性能优化最佳实践

```rust
// 避免不必要的克隆
// ❌ 不推荐
fn process(data: Vec<String>) -> Vec<String> {
    let mut result = data.clone();  // 不必要的克隆
    result.sort();
    result
}

// ✅ 推荐
fn process(mut data: Vec<String>) -> Vec<String> {
    data.sort();
    data
}

// 使用迭代器链
let result: Vec<i32> = (0..100)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .take(10)
    .collect();

// 预分配容量
let mut result = Vec::with_capacity(1000);
for i in 0..1000 {
    result.push(i);
}

// 使用 Cow 避免不必要的分配
use std::borrow::Cow;

fn process_string(input: &str) -> Cow<str> {
    if input.contains("replace") {
        Cow::Owned(input.replace("replace", "new"))
    } else {
        Cow::Borrowed(input)
    }
}
```

### 6. 并发编程最佳实践

```rust
// 使用 Arc 共享数据
use std::sync::Arc;

let shared_data = Arc::new(Data::new());

// 使用 Mutex 保护可变状态
use std::sync::Mutex;

let counter = Arc::new(Mutex::new(0));

// 避免长时间持有锁
// ❌ 不推荐
let data = mutex.lock().unwrap();
process_data(&data);  // 处理时仍持有锁

// ✅ 推荐
let data = {
    let guard = mutex.lock().unwrap();
    guard.clone()  // 快速复制
};  // 立即释放锁
process_data(&data);

// 使用通道进行线程通信
let (tx, rx) = std::sync::mpsc::channel();
```

### 7. 文档编写最佳实践

```rust
/// 计算两个数的最大公约数
///
/// # Arguments
///
/// * `a` - 第一个正整数
/// * `b` - 第二个正整数
///
/// # Returns
///
/// 返回 `a` 和 `b` 的最大公约数
///
/// # Examples
///
/// ```
/// let result = gcd(48, 18);
/// assert_eq!(result, 6);
/// ```
///
/// # Panics
///
/// 如果 `a` 或 `b` 为零，将 panic
///
/// # Complexity
///
/// 时间复杂度: O(log(min(a, b)))
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
```

### 8. 测试最佳实践

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 测试命名：test_<函数名>_<场景>
    #[test]
    fn test_gcd_normal_case() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn test_gcd_same_numbers() {
        assert_eq!(gcd(5, 5), 5);
    }

    #[test]
    fn test_gcd_one_is_one() {
        assert_eq!(gcd(1, 100), 1);
    }

    // 使用测试辅助函数
    fn create_test_user() -> User {
        User {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        }
    }

    #[test]
    fn test_user_creation() {
        let user = create_test_user();
        assert_eq!(user.name, "Test User");
    }
}
```

## 🚀 运行示例

```bash
# 运行最佳实践模块
cargo run best_practices

# 运行 clippy 检查
cargo clippy

# 格式化代码
cargo fmt
```

## 📊 代码质量检查清单

```
┌─────────────────────────────────────────────────────────────┐
│                     代码质量检查清单                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  代码风格                                                    │
│  □ 遵循 Rust 命名规范                                        │
│  □ 使用 cargo fmt 格式化                                     │
│  □ 无 clippy 警告                                           │
│  □ 适当的注释和文档                                          │
│                                                             │
│  错误处理                                                    │
│  □ 使用 Result 而非 panic                                   │
│  □ 提供有意义的错误信息                                      │
│  □ 正确使用 ? 操作符                                        │
│  □ 处理所有可能的错误                                        │
│                                                             │
│  性能                                                        │
│  □ 避免不必要的克隆                                          │
│  □ 使用迭代器代替循环                                        │
│  □ 预分配集合容量                                            │
│  □ 避免不必要的分配                                          │
│                                                             │
│  安全                                                        │
│  □ 验证所有输入                                              │
│  □ 安全处理敏感数据                                          │
│  □ 使用安全的默认值                                          │
│  □ 避免不安全代码                                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 重构代码使其符合 Rust 命名规范
2. 为函数添加文档注释
3. 使用 Result 替代 panic

### 中级
1. 使用 newtype 模式重构代码
2. 实现一个构建器模式
3. 优化性能瓶颈代码

### 高级
1. 设计一个完整的 API 模块
2. 实现一个可扩展的错误处理系统
3. 编写完整的测试套件

## 🔗 相关资源

- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
- [Rust 编码规范](https://rust-coding-guidelines.readthedocs.io/)

## ⚠️ 常见反模式

### 1. 过度使用 unwrap
```rust
// ❌ 不推荐
let value = option.unwrap();

// ✅ 推荐
let value = option.ok_or(Error::NotFound)?;
```

### 2. 忽略错误
```rust
// ❌ 不推荐
let _ = file.write_all(data);

// ✅ 推荐
file.write_all(data).context("写入失败")?;
```

### 3. 过度使用 clone
```rust
// ❌ 不推荐
fn process(data: &Vec<String>) -> Vec<String> {
    data.clone().sort();
    data.clone()
}

// ✅ 推荐
fn process(data: &[String]) -> Vec<String> {
    let mut result = data.to_vec();
    result.sort();
    result
}
```

## 📊 学习检查清单

- [ ] 掌握命名规范
- [ ] 理解错误处理最佳实践
- [ ] 会使用 newtype 模式
- [ ] 掌握函数设计原则
- [ ] 理解性能优化技巧
- [ ] 会编写文档注释
- [ ] 掌握测试最佳实践
- [ ] 理解代码质量标准
