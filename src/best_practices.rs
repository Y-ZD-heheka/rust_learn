//! # Rust最佳实践模式模块
//!
//! 这个模块演示了Rust编程的最佳实践，包括错误处理、异步编程、
//! 资源管理、性能优化、代码组织等方面的实践原则和模式。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

use anyhow::Context;
use std::time::{Duration, Instant};

/// 现代化错误处理最佳实践
pub fn modern_error_handling_best_practices() {
    println!("⚡ 现代化错误处理最佳实践：");

    // 使用anyhow::Result进行应用程序错误处理
    type AppResult<T> = anyhow::Result<T>;

    // 业务逻辑错误类型
    #[derive(Debug, thiserror::Error)]
    enum BusinessError {
        #[error("用户不存在: {0}")]
        UserNotFound(String),
        #[error("权限不足: {0}")]
        PermissionDenied(String),
        #[error("数据验证失败: {0}")]
        ValidationFailed(String),
    }

    // 使用Result<T, E>处理明确的错误类型
    fn validate_user_input(name: &str, age: u32) -> Result<String, BusinessError> {
        if name.trim().is_empty() {
            return Err(BusinessError::ValidationFailed(
                "用户名不能为空".to_string(),
            ));
        }

        if age < 13 {
            return Err(BusinessError::ValidationFailed(
                "用户年龄必须大于等于13岁".to_string(),
            ));
        }

        Ok(format!("有效用户: {}, 年龄: {}", name, age))
    }

    // 使用anyhow处理应用程序级别错误
    fn process_user_data(user_id: &str) -> AppResult<String> {
        let content = std::fs::read_to_string(format!("data/{}.json", user_id))
            .context(format!("无法读取用户文件: {}", user_id))?;

        let parsed: serde_json::Value =
            serde_json::from_str(&content).context("解析用户数据失败")?;

        let name = parsed
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("用户数据格式错误"))?;

        let age = parsed
            .get("age")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| anyhow::anyhow!("年龄数据格式错误"))?;

        validate_user_input(name, age as u32).map_err(|e| anyhow::anyhow!("验证失败: {}", e))
    }

    // 测试错误处理
    println!("\n🔍 错误处理测试:");

    // 成功案例
    match validate_user_input("张三", 25) {
        Ok(user) => println!("✅ {}", user),
        Err(e) => println!("❌ {}", e),
    }

    // 失败案例
    match validate_user_input("", 12) {
        Ok(user) => println!("✅ {}", user),
        Err(e) => println!("❌ {}", e),
    }
}

/// 异步编程最佳实践
pub async fn async_programming_best_practices() {
    println!("🔄 异步编程最佳实践：");

    use tokio::time::{Duration, sleep};

    // 使用 tokio::spawn 的最佳实践
    async fn background_task(task_id: u32) {
        sleep(Duration::from_millis(100)).await;
        println!("后台任务 {} 完成", task_id);
    }

    // 正确的并发处理
    async fn concurrent_operations() {
        let mut handles = vec![];

        for i in 0..5 {
            let handle = tokio::spawn(background_task(i));
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            if let Err(e) = handle.await {
                eprintln!("任务执行错误: {:?}", e);
            }
        }
    }

    // 超时处理最佳实践
    async fn operation_with_timeout() -> Result<String, tokio::time::error::Elapsed> {
        tokio::time::timeout(Duration::from_millis(50), async {
            sleep(Duration::from_millis(100)).await;
            "慢操作完成".to_string()
        })
        .await
    }

    async fn cancellable_operation() {
        use tokio::sync::watch;

        let (cancel_tx, mut cancel_rx) = watch::channel(false);
        let operation = tokio::spawn(async move {
            let mut steps_completed = 0;

            loop {
                tokio::select! {
                    _ = cancel_rx.changed() => {
                        if *cancel_rx.borrow() {
                            return format!("任务在完成 {} 个步骤后收到取消信号并自行收尾", steps_completed);
                        }
                    }
                    _ = sleep(Duration::from_millis(150)) => {
                        steps_completed += 1;
                        println!("⏳ 长时间操作执行到步骤 {}", steps_completed);

                        if steps_completed == 10 {
                            return "长时间操作自然完成".to_string();
                        }
                    }
                }
            }
        });

        sleep(Duration::from_millis(350)).await;
        println!("📨 主任务发送取消信号");

        if cancel_tx.send(true).is_err() {
            eprintln!("⚠️ 取消信号未送达：任务可能已经结束");
        }

        match operation.await {
            Ok(message) => println!("🛑 {}", message),
            Err(e) => eprintln!("❌ 被取消任务执行异常: {:?}", e),
        }
    }

    println!("\n🚀 异步操作演示:");
    concurrent_operations().await;

    match tokio::time::timeout(Duration::from_millis(200), operation_with_timeout()).await {
        Ok(Ok(result)) => println!("✅ {}", result),
        Ok(Err(e)) => println!("⏰ 超时: {:?}", e),
        Err(e) => println!("❌ 超时: {:?}", e),
    }

    cancellable_operation().await;
}

/// 资源管理最佳实践
pub fn resource_management_best_practices() {
    println!("📦 资源管理最佳实践：");

    use std::sync::Weak;
    use std::sync::{Arc, Mutex};

    // RAII模式的资源管理
    struct Resource {
        data: String,
        _guard: ResourceGuard,
    }

    struct ResourceGuard;
    impl Drop for ResourceGuard {
        fn drop(&mut self) {
            println!("🧹 资源被安全释放");
        }
    }

    impl Resource {
        fn new(data: String) -> Self {
            println!("📥 资源创建: {}", data);
            Self {
                data,
                _guard: ResourceGuard,
            }
        }
    }

    // 自动资源清理
    {
        let resource = Resource::new("数据库连接".to_string());
        println!("使用资源: {}", resource.data);
        // 资源会自动释放
    }

    // 使用Arc进行共享所有权
    println!("\n🔄 共享资源管理:");
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));

    // 克隆引用
    let data_clone = Arc::clone(&shared_data);

    // 在另一个线程中使用（使用std::thread替代tokio::spawn，避免需要异步运行时）
    let handle = std::thread::spawn(move || {
        let mut data = data_clone.lock().unwrap();
        data.push(6);
        println!("子线程修改数据: {:?}", *data);
    });

    // 主线程使用
    {
        let data = shared_data.lock().unwrap();
        println!("主线程读取数据: {:?}", *data);
    }

    // 等待子线程完成
    handle.join().unwrap();

    // 防止循环引用的Weak引用
    println!("\n🔗 弱引用防止循环引用:");

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Weak<Node>>,
    }

    let mut root = Node {
        value: 1,
        children: Vec::new(),
    };

    let child = Arc::new(Node {
        value: 2,
        children: Vec::new(),
    });

    root.children.push(Arc::downgrade(&child));
    println!("节点结构创建成功");
}

/// 性能优化最佳实践
pub fn performance_optimization_best_practices() {
    println!("⚡ 性能优化最佳实践：");

    use std::mem::{align_of, size_of};

    // 1. 数据结构优化
    println!("\n1️⃣ 数据结构优化:");

    // 优化前：未对齐的结构体
    struct UnoptimizedStruct {
        a: u8,  // 1字节
        b: u64, // 8字节 (需要8字节对齐)
        c: u32, // 4字节
        d: u16, // 2字节
        e: u8,  // 1字节
    }

    // 优化后：内存对齐的结构体
    #[repr(C)] // 使用C布局更好的内存对齐
    struct OptimizedStruct {
        b: u64, // 8字节
        c: u32, // 4字节
        d: u16, // 2字节
        a: u8,  // 1字节
        e: u8,  // 1字节
    }

    println!(
        "未优化结构体大小: {} 字节, 对齐: {} 字节",
        size_of::<UnoptimizedStruct>(),
        align_of::<UnoptimizedStruct>()
    );
    println!(
        "优化结构体大小: {} 字节, 对齐: {} 字节",
        size_of::<OptimizedStruct>(),
        align_of::<OptimizedStruct>()
    );

    // 2. 零成本抽象
    println!("\n2️⃣ 零成本抽象:");

    fn expensive_iteration(data: &[i32]) -> i32 {
        data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum()
    }

    let data: Vec<i32> = (1..1000).collect();
    let start = Instant::now();
    let result = expensive_iteration(&data);
    let duration = start.elapsed();

    println!("迭代器链处理结果: {}, 耗时: {:?}", result, duration);

    // 3. 避免不必要的分配
    println!("\n3️⃣ 避免不必要的分配:");

    fn process_data_prealloc(data: &[i32], buffer: &mut Vec<i32>) {
        buffer.clear();
        buffer.extend(data.iter().filter(|&&x| x > 50).cloned());
    }

    let mut buffer = Vec::with_capacity(100); // 预分配容量
    let data = (1..100).collect::<Vec<_>>();

    let start = Instant::now();
    for _ in 0..1000 {
        process_data_prealloc(&data, &mut buffer);
    }
    let duration = start.elapsed();
    println!("预分配缓冲区 1000次操作耗时: {:?}", duration);

    // 4. 缓存友好的数据访问
    println!("\n4️⃣ 缓存友好的数据访问:");

    // 行主序（缓存友好）
    let row_major = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];

    let start = Instant::now();
    let mut sum = 0;
    for i in 0..3 {
        for j in 0..4 {
            sum += row_major[i][j]; // 缓存友好的访问模式
        }
    }
    let duration = start.elapsed();
    println!("行主序访问和: {}, 耗时: {:?}", sum, duration);
}

/// API设计最佳实践
pub fn api_design_best_practices() {
    println!("🎨 API设计最佳实践：");

    // 1. 清晰的API设计
    #[derive(Debug, Clone)]
    pub struct Config {
        pub host: String,
        pub port: u16,
        pub timeout: Duration,
        pub retries: u32,
    }

    impl Config {
        // 构造函数提供默认值
        pub fn default() -> Self {
            Self {
                host: "localhost".to_string(),
                port: 8080,
                timeout: Duration::from_secs(30),
                retries: 3,
            }
        }

        // 构建器模式支持链式调用
        pub fn host(&mut self, host: impl Into<String>) -> &mut Self {
            self.host = host.into();
            self
        }

        pub fn port(&mut self, port: u16) -> &mut Self {
            self.port = port;
            self
        }

        pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
            self.timeout = timeout;
            self
        }

        pub fn retries(&mut self, retries: u32) -> &mut Self {
            self.retries = retries;
            self
        }
    }

    let mut binding = Config::default();
    let config = binding
        .host("example.com")
        .port(443)
        .timeout(Duration::from_secs(60))
        .retries(5);

    println!("🛠️ 构建的配置: {:?}", config);

    // 2. 错误类型设计
    #[derive(Debug, thiserror::Error)]
    pub enum ApiError {
        #[error("网络错误: {source}")]
        Network {
            #[from]
            source: reqwest::Error,
        },
        #[error("服务器错误 {code}: {message}")]
        Server { code: u16, message: String },
        #[error("超时错误")]
        Timeout,
        #[error("无效响应")]
        InvalidResponse,
    }

    // 3. Result类型别名
    type ApiResult<T> = Result<T, ApiError>;

    // 4. 泛型约束最佳实践
    pub trait Validator {
        type Item;

        fn validate(&self, item: &Self::Item) -> bool;
        fn validate_with_context(&self, item: &Self::Item, context: &str) -> Result<(), String>;
    }

    // 5. 特征对象使用时机
    pub trait Processor {
        fn process(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    }

    struct HashProcessor;
    impl Processor for HashProcessor {
        fn process(&self, data: &[u8]) -> Result<Vec<u8>, String> {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(data);
            Ok(hasher.finalize().to_vec())
        }
    }

    // 使用特征对象进行动态分发
    fn dynamic_process(processor: &dyn Processor, data: &[u8]) -> Result<Vec<u8>, String> {
        processor.process(data)
    }

    let processor = HashProcessor;
    let data = b"Hello, World!";
    match dynamic_process(&processor, data) {
        Ok(result) => println!("🔐 处理结果: {}", hex::encode(result)),
        Err(e) => println!("❌ 处理失败: {}", e),
    }
}

/// 测试最佳实践
pub fn testing_best_practices() {
    println!("🧪 测试最佳实践：");

    // 1. 单元测试组织
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validate_email_correct_format() {
            assert!(crate::testing::validate_email("user@example.com"));
            assert!(!crate::testing::validate_email("user@domain"));
        }

        #[test]
        fn test_config_functionality() {
            // 测试配置功能的简单示例
            let timeout_duration = Duration::from_secs(10);
            assert!(timeout_duration.as_secs() > 0);
        }

        #[test]
        #[should_panic]
        fn panics_on_invalid_input() {
            panic!("预期的 panic");
        }
    }

    // 2. 属性思维概念演示
    fn reverse_twice<T: Clone + PartialEq>(items: &[T]) -> bool {
        let reversed: Vec<_> = items.iter().cloned().rev().collect();
        let reversed_twice: Vec<_> = reversed.iter().cloned().rev().collect();
        items.iter().eq(reversed_twice.iter())
    }

    println!("🔍 属性思维示例:");
    println!("  这里使用固定样本说明不变量，不等同于真正的 property-based testing 框架。");

    // 测试整数列表
    let int_cases = vec![vec![1, 2, 3], vec![1], vec![]];

    for items in int_cases {
        let result = reverse_twice(&items);
        println!(
            "  整数列表 {:?}: {}",
            items,
            if result { "✅" } else { "❌" }
        );
    }

    // 测试字符列表
    let char_items = vec!['a', 'b', 'c'];
    let result = reverse_twice(&char_items);
    println!("  字符列表: {:?}", if result { "✅" } else { "❌" });

    // 3. 粗粒度耗时观察
    fn timed_iteration_observation() {
        use std::time::Instant;

        let data: Vec<i32> = (1..10000).collect();

        let start = Instant::now();
        let sum: i32 = data.iter().sum();
        let sum_duration = start.elapsed();

        let start = Instant::now();
        let sum2: i32 = data.iter().fold(0, |acc, &x| acc + x);
        let fold_duration = start.elapsed();

        println!("📊 迭代器sum耗时观察: {:?}, 结果: {}", sum_duration, sum);
        println!("📊 fold耗时观察: {:?}, 结果: {}", fold_duration, sum2);
        println!("📌 这些数字会受到环境影响，不能替代正式 benchmark。 ");
    }

    timed_iteration_observation();
}

/// 文档和注释最佳实践
pub fn documentation_best_practices() {
    println!("📚 文档和注释最佳实践：");

    /// 计算两点之间的距离
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.distance_to(&p2), 5.0);
    /// ```
    ///
    /// # Panics
    ///
    /// 如果坐标值不是有限数字，此函数会 panic
    #[derive(Debug, Clone, Copy)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        /// 创建新的点
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        /// 计算到另一个点的距离
        pub fn distance_to(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("📍 点间距离: {:.1}", p1.distance_to(&p2));

    // 结构化注释
    /**
     * 处理用户数据的函数
     *
     * # 参数
     *
     * * `user_id` - 用户的唯一标识符
     * * `data` - 要处理的用户数据
     *
     * # 返回值
     *
     * 返回处理结果，失败时返回错误信息
     *
     * # 示例
     *
     * ```ignore
     * let result = process_user_data("123", "user data");
     * match result {
     *     Ok(processed) => println!("处理成功: {}", processed),
     *     Err(e) => println!("处理失败: {}", e),
     * }
     * ```
     */
    pub fn process_user_data(user_id: &str, data: &str) -> Result<String, String> {
        if user_id.is_empty() {
            return Err("用户ID不能为空".to_string());
        }

        if data.is_empty() {
            return Err("数据不能为空".to_string());
        }

        Ok(format!("用户 {} 的数据处理完成: {}", user_id, data.len()))
    }

    match process_user_data("user123", "重要数据") {
        Ok(result) => println!("✅ {}", result),
        Err(e) => println!("❌ {}", e),
    }
}

/// 运行最佳实践示例
pub fn run_best_practices_examples() {
    println!("🎯 === Rust最佳实践模式示例 ===");
    println!();

    modern_error_handling_best_practices();
    println!();

    println!("异步编程最佳实践:");
    match tokio::runtime::Runtime::new() {
        Ok(rt) => {
            rt.block_on(async {
                async_programming_best_practices().await;
            });
        }
        Err(e) => eprintln!("❌ 无法创建 Tokio 运行时: {}", e),
    }
    println!();

    resource_management_best_practices();
    println!();

    performance_optimization_best_practices();
    println!();

    api_design_best_practices();
    println!();

    testing_best_practices();
    println!();

    documentation_best_practices();

    println!("\n✅ 所有最佳实践示例运行完成！");
}
