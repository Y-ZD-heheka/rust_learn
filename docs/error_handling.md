# 错误处理模块 (error_handling)

## 📖 模块概述

Rust 的错误处理系统是其安全性的重要组成部分。本模块详细讲解 `Result`、`Option`、`panic` 以及自定义错误类型的使用方法。

## 🎯 学习目标

- 理解可恢复错误和不可恢复错误的区别
- 掌握 `Result` 和 `Option` 的使用
- 学会使用 `?` 操作符进行错误传播
- 能够定义自定义错误类型
- 掌握错误处理的最佳实践

## 📚 内容目录

### 1. Panic 处理 (`modern_panic_handling`)

```rust
// 不可恢复错误 - panic
fn cause_panic() {
    panic!("这是一个 panic!");
}

// 安全的数组访问
let v = vec![1, 2, 3];
let value = v.get(5).unwrap_or(&0);  // 安全获取，不会 panic

// 使用 expect 提供错误信息
let config = std::env::var("CONFIG")
    .expect("CONFIG 环境变量未设置");

// debug_assert 仅在 debug 模式生效
debug_assert!(value >= 0, "值必须为正数");
```

### 2. Result 处理 (`modern_result_handling`)

```rust
// 基本 Result 使用
fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// 模式匹配处理
match divide(10.0, 2.0) {
    Ok(result) => println!("结果: {}", result),
    Err(e) => eprintln!("错误: {}", e),
}

// unwrap_or 提供默认值
let result = divide(10.0, 0.0).unwrap_or(0.0);

// unwrap_or_else 处理错误
let result = divide(10.0, 0.0).unwrap_or_else(|e| {
    eprintln!("错误: {}", e);
    0.0
});
```

### 3. ? 操作符 (`modern_question_mark_patterns`)

```rust
// 使用 ? 传播错误
fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // 自动传播错误
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 链式调用
fn complex_operation() -> Result<String, AppError> {
    let content = read_file_content("config.txt")?;
    let number: i32 = content.trim().parse()
        .map_err(|_| AppError::ParseError)?;
    Ok(format!("数字: {}", number))
}
```

### 4. 自定义错误类型 (`modern_error_types`)

```rust
// 使用 thiserror 库
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("解析错误: {0}")]
    Parse(#[from] std::num::ParseIntError),
    
    #[error("网络错误 {code}: {message}")]
    Network { code: u16, message: String },
    
    #[error("自定义错误: {message}")]
    Custom { message: String },
}

// 手动实现
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO错误: {}", e),
            Self::Parse(e) => write!(f, "解析错误: {}", e),
            Self::Network { code, message } => {
                write!(f, "网络错误 {}: {}", code, message)
            }
            Self::Custom { message } => write!(f, "{}", message),
        }
    }
}
```

### 5. 错误恢复策略 (`modern_error_recovery`)

```rust
// 重试机制
fn with_retry<T, F>(max_retries: usize, operation: F) -> Result<T, String>
where
    F: Fn() -> Result<T, &'static str>,
{
    let mut last_error = String::new();
    
    for attempt in 1..=max_retries {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_error = e.to_string();
                if attempt < max_retries {
                    println!("重试第 {} 次...", attempt + 1);
                }
            }
        }
    }
    
    Err(format!("所有重试失败: {}", last_error))
}

// 使用示例
let result = with_retry(3, || unreliable_operation())?;
```

### 6. 网络错误处理 (`network_error_handling`)

```rust
#[derive(Debug)]
enum NetworkError {
    ConnectionTimeout,
    BadRequest { status: u16, message: String },
    ServerError { status: u16, message: String },
    JsonParsingError,
}

async fn fetch_user_data(user_id: u32) -> Result<String, NetworkError> {
    match user_id {
        0 => Err(NetworkError::BadRequest {
            status: 400,
            message: "无效的用户ID".to_string()
        }),
        999 => Err(NetworkError::ConnectionTimeout),
        _ => Ok(format!("用户 {} 的数据", user_id)),
    }
}

// 错误处理策略
match fetch_user_data(user_id).await {
    Ok(data) => println!("成功: {}", data),
    Err(NetworkError::ConnectionTimeout) => {
        // 实现重试逻辑
    },
    Err(NetworkError::BadRequest { status, message }) => {
        // 记录并通知用户
    },
    Err(e) => eprintln!("错误: {}", e),
}
```

### 7. 文件系统错误处理 (`file_system_error_handling`)

```rust
use std::fs;
use std::io;

fn read_config_file() -> Result<String, io::Error> {
    let config_path = "config.json";
    
    // 检查文件存在
    if !std::path::Path::new(config_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("配置文件不存在: {}", config_path)
        ));
    }
    
    fs::read_to_string(config_path)
}

// 错误分类处理
match fs::read_to_string("data.txt") {
    Ok(contents) => println!("内容: {}", contents),
    Err(e) => match e.kind() {
        io::ErrorKind::NotFound => eprintln!("文件不存在"),
        io::ErrorKind::PermissionDenied => eprintln!("权限不足"),
        io::ErrorKind::InvalidData => eprintln!("数据无效"),
        _ => eprintln!("其他错误: {}", e),
    }
}
```

### 8. 业务验证错误 (`business_validation_error_handling`)

```rust
#[derive(Debug)]
enum ValidationError {
    EmptyField(String),
    InvalidFormat { field: String, format: String },
    ValueOutOfRange { field: String, min: i32, max: i32, actual: i32 },
    DuplicateEntry(String),
}

fn validate_user_registration(
    username: &str,
    email: &str,
    age: i32,
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
    
    if !email.contains('@') {
        return Err(ValidationError::InvalidFormat {
            field: "邮箱".to_string(),
            format: "必须包含@符号".to_string(),
        });
    }
    
    Ok(())
}
```

## 🚀 运行示例

```bash
# 运行错误处理模块
cargo run error_handling

# 运行测试
cargo test error_handling
```

## 📊 错误处理流程图

```
┌─────────────────────────────────────────────────────────────┐
│                     错误处理决策树                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                    ┌─────────────┐                          │
│                    │  发生错误?   │                          │
│                    └──────┬──────┘                          │
│                           │                                 │
│              ┌────────────┴────────────┐                    │
│              │                         │                    │
│              ▼                         ▼                    │
│      ┌───────────────┐         ┌───────────────┐           │
│      │ 可恢复错误?    │         │ 不可恢复错误   │           │
│      └───────┬───────┘         └───────┬───────┘           │
│              │                         │                    │
│              ▼                         ▼                    │
│      ┌───────────────┐         ┌───────────────┐           │
│      │ Result<T, E>  │         │    panic!     │           │
│      │ Option<T>     │         │  unwrap()     │           │
│      └───────┬───────┘         └───────────────┘           │
│              │                                              │
│              ▼                                              │
│      ┌───────────────┐                                      │
│      │ 如何处理?      │                                      │
│      └───────┬───────┘                                      │
│              │                                              │
│   ┌──────────┼──────────┬──────────┐                        │
│   ▼          ▼          ▼          ▼                        │
│ match    unwrap_or   ? 操作符   map_err                    │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 编写一个函数 `safe_divide`，返回 `Result<f64, String>`
2. 使用 `match` 处理 `Option` 类型的值
3. 使用 `unwrap_or` 为 `None` 提供默认值

### 中级
1. 定义一个自定义错误类型，实现 `Display` 和 `Error` 特征
2. 使用 `?` 操作符实现链式错误传播
3. 实现一个带重试机制的函数

### 高级
1. 使用 `thiserror` 库定义复杂的错误类型
2. 实现一个错误处理中间件
3. 设计一个统一的错误处理框架

## 🔗 相关资源

- [Rust 错误处理](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror 库](https://docs.rs/thiserror)
- [anyhow 库](https://docs.rs/anyhow)

## ⚠️ 常见陷阱

### 1. 过度使用 unwrap
```rust
// ❌ 不推荐：可能 panic
let value = some_option.unwrap();

// ✅ 推荐：安全处理
let value = some_option.unwrap_or(default);
let value = some_option.ok_or(MyError::NotFound)?;
```

### 2. 忽略错误
```rust
// ❌ 不推荐：忽略错误
let _ = file.write_all(data);

// ✅ 推荐：处理错误
if let Err(e) = file.write_all(data) {
    eprintln!("写入失败: {}", e);
}
```

### 3. 错误类型转换
```rust
// ❌ 错误：类型不匹配
fn foo() -> Result<String, io::Error> {
    let n: i32 = "abc".parse()?;  // ParseIntError != io::Error
    Ok(n.to_string())
}

// ✅ 正确：使用 map_err 或 From trait
fn foo() -> Result<String, AppError> {
    let n: i32 = "abc".parse().map_err(AppError::from)?;
    Ok(n.to_string())
}
```

## 📊 学习检查清单

- [ ] 理解 panic 和 Result 的区别
- [ ] 掌握 Option 的使用方法
- [ ] 会使用 match 处理错误
- [ ] 掌握 ? 操作符
- [ ] 能够定义自定义错误类型
- [ ] 理解错误传播机制
- [ ] 会使用 thiserror 和 anyhow
- [ ] 掌握错误恢复策略
