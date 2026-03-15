//! 教学主题四：资源加载与外部服务错误。
//!
//! 这一层将工程实践中更贴近“系统边界”的错误放在一起：
//! - 网络请求与状态码分类
//! - 资源加载与回退策略
//! - 第三方服务故障与降级处理

use std::collections::HashMap;
use std::fmt;

use super::fundamentals::{log_demo_error, log_demo_message};

/// 演示真实网络请求错误处理。
pub fn network_error_handling() {
    println!("🌐 网络请求错误处理：");

    #[derive(Debug)]
    enum NetworkError {
        ConnectionTimeout,
        BadRequest { status: u16, message: String },
        ServerError { status: u16, message: String },
        Transport(String),
        JsonParsing { endpoint: String },
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
                Self::Transport(message) => write!(f, "网络错误: {}", message),
                Self::JsonParsing { endpoint } => write!(f, "接口 {} 的 JSON 解析失败", endpoint),
            }
        }
    }

    fn simulate_network_request(user_id: u32) -> Result<String, NetworkError> {
        match user_id {
            0 => Err(NetworkError::BadRequest {
                status: 400,
                message: "无效的用户 ID".to_string(),
            }),
            2 => Err(NetworkError::JsonParsing {
                endpoint: "/users/2".to_string(),
            }),
            999 => Err(NetworkError::ConnectionTimeout),
            1000 => Err(NetworkError::ServerError {
                status: 500,
                message: "内部服务器错误".to_string(),
            }),
            2000 => Err(NetworkError::Transport("TLS 握手失败".to_string())),
            _ => Ok(format!("用户 {} 的数据", user_id)),
        }
    }

    for user_id in [0, 1, 2, 999, 1000, 2000] {
        match simulate_network_request(user_id) {
            Ok(data) => println!("✅ 获取成功: {}", data),
            Err(NetworkError::BadRequest { status, message }) => {
                println!("❌ 客户端错误 {}: {}", status, message);
                println!("💡 检查请求参数是否正确");
            }
            Err(NetworkError::ServerError { status, message }) => {
                println!("❌ 服务器错误 {}: {}", status, message);
                println!("💡 建议记录监控并稍后重试");
            }
            Err(NetworkError::ConnectionTimeout) => {
                log_demo_error("网络请求", &NetworkError::ConnectionTimeout);
                println!("💡 可以增加重试与退避策略");
            }
            Err(error) => {
                log_demo_error("网络请求", &error);
                println!("💡 将错误返回给上层后，可再决定是否降级或重试");
            }
        }
    }
}

/// 演示资源加载错误处理。
pub fn resource_loading_error_handling() {
    println!("📦 资源加载错误处理：");

    #[derive(Debug)]
    enum ResourceError {
        FileNotFound(String),
        NetworkUnavailable(String),
        Timeout,
        CorruptedData(String),
        PermissionDenied(String),
        InsufficientMemory,
    }

    impl fmt::Display for ResourceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::FileNotFound(path) => write!(f, "文件未找到: {}", path),
                Self::NetworkUnavailable(message) => write!(f, "网络不可用: {}", message),
                Self::Timeout => write!(f, "加载超时"),
                Self::CorruptedData(source) => write!(f, "数据损坏: {}", source),
                Self::PermissionDenied(resource) => write!(f, "权限拒绝: {}", resource),
                Self::InsufficientMemory => write!(f, "内存不足"),
            }
        }
    }

    struct ResourceLoader {
        cache: HashMap<String, Vec<u8>>,
    }

    impl ResourceLoader {
        fn new() -> Self {
            Self {
                cache: HashMap::new(),
            }
        }

        fn load_image(&mut self, path: &str) -> Result<Vec<u8>, ResourceError> {
            if let Some(data) = self.cache.get(path) {
                return Ok(data.clone());
            }

            match path {
                "missing.png" => Err(ResourceError::FileNotFound(path.to_string())),
                "corrupted.jpg" => Err(ResourceError::CorruptedData(path.to_string())),
                "slow_network.webp" => {
                    Err(ResourceError::NetworkUnavailable("网络延迟".to_string()))
                }
                "timeout.webp" => Err(ResourceError::Timeout),
                _ => {
                    let data = vec![255, 216, 255, 224];
                    self.cache.insert(path.to_string(), data.clone());
                    Ok(data)
                }
            }
        }

        fn load_font(&mut self, font_name: &str) -> Result<Vec<u8>, ResourceError> {
            match font_name {
                "admin_font.ttf" => {
                    Err(ResourceError::PermissionDenied("管理员字体".to_string()))
                }
                "large_font.ttf" => Err(ResourceError::InsufficientMemory),
                "default.ttf" => {
                    let data = vec![0, 1, 2, 3];
                    self.cache.insert(font_name.to_string(), data.clone());
                    Ok(data)
                }
                _ => Err(ResourceError::FileNotFound(format!("字体: {}", font_name))),
            }
        }
    }

    let mut loader = ResourceLoader::new();

    println!("图片加载测试:");
    for image in [
        "avatar.png",
        "missing.png",
        "corrupted.jpg",
        "slow_network.webp",
        "timeout.webp",
    ] {
        match loader.load_image(image) {
            Ok(data) => println!("✅ 图片 {} 加载成功，大小: {} 字节", image, data.len()),
            Err(error) => {
                log_demo_error("资源加载", &format!("图片 {} 加载失败: {}", image, error));
                match error {
                    ResourceError::FileNotFound(_) => println!("🔄 使用默认图片"),
                    ResourceError::CorruptedData(_) => println!("🔄 重新下载损坏的图片"),
                    ResourceError::NetworkUnavailable(_) | ResourceError::Timeout => {
                        println!("🔄 尝试从 CDN 或缓存加载")
                    }
                    _ => println!("🔄 跳过此资源"),
                }
            }
        }
    }

    println!("\n字体加载测试:");
    for font in ["default.ttf", "admin_font.ttf", "large_font.ttf"] {
        match loader.load_font(font) {
            Ok(_) => println!("✅ 字体 {} 加载成功", font),
            Err(error) => {
                log_demo_error("资源加载", &format!("字体 {} 加载失败: {}", font, error));
                println!("🔄 回退到系统默认字体");
            }
        }
    }
}

/// 演示第三方服务错误处理。
pub fn external_service_error_handling() {
    println!("🔌 第三方服务错误处理：");

    #[derive(Debug)]
    enum ServiceError {
        RateLimitExceeded { service: String, retry_after: u32 },
        AuthenticationFailed { service: String },
        ServiceUnavailable { service: String },
        QuotaExceeded { service: String },
        BusinessRejected { service: String, reason: String },
    }

    impl fmt::Display for ServiceError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::RateLimitExceeded {
                    service,
                    retry_after,
                } => write!(f, "{} 速率限制，等待 {} 秒后重试", service, retry_after),
                Self::AuthenticationFailed { service } => write!(f, "{} 认证失败", service),
                Self::ServiceUnavailable { service } => write!(f, "{} 服务不可用", service),
                Self::QuotaExceeded { service } => write!(f, "{} 配额已用完", service),
                Self::BusinessRejected { service, reason } => {
                    write!(f, "{} 业务拒绝: {}", service, reason)
                }
            }
        }
    }

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
                "expired_card" => Err(ServiceError::BusinessRejected {
                    service: "支付网关".to_string(),
                    reason: "信用卡已过期".to_string(),
                }),
                "insufficient_funds" => Err(ServiceError::BusinessRejected {
                    service: "支付网关".to_string(),
                    reason: "余额不足".to_string(),
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

    fn handle_payment_error(reason: &str) {
        match reason {
            "余额不足" => println!("🔄 提示用户充值"),
            "信用卡已过期" => println!("🔄 提示用户更新支付信息"),
            _ => println!("🔄 记录错误并人工处理"),
        }
    }

    fn handle_service_error(error: ServiceError) {
        log_demo_error("第三方服务", &error);
        match error {
            ServiceError::RateLimitExceeded { retry_after, .. } => {
                println!("🔄 等待 {} 秒后重试", retry_after);
            }
            ServiceError::AuthenticationFailed { .. } => {
                println!("🔄 刷新认证令牌并重试");
            }
            ServiceError::ServiceUnavailable { .. } => {
                println!("🔄 切换到备用服务");
            }
            ServiceError::QuotaExceeded { .. } => {
                println!("🔄 延迟处理，等待配额重置");
            }
            ServiceError::BusinessRejected { reason, .. } => {
                handle_payment_error(&reason);
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
            Err(error) => {
                handle_service_error(error);
                log_demo_message("第三方服务", "上层可根据错误类别决定重试、降级或直接返回失败");
            }
        }
        println!();
    }

    let test_cases = [
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

    for (service, data) in test_cases {
        process_service_call(service, data);
    }
}
