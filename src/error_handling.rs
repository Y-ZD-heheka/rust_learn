//! # 错误处理模块
//!
//! 这个模块演示了Rust的错误处理机制，包括可恢复错误和不可恢复错误。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

use std::fmt;
use std::fs::File;
use std::io::{self, Read};

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
    // 注意：debug_assert只在debug模式下触发，release模式下会被移除
    let positive_number = 5; // 使用正数避免触发断言
    debug_assert!(positive_number > 0, "数字必须为正数");
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
        Self::Network {
            code,
            message: message.to_string(),
        }
    }

    fn custom_error(message: &str) -> Self {
        Self::Custom {
            message: message.to_string(),
        }
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
    std::fs::write("test_file.txt", test_content).expect("Failed to create test file");

    fn read_file_content(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // 现代化错误处理链
    let _content = read_file_content("test_file.txt")
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
        let number: i32 = file_content
            .trim()
            .parse()
            .map_err(|_| AppError::custom_error("数字解析失败"))?;
        Ok(format!("解析得到数字: {}", number))
    }

    let complex_result = complex_operation().unwrap_or_else(|e| {
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
            Err(AppError::Network { code, message }) => {
                println!("❌ '{}' -> 网络错误 {}: {}", case, code, message)
            }
            Err(AppError::Parse { .. }) => println!("❌ '{}' -> 解析错误", case),
            Err(AppError::Custom { message }) => {
                println!("❌ '{}' -> 自定义错误: {}", case, message)
            }
            Err(AppError::Io { .. }) => println!("❌ '{}' -> IO错误", case),
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

/// 演示真实网络请求错误处理
pub fn network_error_handling() {
    println!("🌐 网络请求错误处理：");

    // 模拟HTTP客户端错误
    #[derive(Debug)]
    enum NetworkError {
        ConnectionTimeout,
        BadRequest { status: u16, message: String },
        ServerError { status: u16, message: String },
        NetworkError(String),
        JsonParsingError,
    }

    impl fmt::Display for NetworkError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ConnectionTimeout => write!(f, "连接超时"),
                Self::BadRequest { status, message } => {
                    write!(f, "客户端错误 {}: {}", status, message)
                }
                Self::ServerError { status, message } => {
                    write!(f, "服务器错误 {}: {}", status, message)
                }
                Self::NetworkError(msg) => write!(f, "网络错误: {}", msg),
                Self::JsonParsingError => write!(f, "JSON解析错误"),
            }
        }
    }

    // 模拟API调用
    async fn fetch_user_data(user_id: u32) -> Result<String, NetworkError> {
        match user_id {
            0 => Err(NetworkError::BadRequest {
                status: 400,
                message: "无效的用户ID".to_string(),
            }),
            999 => Err(NetworkError::ConnectionTimeout),
            1000 => Err(NetworkError::ServerError {
                status: 500,
                message: "内部服务器错误".to_string(),
            }),
            _ => Ok(format!("用户 {} 的数据", user_id)),
        }
    }

    // 错误处理策略（改为同步版本避免async问题）
    fn simulate_network_request(user_id: u32) -> Result<String, NetworkError> {
        match user_id {
            0 => Err(NetworkError::BadRequest {
                status: 400,
                message: "无效的用户ID".to_string(),
            }),
            999 => Err(NetworkError::ConnectionTimeout),
            1000 => Err(NetworkError::ServerError {
                status: 500,
                message: "内部服务器错误".to_string(),
            }),
            _ => Ok(format!("用户 {} 的数据", user_id)),
        }
    }

    let user_ids = vec![0, 1, 999, 1000, 2];

    for user_id in user_ids {
        match simulate_network_request(user_id) {
            Ok(data) => println!("✅ 获取成功: {}", data),
            Err(NetworkError::BadRequest { status, message }) => {
                println!("❌ 客户端错误 {}: {}", status, message);
                // 可以进行重试或用户友好的错误处理
            }
            Err(NetworkError::ServerError { status, message }) => {
                println!("❌ 服务器错误 {}: {}", status, message);
                // 记录到监控系统，可能需要回滚操作
            }
            Err(NetworkError::ConnectionTimeout) => {
                println!("❌ 连接超时");
                // 实现重试逻辑
            }
            Err(e) => println!("❌ 其他网络错误: {}", e),
        }
    }
}

/// 演示真实文件系统错误处理
pub fn file_system_error_handling() {
    println!("📁 文件系统错误处理：");

    use std::fs;
    use std::io::{self, Read};

    // 文件操作错误处理
    fn read_config_file() -> Result<String, io::Error> {
        let config_path = "config.json";

        // 检查文件是否存在
        if !std::path::Path::new(config_path).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("配置文件不存在: {}", config_path),
            ));
        }

        // 读取文件内容
        let mut file = fs::File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    // 创建备份文件
    fn create_backup(source: &str, backup: &str) -> Result<(), io::Error> {
        let contents = fs::read_to_string(source)?;
        fs::write(backup, contents)?;
        println!("✅ 备份创建成功: {}", backup);
        Ok(())
    }

    // 批量文件处理
    let files = vec!["data1.txt", "data2.txt", "missing.txt"];

    for file in files {
        match fs::read_to_string(file) {
            Ok(contents) => {
                println!("📄 文件 {} 内容长度: {}", file, contents.len());

                // 创建备份
                let backup_name = format!("{}.backup", file);
                if let Err(e) = create_backup(file, &backup_name) {
                    eprintln!("⚠️ 备份失败 {}: {}", backup_name, e);
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("❌ 文件不存在: {}", file);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("❌ 权限不足: {}", file);
                }
                io::ErrorKind::InvalidData => {
                    eprintln!("❌ 数据无效: {}", file);
                }
                _ => {
                    eprintln!("❌ 文件读取错误 {}: {}", file, e);
                }
            },
        }
    }
}

/// 演示配置解析错误处理
pub fn configuration_error_handling() {
    println!("⚙️ 配置解析错误处理：");

    #[derive(Debug)]
    enum ConfigError {
        MissingField(String),
        InvalidType {
            field: String,
            expected: String,
            actual: String,
        },
        InvalidValue {
            field: String,
            value: String,
            reason: String,
        },
        FileParseError(String),
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::MissingField(field) => write!(f, "缺少必需字段: {}", field),
                Self::InvalidType {
                    field,
                    expected,
                    actual,
                } => {
                    write!(
                        f,
                        "字段 {} 类型错误，期望 {}，实际 {}",
                        field, expected, actual
                    )
                }
                Self::InvalidValue {
                    field,
                    value,
                    reason,
                } => {
                    write!(f, "字段 {} 值无效: {} - {}", field, value, reason)
                }
                Self::FileParseError(msg) => write!(f, "配置文件解析错误: {}", msg),
            }
        }
    }

    // 模拟配置解析
    #[derive(Debug)]
    struct AppConfig {
        host: String,
        port: u16,
        database_url: String,
        debug: bool,
    }

    impl AppConfig {
        fn from_env() -> Result<Self, ConfigError> {
            let host = std::env::var("APP_HOST")
                .map_err(|_| ConfigError::MissingField("APP_HOST".to_string()))?;

            let port_str = std::env::var("APP_PORT")
                .map_err(|_| ConfigError::MissingField("APP_PORT".to_string()))?;

            let port = port_str
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidValue {
                    field: "APP_PORT".to_string(),
                    value: port_str,
                    reason: "不是有效的端口号".to_string(),
                })?;

            let database_url = std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::MissingField("DATABASE_URL".to_string()))?;

            if !database_url.starts_with("postgresql://") {
                return Err(ConfigError::InvalidValue {
                    field: "DATABASE_URL".to_string(),
                    value: database_url.clone(),
                    reason: "必须是PostgreSQL连接字符串".to_string(),
                });
            }

            let debug = std::env::var("APP_DEBUG")
                .unwrap_or_else(|_| "false".to_string())
                .parse::<bool>()
                .unwrap_or(false);

            Ok(Self {
                host,
                port,
                database_url,
                debug,
            })
        }
    }

    // 模拟配置验证
    let config_test_cases = vec![
        ("APP_HOST", "127.0.0.1"),
        ("APP_PORT", "8080"),
        ("DATABASE_URL", "postgresql://user:pass@localhost/db"),
        ("APP_DEBUG", "true"),
    ];

    // 模拟不同的配置错误场景
    let error_scenarios = vec![
        vec![],                                                        // 缺少所有配置
        vec![("APP_HOST", "localhost")],                               // 部分配置
        vec![("APP_HOST", "localhost"), ("APP_PORT", "invalid_port")], // 无效端口
        vec![
            ("APP_HOST", "localhost"),
            ("APP_PORT", "8080"),
            ("DATABASE_URL", "invalid_url"),
        ], // 无效数据库URL
    ];

    for (i, scenario) in error_scenarios.iter().enumerate() {
        println!("测试场景 {}:", i + 1);

        // 清理环境变量
        for (key, _) in config_test_cases.iter() {
            // SAFETY: 这是在单线程测试代码中修改环境变量
            unsafe {
                std::env::remove_var(key);
            }
        }

        // 设置测试环境变量
        for (key, value) in scenario {
            // SAFETY: 这是在单线程测试代码中修改环境变量
            unsafe {
                std::env::set_var(key, value);
            }
        }

        match AppConfig::from_env() {
            Ok(_config) => println!("✅ 配置解析成功"),
            Err(e) => println!("❌ 配置错误: {}", e),
        }

        println!();
    }
}

/// 演示业务逻辑验证错误处理
pub fn business_validation_error_handling() {
    println!("🔍 业务逻辑验证错误处理：");

    #[derive(Debug)]
    enum ValidationError {
        EmptyField(String),
        InvalidFormat {
            field: String,
            format: String,
        },
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
                } => {
                    write!(
                        f,
                        "字段 {} 值超出范围 [{}, {}]，实际: {}",
                        field, min, max, actual
                    )
                }
                Self::DuplicateEntry(entry) => write!(f, "重复条目: {}", entry),
                Self::ForbiddenAction(action) => write!(f, "禁止操作: {}", action),
            }
        }
    }

    // 用户注册验证
    fn validate_user_registration(
        username: &str,
        email: &str,
        age: i32,
        existing_users: &[&str],
    ) -> Result<(), ValidationError> {
        // 验证用户名
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

        // 检查用户名是否已存在
        if existing_users.contains(&username) {
            return Err(ValidationError::DuplicateEntry("用户名".to_string()));
        }

        // 验证邮箱
        if email.trim().is_empty() {
            return Err(ValidationError::EmptyField("邮箱".to_string()));
        }

        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidFormat {
                field: "邮箱".to_string(),
                format: "必须包含@和域名".to_string(),
            });
        }

        // 验证年龄
        if age < 13 || age > 120 {
            return Err(ValidationError::ValueOutOfRange {
                field: "年龄".to_string(),
                min: 13,
                max: 120,
                actual: age,
            });
        }

        Ok(())
    }

    // 测试用户注册场景
    let existing_users = vec!["john_doe", "jane_smith"];

    let test_cases = vec![
        ("", "john@example.com", 25),         // 空用户名
        ("jo", "jane@example.com", 25),       // 用户名太短
        ("john_doe", "jane@example.com", 25), // 用户名已存在
        ("john_doe", "invalid-email", 25),    // 无效邮箱
        ("john_doe", "john@example.com", 10), // 年龄太小
        ("alice", "alice@example.com", 25),   // 有效注册
    ];

    for (i, (username, email, age)) in test_cases.iter().enumerate() {
        println!("测试用例 {}: ({}, {}, {})", i + 1, username, email, age);

        match validate_user_registration(username, email, *age, &existing_users) {
            Ok(_) => println!("✅ 注册验证通过"),
            Err(e) => {
                println!("❌ 验证失败: {}", e);

                // 根据错误类型提供不同的用户友好提示
                match e {
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

/// 演示资源加载错误处理
pub fn resource_loading_error_handling() {
    println!("📦 资源加载错误处理：");

    #[derive(Debug)]
    enum ResourceError {
        FileNotFound(String),
        NetworkError(String),
        TimeoutError,
        CorruptedData(String),
        PermissionDenied(String),
        InsufficientMemory,
    }

    impl fmt::Display for ResourceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::FileNotFound(path) => write!(f, "文件未找到: {}", path),
                Self::NetworkError(msg) => write!(f, "网络错误: {}", msg),
                Self::TimeoutError => write!(f, "加载超时"),
                Self::CorruptedData(source) => write!(f, "数据损坏: {}", source),
                Self::PermissionDenied(resource) => write!(f, "权限拒绝: {}", resource),
                Self::InsufficientMemory => write!(f, "内存不足"),
            }
        }
    }

    // 模拟资源加载器
    struct ResourceLoader {
        cache: std::collections::HashMap<String, Vec<u8>>,
    }

    impl ResourceLoader {
        fn new() -> Self {
            Self {
                cache: std::collections::HashMap::new(),
            }
        }

        fn load_image(&mut self, path: &str) -> Result<Vec<u8>, ResourceError> {
            // 检查缓存
            if let Some(data) = self.cache.get(path) {
                return Ok(data.clone());
            }

            // 模拟不同的加载错误
            match path {
                "missing.png" => return Err(ResourceError::FileNotFound(path.to_string())),
                "corrupted.jpg" => return Err(ResourceError::CorruptedData(path.to_string())),
                "slow_network.webp" => {
                    return Err(ResourceError::NetworkError("网络延迟".to_string()));
                }
                _ => {
                    // 模拟成功加载
                    let data = vec![255, 216, 255, 224]; // 模拟JPEG文件头
                    self.cache.insert(path.to_string(), data.clone());
                    Ok(data)
                }
            }
        }

        fn load_font(&mut self, font_name: &str) -> Result<Vec<u8>, ResourceError> {
            match font_name {
                "admin_font.ttf" => {
                    return Err(ResourceError::PermissionDenied("管理员字体".to_string()));
                }
                "large_font.ttf" => return Err(ResourceError::InsufficientMemory),
                "default.ttf" => {
                    let data = vec![0, 1, 2, 3]; // 模拟字体数据
                    self.cache.insert(font_name.to_string(), data.clone());
                    Ok(data)
                }
                _ => Err(ResourceError::FileNotFound(format!("字体: {}", font_name))),
            }
        }
    }

    let mut loader = ResourceLoader::new();

    // 测试图片加载
    let images = vec![
        "avatar.png",
        "missing.png",
        "corrupted.jpg",
        "slow_network.webp",
    ];

    println!("图片加载测试:");
    for img in images {
        match loader.load_image(img) {
            Ok(data) => println!("✅ 图片 {} 加载成功，大小: {} 字节", img, data.len()),
            Err(e) => {
                println!("❌ 图片 {} 加载失败: {}", img, e);

                // 实现回退策略
                match e {
                    ResourceError::FileNotFound(_) => {
                        println!("🔄 使用默认图片");
                    }
                    ResourceError::CorruptedData(_) => {
                        println!("🔄 重新下载损坏的图片");
                    }
                    ResourceError::NetworkError(_) => {
                        println!("🔄 尝试从CDN加载");
                    }
                    _ => {
                        println!("🔄 跳过此资源");
                    }
                }
            }
        }
    }

    // 测试字体加载
    let fonts = vec!["default.ttf", "admin_font.ttf", "large_font.ttf"];

    println!("\n字体加载测试:");
    for font in fonts {
        match loader.load_font(font) {
            Ok(_data) => println!("✅ 字体 {} 加载成功", font),
            Err(e) => {
                println!("❌ 字体 {} 加载失败: {}", font, e);

                // 实现字体回退链
                println!("🔄 回退到系统默认字体");
            }
        }
    }
}

/// 演示第三方服务错误处理
pub fn external_service_error_handling() {
    println!("🔌 第三方服务错误处理：");

    #[derive(Debug)]
    enum ServiceError {
        RateLimitExceeded { service: String, retry_after: u32 },
        AuthenticationFailed { service: String },
        ServiceUnavailable { service: String },
        QuotaExceeded { service: String },
        ServiceError { service: String, message: String },
    }

    impl fmt::Display for ServiceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::RateLimitExceeded {
                    service,
                    retry_after,
                } => {
                    write!(f, "{} 速率限制，等待 {} 秒后重试", service, retry_after)
                }
                Self::AuthenticationFailed { service } => {
                    write!(f, "{} 认证失败", service)
                }
                Self::ServiceUnavailable { service } => {
                    write!(f, "{} 服务不可用", service)
                }
                Self::QuotaExceeded { service } => {
                    write!(f, "{} 配额已用完", service)
                }
                Self::ServiceError { service, message } => {
                    write!(f, "{} 服务错误: {}", service, message)
                }
            }
        }
    }

    // 模拟第三方服务调用
    #[derive(Debug, Clone)]
    enum Service {
        PaymentGateway,
        EmailService,
        SmsService,
        Analytics,
    }

    fn call_external_service(service: Service, request_data: &str) -> Result<String, ServiceError> {
        match service {
            Service::PaymentGateway => match request_data {
                "expired_card" => Err(ServiceError::ServiceError {
                    service: "支付网关".to_string(),
                    message: "信用卡已过期".to_string(),
                }),
                "insufficient_funds" => Err(ServiceError::ServiceError {
                    service: "支付网关".to_string(),
                    message: "余额不足".to_string(),
                }),
                _ => Ok("支付处理成功".to_string()),
            },

            Service::EmailService => {
                if request_data == "rate_limit" {
                    Err(ServiceError::RateLimitExceeded {
                        service: "邮件服务".to_string(),
                        retry_after: 60,
                    })
                } else {
                    Ok("邮件发送成功".to_string())
                }
            }

            Service::SmsService => {
                if request_data == "unauthorized" {
                    Err(ServiceError::AuthenticationFailed {
                        service: "短信服务".to_string(),
                    })
                } else if request_data == "quota_exceeded" {
                    Err(ServiceError::QuotaExceeded {
                        service: "短信服务".to_string(),
                    })
                } else {
                    Ok("短信发送成功".to_string())
                }
            }

            Service::Analytics => {
                if request_data == "maintenance" {
                    Err(ServiceError::ServiceUnavailable {
                        service: "分析服务".to_string(),
                    })
                } else {
                    Ok("数据收集成功".to_string())
                }
            }
        }
    }

    fn handle_service_error(service_name: &str, error: ServiceError) {
        println!("❌ 错误: {}", error);

        // 根据错误类型实施不同的恢复策略
        match error {
            ServiceError::RateLimitExceeded { retry_after, .. } => {
                println!("🔄 等待 {} 秒后重试", retry_after);
                // 在实际应用中这里会实现退避重试算法
            }
            ServiceError::AuthenticationFailed { .. } => {
                println!("🔄 刷新认证令牌并重试");
                // 更新API密钥或token
            }
            ServiceError::ServiceUnavailable { .. } => {
                println!("🔄 切换到备用服务");
                // 故障转移到备用服务商
            }
            ServiceError::QuotaExceeded { .. } => {
                println!("🔄 延迟处理，等待配额重置");
                // 将请求排入队列
            }
            ServiceError::ServiceError { message, .. } => {
                handle_payment_error(service_name, &message);
            }
        }
    }

    fn handle_payment_error(_service_name: &str, message: &str) {
        match message {
            "余额不足" => {
                println!("🔄 提示用户充值");
                // 向用户发送充值提醒
            }
            "信用卡已过期" => {
                println!("🔄 提示用户更新支付信息");
                // 要求用户更新信用卡信息
            }
            _ => {
                println!("🔄 记录错误并人工处理");
                // 记录到错误跟踪系统
            }
        }
    }

    fn get_service_name(service: &Service) -> &'static str {
        match service {
            Service::PaymentGateway => "支付网关",
            Service::EmailService => "邮件服务",
            Service::SmsService => "短信服务",
            Service::Analytics => "分析服务",
        }
    }

    fn process_service_call(service: Service, data: &str) {
        let service_name = get_service_name(&service);
        println!("调用 {}: {}", service_name, data);

        match call_external_service(service, data) {
            Ok(result) => println!("✅ 成功: {}", result),
            Err(e) => handle_service_error(service_name, e),
        }
        println!();
    }

    // 测试各种服务调用场景
    let test_cases = vec![
        (Service::PaymentGateway, "expired_card"),
        (Service::PaymentGateway, "valid_payment"),
        (Service::EmailService, "rate_limit"),
        (Service::EmailService, "normal_email"),
        (Service::SmsService, "unauthorized"),
        (Service::SmsService, "quota_exceeded"),
        (Service::SmsService, "valid_sms"),
        (Service::Analytics, "maintenance"),
        (Service::Analytics, "normal_event"),
    ];

    for (service, data) in test_cases.iter() {
        process_service_call(service.clone(), data);
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
    println!();

    network_error_handling();
    println!();

    file_system_error_handling();
    println!();

    configuration_error_handling();
    println!();

    business_validation_error_handling();
    println!();

    resource_loading_error_handling();
    println!();

    external_service_error_handling();

    println!("\n✅ 所有错误处理示例运行完成！");
}
