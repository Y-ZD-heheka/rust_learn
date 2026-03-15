//! 教学主题二：文件 IO 与配置错误。
//!
//! 这一层把“会接触操作系统或环境输入”的错误放在一起：
//! - 文件不存在、权限不足、数据无效
//! - 配置缺失、配置值非法
//! - 读取后如何给出更友好的恢复提示

use std::fmt;
use std::fs;
use std::io::{self, Read};
use std::time::{SystemTime, UNIX_EPOCH};

use super::fundamentals::{log_demo_error, log_demo_message};

fn unique_demo_path(label: &str, extension: &str) -> std::path::PathBuf {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    std::env::temp_dir().join(format!(
        "rust_learn_error_handling_{}_{}.{}",
        label, unique_id, extension
    ))
}

/// 演示真实文件系统错误处理。
pub fn file_system_error_handling() {
    println!("📁 文件系统错误处理：");

    fn create_demo_file(path: &std::path::Path, content: &str) -> Result<(), io::Error> {
        fs::write(path, content)
    }

    fn read_config_file(path: &std::path::Path) -> Result<String, io::Error> {
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("配置文件不存在: {}", path.display()),
            ));
        }

        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn create_backup(source: &std::path::Path, backup: &std::path::Path) -> Result<(), io::Error> {
        let contents = fs::read_to_string(source)?;
        fs::write(backup, contents)?;
        println!("✅ 备份创建成功: {}", backup.display());
        Ok(())
    }

    let existing_file_1 = unique_demo_path("data1", "txt");
    let existing_file_2 = unique_demo_path("data2", "txt");
    let missing_file = unique_demo_path("missing", "txt");

    if let Err(error) = create_demo_file(&existing_file_1, "alpha data") {
        log_demo_error("创建示例文件 data1", &error);
        return;
    }
    if let Err(error) = create_demo_file(&existing_file_2, "beta data") {
        log_demo_error("创建示例文件 data2", &error);
        let _ = fs::remove_file(&existing_file_1);
        return;
    }

    let files = vec![
        existing_file_1.clone(),
        existing_file_2.clone(),
        missing_file.clone(),
    ];

    for file in &files {
        match fs::read_to_string(file) {
            Ok(contents) => {
                println!("📄 文件 {} 内容长度: {}", file.display(), contents.len());

                let backup_name = file.with_extension("txt.backup");
                if let Err(error) = create_backup(file, &backup_name) {
                    log_demo_error(&format!("创建备份 {}", backup_name.display()), &error);
                }
                let _ = fs::remove_file(backup_name);
            }
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => {
                    log_demo_error("读取文件", &format!("文件不存在: {}", file.display()));
                }
                io::ErrorKind::PermissionDenied => {
                    log_demo_error("读取文件", &format!("权限不足: {}", file.display()));
                }
                io::ErrorKind::InvalidData => {
                    log_demo_error("读取文件", &format!("数据无效: {}", file.display()));
                }
                _ => {
                    log_demo_error("读取文件", &format!("{}: {}", file.display(), error));
                }
            },
        }
    }

    let config_path = unique_demo_path("config", "json");
    if let Err(error) = create_demo_file(&config_path, r#"{"mode":"demo"}"#) {
        log_demo_error("创建配置文件", &error);
    } else {
        match read_config_file(&config_path) {
            Ok(contents) => println!("⚙️ 配置读取成功: {}", contents),
            Err(error) => log_demo_error("读取配置文件", &error),
        }
    }

    let _ = fs::remove_file(existing_file_1);
    let _ = fs::remove_file(existing_file_2);
    let _ = fs::remove_file(config_path);
}

/// 演示配置解析错误处理。
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
        FileParseError {
            field: String,
            message: String,
        },
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::MissingField(field) => write!(f, "缺少必需字段: {}", field),
                Self::InvalidType {
                    field,
                    expected,
                    actual,
                } => write!(
                    f,
                    "字段 {} 类型错误，期望 {}，实际 {}",
                    field, expected, actual
                ),
                Self::InvalidValue {
                    field,
                    value,
                    reason,
                } => write!(f, "字段 {} 值无效: {} - {}", field, value, reason),
                Self::FileParseError { field, message } => {
                    write!(f, "字段 {} 解析失败: {}", field, message)
                }
            }
        }
    }

    #[derive(Debug)]
    struct AppConfig {
        host: String,
        port: u16,
        database_url: String,
        debug: bool,
    }

    impl AppConfig {
        fn from_pairs(entries: &[(&str, &str)]) -> Result<Self, ConfigError> {
            fn find_value<'a>(entries: &'a [(&str, &str)], key: &str) -> Option<&'a str> {
                entries
                    .iter()
                    .find_map(|(entry_key, entry_value)| (*entry_key == key).then_some(*entry_value))
            }

            let host = find_value(entries, "APP_HOST")
                .ok_or_else(|| ConfigError::MissingField("APP_HOST".to_string()))?
                .to_string();

            let port_str = find_value(entries, "APP_PORT")
                .ok_or_else(|| ConfigError::MissingField("APP_PORT".to_string()))?;

            let port = port_str
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidType {
                    field: "APP_PORT".to_string(),
                    expected: "u16".to_string(),
                    actual: port_str.to_string(),
                })?;

            let database_url = find_value(entries, "DATABASE_URL")
                .ok_or_else(|| ConfigError::MissingField("DATABASE_URL".to_string()))?
                .to_string();

            if !database_url.starts_with("postgresql://") {
                return Err(ConfigError::InvalidValue {
                    field: "DATABASE_URL".to_string(),
                    value: database_url.clone(),
                    reason: "必须是 PostgreSQL 连接字符串".to_string(),
                });
            }

            let debug = find_value(entries, "APP_DEBUG")
                .unwrap_or("false")
                .parse::<bool>()
                .map_err(|_| ConfigError::FileParseError {
                    field: "APP_DEBUG".to_string(),
                    message: "不是合法布尔值".to_string(),
                })?;

            Ok(Self {
                host,
                port,
                database_url,
                debug,
            })
        }
    }

    let error_scenarios = [
        vec![],
        vec![("APP_HOST", "localhost")],
        vec![("APP_HOST", "localhost"), ("APP_PORT", "invalid_port")],
        vec![
            ("APP_HOST", "localhost"),
            ("APP_PORT", "8080"),
            ("DATABASE_URL", "invalid_url"),
        ],
        vec![
            ("APP_HOST", "127.0.0.1"),
            ("APP_PORT", "8080"),
            ("DATABASE_URL", "postgresql://user:pass@localhost/db"),
            ("APP_DEBUG", "true"),
        ],
    ];

    for (index, scenario) in error_scenarios.iter().enumerate() {
        println!("测试场景 {}:", index + 1);

        match AppConfig::from_pairs(scenario) {
            Ok(config) => println!(
                "✅ 配置解析成功: host={}, port={}, database_url={}, debug={}",
                config.host, config.port, config.database_url, config.debug
            ),
            Err(error) => {
                log_demo_error("配置解析", &error);
                log_demo_message("配置解析", "教学重点是识别配置错误并给出可读反馈，而不是直接 panic");
            }
        }

        println!();
    }
}
