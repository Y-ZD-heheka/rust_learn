//! # 安全编程模块
//!
//! 这个模块演示了Rust的安全编程实践，包括密码学、安全随机数生成、
//! 输入验证和清理、内存安全保证等安全开发的关键要素。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

/// 安全随机数生成
pub fn secure_random_generation() {
    println!("🔐 安全随机数生成：");
    
    println!("⚠️ 安全随机数生成功能由于依赖版本冲突暂时禁用");
    
    // 生成UUID（v4使用随机数）
    let uuid = uuid::Uuid::new_v4();
    println!("🆔 安全UUID: {}", uuid);
    
    // 时间戳验证
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    println!("⏰ 当前时间戳: {}", now);
}

/// 密码学哈希函数演示
pub fn cryptography_hash_functions() {
    println!("🔒 密码学哈希函数：");
    
    use sha2::{Sha256, Sha512, Digest};
    
    let data = b"Rust Security Programming";
    
    // SHA-256哈希
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    println!("🧮 SHA-256: {}", hex::encode(result));
    
    // SHA-512哈希
    let mut hasher512 = Sha512::new();
    hasher512.update(data);
    let result512 = hasher512.finalize();
    println!("🧮 SHA-512: {}", hex::encode(result512));
}

/// HMAC消息认证码
pub fn hmac_message_authentication() {
    println!("✉️ HMAC消息认证码：");
    println!("⚠️ HMAC功能由于依赖版本冲突暂时禁用");
    println!("🔑 密钥: [示例中不输出；应由调用者安全提供]");
    println!("📝 消息内容: [示例消息已省略]");
    println!("🔐 HMAC值: 未计算（功能暂时禁用）");
}

/// Base64编码解码
pub fn base64_encoding_decoding() {
    println!("📦 Base64编码解码：");
    
    // 编码
    let data = b"Rust security programming best practices";
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data);
    println!("📤 原始数据: {}", str::from_utf8(data).unwrap());
    println!("📤 Base64编码: {}", encoded);
    
    // 解码
    if let Ok(decoded) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &encoded) {
        println!("📥 解码数据: {}", str::from_utf8(&decoded).unwrap());
    }
}

/// 输入验证和清理
pub fn input_validation_sanitization() {
    println!("🛡️ 输入验证和清理：");
    
    // 用户名验证
    fn validate_username(username: &str) -> Result<String, String> {
        let clean = username.trim();
        
        if clean.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        
        if clean.len() < 3 || clean.len() > 20 {
            return Err("用户名长度必须在3-20个字符之间".to_string());
        }
        
        // 检查是否只包含字母、数字和下划线
        if !clean.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err("用户名只能包含字母、数字和下划线".to_string());
        }
        
        Ok(clean.to_string())
    }
    
    // 邮箱验证
    fn validate_email(email: &str) -> Result<String, String> {
        let clean = email.trim().to_lowercase();
        
        if clean.is_empty() {
            return Err("邮箱不能为空".to_string());
        }
        
        let email_regex = regex::Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();
        
        if !email_regex.is_match(&clean) {
            return Err("邮箱格式无效".to_string());
        }
        
        Ok(clean)
    }
    
    // 测试用例
    let binding = "a".repeat(25);
    let test_usernames = vec![
        "valid_user",
        "123",
        "user@name", // 包含特殊字符
        &binding, // 过长
        "valid_user123",
        "",
    ];
    
    let test_emails = vec![
        "user@example.com",
        "invalid.email",
        "@example.com",
        "user@",
        "user@example",
        "USER@EXAMPLE.COM",
    ];
    
    println!("🔍 用户名验证测试:");
    for username in test_usernames {
        match validate_username(username) {
            Ok(valid) => println!("  ✅ '{}' -> {}", username, valid),
            Err(e) => println!("  ❌ '{}' -> {}", username, e),
        }
    }
    
    println!("\n🔍 邮箱验证测试:");
    for email in test_emails {
        match validate_email(email) {
            Ok(valid) => println!("  ✅ '{}' -> {}", email, valid),
            Err(e) => println!("  ❌ '{}' -> {}", email, e),
        }
    }
}

/// 安全密码存储
pub fn secure_password_storage() {
    println!("🔑 安全密码存储：");
    
    println!("⚠️ PBKDF2功能由于依赖版本冲突暂时禁用");
    println!("🔒 密码输入: [示例中不回显；应由调用者提供]");
    println!("🧂 盐值: [示例中不展示；应使用每次唯一的随机盐]");
    println!("🔐 PBKDF2哈希: 未计算（功能暂时禁用）");
    
    // 密码验证函数
    fn verify_password(_password: &str, _salt: &str, _stored_hash: &[u8]) -> bool {
        println!("⚠️ 密码验证功能暂时禁用");
        false
    }
    
    // 测试验证
    let is_valid = verify_password("test", "salt", &[0; 32]);
    println!("✅ 密码验证结果: {}", if is_valid { "有效" } else { "无效" });
    
    // 测试错误密码
    let is_wrong_valid = verify_password("wrong_password", "salt", &[0; 32]);
    println!("✅ 错误密码验证: {}", if is_wrong_valid { "有效" } else { "无效" });
}

/// 安全随机字符串生成
pub fn secure_random_strings() {
    println!("🎲 安全随机字符串生成：");
    
    // 生成安全随机字符串
    fn generate_secure_token(length: usize) -> Result<String, String> {
        if length == 0 || length > 1024 {
            return Err("长度必须在1-1024之间".to_string());
        }
        
        println!("⚠️ 安全随机令牌生成功能由于依赖版本冲突暂时禁用");
        Ok(format!("未生成（{}字节令牌占位，功能暂时禁用）", length))
    }
    
    // 生成不同长度的安全令牌
    for len in [16, 32, 64, 128] {
        match generate_secure_token(len) {
            Ok(token_status) => println!("🔑 {}字节安全令牌状态: {}", len, token_status),
            Err(e) => println!("❌ 生成失败: {}", e),
        }
    }
    
    // 生成密码学随机密码
    fn generate_secure_password(length: usize, include_symbols: bool) -> String {
        println!("⚠️ 安全密码生成功能由于依赖版本冲突暂时禁用");
        let complexity = if include_symbols { "含符号" } else { "仅字母数字" };
        format!("未生成（{}位{}密码占位，功能暂时禁用）", length, complexity)
    }
    
    let simple_pwd = generate_secure_password(12, false);
    let complex_pwd = generate_secure_password(16, true);
    
    println!("🔐 简单密码状态: {}", simple_pwd);
    println!("🔐 复杂密码状态: {}", complex_pwd);
}

/// 内存安全保证演示
pub fn memory_safety_guarantees() {
    println!("🛡️ 内存安全保证演示：");
    
    // 演示Rust的内存安全特性
    
    // 1. 缓冲区溢出保护
    println!("\n1️⃣ 缓冲区溢出保护:");
    let mut vec = Vec::with_capacity(5);
    vec.extend_from_slice(&[1, 2, 3, 4, 5]);
    
    // 安全的索引访问
    match vec.get(3) {
        Some(value) => println!("安全索引访问: vec[3] = {}", value),
        None => println!("索引超出范围"),
    }
    
    // 2. 空指针解引用保护
    println!("\n2️⃣ 空指针解引用保护:");
    let option_value: Option<&i32> = None;
    
    match option_value {
        Some(value) => println!("访问值: {}", value),
        None => println!("安全的None检查"),
    }
    
    // 3. 双重释放保护
    println!("\n3️⃣ 双重释放保护:");
    let boxed_value = Box::new(42);
    let moved_value = boxed_value; // 所有权转移
    // println!("{}", boxed_value); // 这会编译错误，防止双重释放
    
    println!("安全的所有权转移: {}", moved_value);
    
    // 4. 数据竞争保护
    println!("\n4️⃣ 数据竞争保护:");
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let shared_data = Arc::new(Mutex::new(0));
    let shared_data_clone = Arc::clone(&shared_data);
    
    let handle = thread::spawn(move || {
        let mut data = shared_data_clone.lock().unwrap();
        *data += 1;
        println!("线程修改后值: {}", *data);
    });
    
    {
        let mut data = shared_data.lock().unwrap();
        *data += 10;
        println!("主线程修改后值: {}", *data);
    }
    
    handle.join().unwrap();
    println!("最终安全值: {}", *shared_data.lock().unwrap());
}

/// 固定流程比较示意
pub fn constant_time_comparison() {
    println!("⏱️ 固定流程比较示意：");
    println!("ℹ️ 此示例仅演示遍历全部字节的比较思路，不应视为生产级常量时间比较实现。");
    
    // 教学示例：长度不同时仍会提前返回，且编译器/平台行为也可能影响真实时序。
    fn length_checked_xor_eq_demo(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        
        let mut result = 0u8;
        for (&x, &y) in a_bytes.iter().zip(b_bytes.iter()) {
            result |= x ^ y;
        }
        
        result == 0
    }
    
    // 测试用例
    let test_cases = vec![
        ("相同输入", "demo-value-a", "demo-value-a", true),
        ("不同但等长输入", "demo-value-a", "demo-value-b", false),
        ("不同长度输入", "short", "much-longer-demo-value", false),
        ("空字符串", "", "", true),
        ("短字符串", "test", "test", true),
    ];
    
    println!("🔍 比较结果测试:");
    for (label, a, b, expected) in test_cases {
        let result = length_checked_xor_eq_demo(a, b);
        let status = if result == expected { "✅" } else { "❌" };
        println!("  {} {} = {} (期望: {})", status, label, result, expected);
    }
}

/// 运行安全编程示例
pub fn run_security_examples() {
    println!("🎯 === 现代化安全编程示例 ===");
    println!();
    
    secure_random_generation();
    println!();
    
    cryptography_hash_functions();
    println!();
    
    hmac_message_authentication();
    println!();
    
    base64_encoding_decoding();
    println!();
    
    input_validation_sanitization();
    println!();
    
    secure_password_storage();
    println!();
    
    secure_random_strings();
    println!();
    
    memory_safety_guarantees();
    println!();
    
    constant_time_comparison();
    
    println!("\n✅ 所有安全编程示例运行完成！");
}
