# 安全编程模块 (security)

## 📖 模块概述

Rust 的内存安全特性使其成为安全编程的理想选择。本模块讲解 Rust 中的安全编程实践，包括密码学、安全数据处理和常见安全漏洞的防范。

## 🎯 学习目标

- 理解 Rust 的安全保证
- 掌握密码学基础操作
- 学会安全处理敏感数据
- 了解常见安全漏洞及防范
- 掌握安全编码最佳实践

## 📚 内容目录

### 1. 密码哈希

```rust
use argon2::{self, Config};

// 安全的密码哈希
fn hash_password(password: &str, salt: &[u8]) -> Result<String, String> {
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), salt, &config)
        .map_err(|e| format!("哈希失败: {}", e))
}

// 验证密码
fn verify_password(hash: &str, password: &str) -> Result<bool, String> {
    argon2::verify_encoded(hash, password.as_bytes())
        .map_err(|e| format!("验证失败: {}", e))
}

// 使用
let salt = generate_salt();
let hash = hash_password("my_password", &salt)?;
let is_valid = verify_password(&hash, "my_password")?;
```

### 2. 安全随机数生成

```rust
use rand::RngCore;
use rand::rngs::OsRng;

// 生成安全的随机字节
fn generate_random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

// 生成安全的令牌
fn generate_token() -> String {
    let mut token = [0u8; 32];
    OsRng.fill_bytes(&mut token);
    base64::encode(token)
}

// 生成 UUID
fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}
```

### 3. 敏感数据处理

```rust
use zeroize::Zeroize;

// 敏感数据结构
struct Password(String);

impl Drop for Password {
    fn drop(&mut self) {
        // 安全擦除内存
        self.0.zeroize();
    }
}

impl Password {
    fn new(password: &str) -> Self {
        Self(password.to_string())
    }
    
    fn verify(&self, hash: &str) -> bool {
        // 验证逻辑
        verify_password(hash, &self.0).unwrap_or(false)
    }
}

// 使用 SecStr 库
use secstr::SecStr;

let password = SecStr::new("secret".to_string());
// 内存会被安全擦除
```

### 4. 输入验证

```rust
// SQL 注入防护
fn sanitize_input(input: &str) -> Result<String, String> {
    // 使用参数化查询而不是字符串拼接
    if input.contains('\'') || input.contains('"') {
        return Err("输入包含非法字符".to_string());
    }
    Ok(input.to_string())
}

// XSS 防护
fn escape_html(input: &str) -> String {
    input
        .replace('&', "&")
        .replace('<', "<")
        .replace('>', ">")
        .replace('"', """)
        .replace('\'', "&#x27;")
}

// 路径遍历防护
fn validate_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    let canonical = path.canonicalize()
        .map_err(|_| "无效路径")?;
    
    // 确保路径在允许的目录内
    if !canonical.starts_with("/allowed/directory") {
        return Err("路径遍历攻击".to_string());
    }
    
    Ok(canonical)
}
```

### 5. 加密解密

```rust
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, NewAead},
};

// AES-GCM 加密
fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    
    let nonce = Nonce::from_slice(b"unique nonce"); // 实际使用中应该随机生成
    
    cipher.encrypt(nonce, data)
        .map_err(|e| format!("加密失败: {}", e))
}

// AES-GCM 解密
fn decrypt(encrypted: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    
    let nonce = Nonce::from_slice(b"unique nonce");
    
    cipher.decrypt(nonce, encrypted)
        .map_err(|e| format!("解密失败: {}", e))
}
```

### 6. 安全配置

```rust
// 安全的服务器配置
struct SecureConfig {
    // 禁用不安全的 TLS 版本
    min_tls_version: TlsVersion,
    // 启用 HSTS
    hsts_enabled: bool,
    // 安全的 Cookie 设置
    cookie_secure: bool,
    cookie_http_only: bool,
    // CORS 配置
    allowed_origins: Vec<String>,
}

impl Default for SecureConfig {
    fn default() -> Self {
        Self {
            min_tls_version: TlsVersion::TLS_1_2,
            hsts_enabled: true,
            cookie_secure: true,
            cookie_http_only: true,
            allowed_origins: vec![],
        }
    }
}

// 环境变量安全处理
fn get_secret(key: &str) -> Result<String, String> {
    std::env::var(key)
        .map_err(|_| format!("环境变量 {} 未设置", key))
}
```

### 7. 安全日志

```rust
// 安全的日志记录
fn log_security_event(event: &SecurityEvent) {
    match event {
        SecurityEvent::LoginAttempt { user, success } => {
            // 不记录密码
            info!("登录尝试: user={}, success={}", user, success);
        }
        SecurityEvent::PasswordChange { user } => {
            info!("密码更改: user={}", user);
        }
        SecurityEvent::AccessDenied { user, resource } => {
            warn!("访问拒绝: user={}, resource={}", user, resource);
        }
    }
}

// 敏感数据脱敏
fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() == 2 {
        let user = parts[0];
        let domain = parts[1];
        let masked_user = if user.len() > 2 {
            format!("{}***{}", &user[..1], &user[user.len()-1..])
        } else {
            "***".to_string()
        };
        format!("{}@{}", masked_user, domain)
    } else {
        "***".to_string()
    }
}
```

### 8. 安全头设置

```rust
// HTTP 安全头
fn add_security_headers(response: &mut Response) {
    // 防止点击劫持
    response.headers().insert(
        "X-Frame-Options",
        "DENY".parse().unwrap(),
    );
    
    // 防止 MIME 类型嗅探
    response.headers().insert(
        "X-Content-Type-Options",
        "nosniff".parse().unwrap(),
    );
    
    // XSS 保护
    response.headers().insert(
        "X-XSS-Protection",
        "1; mode=block".parse().unwrap(),
    );
    
    // 内容安全策略
    response.headers().insert(
        "Content-Security-Policy",
        "default-src 'self'".parse().unwrap(),
    );
}
```

## 🚀 运行示例

```bash
# 运行安全模块
cargo run security

# 运行安全测试
cargo test security
```

## 📊 安全检查清单

```
┌─────────────────────────────────────────────────────────────┐
│                     安全检查清单                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  输入验证                                                    │
│  □ 所有用户输入都经过验证                                    │
│  □ 使用参数化查询防止 SQL 注入                               │
│  □ 对输出进行编码防止 XSS                                    │
│  □ 验证文件路径防止路径遍历                                  │
│                                                             │
│  认证授权                                                    │
│  □ 使用安全的密码哈希算法                                    │
│  □ 实现适当的会话管理                                        │
│  □ 使用安全的密码重置流程                                    │
│  □ 实现适当的访问控制                                        │
│                                                             │
│  数据保护                                                    │
│  □ 敏感数据加密存储                                          │
│  □ 使用 HTTPS 传输                                          │
│  □ 安全处理敏感内存数据                                      │
│  □ 不在日志中记录敏感信息                                    │
│                                                             │
│  配置安全                                                    │
│  □ 使用安全的默认配置                                        │
│  □ 禁用不必要的服务                                          │
│  □ 定期更新依赖                                              │
│  □ 使用安全的编译选项                                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 实现一个安全的密码哈希函数
2. 编写输入验证函数防止 SQL 注入
3. 实现敏感数据的内存安全擦除

### 中级
1. 实现完整的用户认证系统
2. 编写安全的文件上传处理
3. 实现 AES 加密解密功能

### 高级
1. 实现安全的会话管理系统
2. 设计一个安全审计日志系统
3. 实现端到端加密通信

## 🔗 相关资源

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust 安全指南](https://rust-lang.github.io/rust-clippy/master/index.html)
- [RustSec 数据库](https://rustsec.org/)

## ⚠️ 常见安全漏洞

### 1. 硬编码密钥
```rust
// ❌ 危险：硬编码密钥
let key = b"my_secret_key_123";

// ✅ 安全：从环境变量读取
let key = std::env::var("ENCRYPTION_KEY")
    .expect("ENCRYPTION_KEY must be set");
```

### 2. 弱随机数
```rust
// ❌ 危险：使用弱随机数生成器
let token: u32 = rand::thread_rng().gen();

// ✅ 安全：使用加密安全的随机数生成器
let mut token = [0u8; 32];
OsRng.fill_bytes(&mut token);
```

### 3. 时序攻击
```rust
// ❌ 危险：可能受时序攻击
if user_input == expected_token {
    // ...
}

// ✅ 安全：使用常量时间比较
use subtle::ConstantTimeEq;
if user_input.ct_eq(expected_token).into() {
    // ...
}
```

## 📊 学习检查清单

- [ ] 理解 Rust 的安全保证
- [ ] 掌握密码哈希
- [ ] 会使用安全随机数
- [ ] 掌握敏感数据处理
- [ ] 理解输入验证
- [ ] 会使用加密解密
- [ ] 理解安全配置
- [ ] 掌握安全日志
