//! # 测试和文档模块
//!
//! 这个模块演示了Rust的测试功能和文档生成。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

use std::time::Instant;

/// 一个现代数学函数，用于测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
/// 另一个现代函数，用于测试私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/// 现代化验证函数 - 改进的邮箱验证
pub fn validate_email(email: &str) -> bool {
    // 基本验证：必须包含 @ 和 .
    // @ 不能在开头或结尾
    // 必须包含至少一个 . 在 @ 之后
    if !email.contains('@') || !email.contains('.') {
        return false;
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    // 本地部分不能为空
    if local.is_empty() {
        return false;
    }

    // 域名部分必须包含 . 且不能在开头或结尾
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    // 总长度检查
    email.len() >= 5
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
        let user = User::new("张三".to_string(), "zhangsan@example.com".to_string(), 25)
            .expect("Failed to create user for test");

        assert_eq!(user.name, "张三");
        assert_eq!(user.email, "zhangsan@example.com");
        assert_eq!(user.age, 25);
        assert!(user.is_adult());
    }

    #[test]
    fn test_user_creation_invalid_email() {
        let result = User::new("李四".to_string(), "invalid-email".to_string(), 20);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "邮箱格式不正确");
    }

    #[test]
    fn test_user_creation_minor() {
        let result = User::new("小明".to_string(), "xiaoming@example.com".to_string(), 12);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户年龄必须大于等于13岁");
    }

    #[test]
    fn test_user_methods() {
        let user = User::new("王五".to_string(), "wangwu@example.com".to_string(), 30)
            .expect("Failed to create user for test");

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
                20 + (i % 50) as u8,
            )
            .expect("Failed to create user in performance test");

            manager.add_user(user).expect("Failed to add user");
        }

        assert_eq!(manager.user_count(), 1000);
        assert_eq!(manager.get_adult_users().len(), 1000);
    }
}

/// 现代化条件编译测试
#[cfg(test)]
mod conditional_tests {
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

/// 演示企业级测试策略
pub fn enterprise_testing_strategies() {
    println!("🏢 企业级测试策略：");

    // 简单的计算器测试案例
    pub struct Calculator {
        history: Vec<f64>,
    }

    impl Calculator {
        pub fn new() -> Self {
            Self {
                history: Vec::new(),
            }
        }

        pub fn add(&mut self, a: f64, b: f64) -> f64 {
            let result = a + b;
            self.history.push(result);
            result
        }

        pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
            let result = a - b;
            self.history.push(result);
            result
        }

        pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
            let result = a * b;
            self.history.push(result);
            result
        }

        pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                return Err("除数不能为零".to_string());
            }
            let result = a / b;
            self.history.push(result);
            Ok(result)
        }

        pub fn get_history(&self) -> &[f64] {
            &self.history
        }
    }

    // 企业级测试场景
    let mut calc = Calculator::new();

    println!("📊 基础运算测试:");
    assert_eq!(calc.add(2.0, 3.0), 5.0);
    assert_eq!(calc.subtract(10.0, 4.0), 6.0);
    assert_eq!(calc.multiply(3.0, 4.0), 12.0);
    assert_eq!(
        calc.divide(15.0, 3.0).expect("Division should succeed"),
        5.0
    );

    println!("✅ 基础运算测试通过");

    // 错误处理测试
    assert!(calc.divide(10.0, 0.0).is_err());
    println!("✅ 错误处理测试通过");

    // 历史记录测试
    assert_eq!(calc.get_history().len(), 4);
    println!("✅ 历史记录功能测试通过");

    println!("📊 企业级测试策略演示完成");
}

/// 演示属性测试基础
pub fn property_based_testing_basics() {
    println!("🎯 属性测试基础：");

    // 属性测试函数：反转两次应该得到原值
    fn reverse_twice<T: Clone + std::cmp::PartialEq>(items: &[T]) -> bool {
        let reversed: Vec<_> = items.iter().cloned().rev().collect();
        let reversed_twice: Vec<_> = reversed.iter().cloned().rev().collect();
        items.iter().eq(reversed_twice.iter())
    }

    // 属性测试：交换律
    #[allow(clippy::eq_op)]
    fn addition_commutative(a: i32, b: i32) -> bool {
        a + b == b + a
    }

    // 属性测试：结合律
    fn addition_associative(a: i32, b: i32, c: i32) -> bool {
        (a + b) + c == a + (b + c)
    }

    // 属性测试：乘法分配律
    fn multiplication_distributive(a: i32, b: i32, c: i32) -> bool {
        a * (b + c) == a * b + a * c
    }

    // 属性测试：质数检查
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

    // 属性测试：阶乘函数的逆函数
    fn factorial_property(n: u32) -> bool {
        if n == 0 || n == 1 {
            return true;
        }
        let result = factorial(n);
        result > 0 && result >= n as u64
    }

    fn factorial(n: u32) -> u64 {
        (1..=n as u64).product::<u64>()
    }

    println!("🔍 属性测试示例:");

    // 1. 反转测试
    let reverse_test_cases = vec![
        (vec![1, 2, 3, 4, 5], "数字列表"),
        (vec![0], "单元素列表"),
        (vec![], "空列表"),
        (vec![1, 2, 3], "整数列表"),
    ];

    for (items, desc) in reverse_test_cases {
        let result = reverse_twice(&items);
        println!(
            "  反转测试 - {}: {}",
            desc,
            if result { "✅ 通过" } else { "❌ 失败" }
        );
    }

    // 2. 算术运算属性
    println!("\n  算术运算属性:");
    let arithmetic_tests = vec![((5, 10), "交换律"), ((3, 7), "结合律"), ((4, 2), "分配律")];

    for (data, desc) in arithmetic_tests {
        let result = match desc {
            "交换律" => {
                let (a, b) = data;
                addition_commutative(a, b)
            }
            "结合律" => {
                let (a, b) = data;
                addition_associative(a, b, 0) // 第三个参数占位符
            }
            "分配律" => {
                let (a, b) = data;
                multiplication_distributive(a, b, 0) // 第三个参数占位符
            }
            _ => false,
        };
        println!(
            "    {}: {}",
            desc,
            if result { "✅ 通过" } else { "❌ 失败" }
        );
    }

    // 3. 质数属性测试
    println!("\n  质数属性测试:");
    let prime_tests = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for &num in &prime_tests {
        let result = is_prime(num);
        println!(
            "    {} 是质数: {}",
            num,
            if result { "✅ 正确" } else { "❌ 错误" }
        );
    }

    // 4. 阶乘属性测试
    println!("\n  阶乘属性测试:");
    for n in 0..10 {
        let result = factorial_property(n);
        println!(
            "    阶乘({}) 属性: {}",
            n,
            if result { "✅ 通过" } else { "❌ 失败" }
        );
    }

    println!("📊 属性测试演示完成");
}

/// 演示性能测试和基准测试
pub fn performance_testing_examples() {
    println!("⚡ 性能测试和基准测试：");

    use std::time::Instant;

    // 大数据集性能测试
    let large_dataset: Vec<i64> = (1..100000).collect();

    let start_time = Instant::now();
    let result: i64 = large_dataset
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    let processing_time = start_time.elapsed();

    println!("📈 大数据集处理性能:");
    println!("  数据量: {} 个元素", large_dataset.len());
    println!("  处理结果: {}", result);
    println!("  处理时间: {:.2}ms", processing_time.as_millis());

    // 字符串操作性能测试
    fn string_operations_performance() {
        let start = Instant::now();

        let mut result = String::with_capacity(10000 * 7); // 预分配足够空间
        for i in 0..10000 {
            result.push_str("Item ");
            result.push_str(&i.to_string());
            result.push(' ');
        }

        let processing_time = start.elapsed();
        println!("📈 字符串操作性能:");
        println!("  操作次数: 10000");
        println!("  结果长度: {} 字符", result.len());
        println!("  处理时间: {:.2}ms", processing_time.as_millis());
    }

    string_operations_performance();

    // 数据结构性能对比
    println!("\n📊 数据结构性能对比:");

    // Vec vs HashMap 查找性能
    let data: Vec<i32> = (1..10000).collect();
    let hash_map: std::collections::HashMap<i32, i32> = data
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, val)| (val, i as i32))
        .collect();

    // Vec 查找性能
    let start_vec = Instant::now();
    let found_in_vec = data.iter().find(|&&x| x == 5000);
    let vec_time = start_vec.elapsed();

    // HashMap 查找性能
    let start_map = Instant::now();
    let found_in_map = hash_map.get(&5000);
    let map_time = start_map.elapsed();

    println!(
        "  Vec 查找 5000: {} (耗时: {:.2}μs)",
        if found_in_vec.is_some() {
            "找到"
        } else {
            "未找到"
        },
        vec_time.as_micros()
    );
    println!(
        "  HashMap 查找 5000: {} (耗时: {:.2}μs)",
        if found_in_map.is_some() {
            "找到"
        } else {
            "未找到"
        },
        map_time.as_micros()
    );

    // 排序算法性能对比
    println!("\n📊 排序算法性能对比:");
    let mut data_to_sort = (1..1000).rev().collect::<Vec<_>>();
    let data_copy = data_to_sort.clone();

    // 冒泡排序
    let start_bubble = Instant::now();
    bubble_sort(&mut data_to_sort);
    let bubble_time = start_bubble.elapsed();
    println!("  冒泡排序: {:.2}ms", bubble_time.as_millis());

    // 快速排序
    let start_quick = Instant::now();
    let mut quick_data = data_copy.clone();
    let len = quick_data.len();
    quick_sort(&mut quick_data, 0, len - 1);
    let quick_time = start_quick.elapsed();
    println!("  快速排序: {:.2}ms", quick_time.as_millis());

    // Rust标准库排序
    let start_std = Instant::now();
    let mut std_data = data_copy.clone();
    std_data.sort();
    let std_time = start_std.elapsed();
    println!("  标准库排序: {:.2}ms", std_time.as_millis());

    // 内存使用性能测试
    println!("\n📊 内存使用性能测试:");
    memory_performance_test();

    println!("📊 性能测试完成");
}

// 冒泡排序实现
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 快速排序实现
fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);
        if pi > 0 {
            quick_sort(arr, low, pi - 1);
        }
        quick_sort(arr, pi + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

// 内存性能测试
fn memory_performance_test() {
    // 栈分配 vs 堆分配
    let start_stack = Instant::now();
    let _stack_array: [i32; 10000] = [0; 10000];
    let stack_time = start_stack.elapsed();

    let start_heap = Instant::now();
    let heap_array = vec![0i32; 10000];
    let heap_time = start_heap.elapsed();

    println!("  栈分配 10000 个 i32: {:.2}μs", stack_time.as_micros());
    println!("  堆分配 10000 个 i32: {:.2}μs", heap_time.as_micros());

    // 避免优化
    println!("  栈数组第一个元素: {}", _stack_array[0]);
    println!("  堆数组第一个元素: {}", heap_array[0]);
}

/// 基准测试示例
#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[test]
    fn benchmark_string_concatenation() {
        let iterations = 1000;

        // 使用 push_str
        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(300); // 预分配空间
            for i in 0..100 {
                s.push_str(&i.to_string());
            }
        }
        let push_str_time = start.elapsed();

        // 使用 format!
        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(300); // 预分配空间
            for i in 0..100 {
                s.push_str(&i.to_string());
            }
        }
        let format_time = start.elapsed();

        println!("push_str: {:.2}ms", push_str_time.as_millis());
        println!("format!: {:.2}ms", format_time.as_millis());
    }

    #[test]
    fn benchmark_data_structure_operations() {
        let data_size = 10000;

        // Vec 插入
        let start = Instant::now();
        let mut vec = Vec::new();
        for i in 0..data_size {
            vec.push(i);
        }
        let vec_insert_time = start.elapsed();

        // HashMap 插入
        let start = Instant::now();
        let mut hashmap = std::collections::HashMap::new();
        for i in 0..data_size {
            hashmap.insert(i, i);
        }
        let hashmap_insert_time = start.elapsed();

        println!(
            "Vec 插入 {} 元素: {:.2}ms",
            data_size,
            vec_insert_time.as_millis()
        );
        println!(
            "HashMap 插入 {} 元素: {:.2}ms",
            data_size,
            hashmap_insert_time.as_millis()
        );
    }

    #[test]
    fn benchmark_iteration_patterns() {
        // 减小数据范围，避免 i32 溢出
        // 使用 1..=10000 而不是 1..100000
        // 因为 1..100000 的和是 4999950000，超过了 i32::MAX (2147483647)
        let data: Vec<i32> = (1..=10000).collect();

        // for 循环
        let start = Instant::now();
        let mut sum1: i64 = 0; // 使用 i64 避免溢出
        for &item in &data {
            sum1 += item as i64;
        }
        let for_loop_time = start.elapsed();

        // iter().sum()
        let start = Instant::now();
        let sum2: i64 = data.iter().map(|&x| x as i64).sum();
        let sum_time = start.elapsed();

        // 迭代器模式
        let start = Instant::now();
        let sum3: i64 = data.iter().fold(0i64, |acc, &x| acc + x as i64);
        let fold_time = start.elapsed();

        println!(
            "for 循环求和: {:.2}ms, 结果: {}",
            for_loop_time.as_millis(),
            sum1
        );
        println!(
            "iter().sum() 求和: {:.2}ms, 结果: {}",
            sum_time.as_millis(),
            sum2
        );
        println!("fold 求和: {:.2}ms, 结果: {}", fold_time.as_millis(), sum3);

        assert_eq!(sum1, sum2);
        assert_eq!(sum1, sum3);
    }
}

/// 演示集成测试场景
pub fn integration_testing_scenarios() {
    println!("🔗 集成测试场景：");

    // 模拟订单处理系统
    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: u32,
        pub items: Vec<OrderItem>,
        pub total: f64,
        pub status: OrderStatus,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product_id: u32,
        pub quantity: u32,
        pub price: f64,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum OrderStatus {
        Pending,
        Confirmed,
        Cancelled,
    }

    // 订单处理服务
    pub struct OrderProcessor {
        orders: std::collections::HashMap<u32, Order>,
        next_id: u32,
    }

    impl OrderProcessor {
        pub fn new() -> Self {
            Self {
                orders: std::collections::HashMap::new(),
                next_id: 1,
            }
        }

        pub fn create_order(&mut self, items: Vec<OrderItem>) -> Result<u32, String> {
            if items.is_empty() {
                return Err("订单不能为空".to_string());
            }

            let total: f64 = items
                .iter()
                .map(|item| item.price * item.quantity as f64)
                .sum();

            let order = Order {
                id: self.next_id,
                items: items.clone(),
                total,
                status: OrderStatus::Pending,
            };

            self.orders.insert(self.next_id, order);
            let order_id = self.next_id;
            self.next_id += 1;

            Ok(order_id)
        }

        pub fn confirm_order(&mut self, order_id: u32) -> Result<(), String> {
            if let Some(order) = self.orders.get_mut(&order_id) {
                match order.status {
                    OrderStatus::Pending => {
                        order.status = OrderStatus::Confirmed;
                        Ok(())
                    }
                    _ => Err("订单状态不允许确认".to_string()),
                }
            } else {
                Err("订单不存在".to_string())
            }
        }

        pub fn get_order(&self, order_id: u32) -> Option<&Order> {
            self.orders.get(&order_id)
        }
    }

    // 集成测试场景
    let mut processor = OrderProcessor::new();

    println!("📦 订单处理系统集成测试:");

    // 测试场景1：正常订单流程
    let test_items = vec![
        OrderItem {
            product_id: 1,
            quantity: 2,
            price: 29.99,
        },
        OrderItem {
            product_id: 2,
            quantity: 1,
            price: 99.99,
        },
    ];

    match processor.create_order(test_items) {
        Ok(order_id) => {
            println!("✅ 订单创建成功，ID: {}", order_id);

            match processor.confirm_order(order_id) {
                Ok(_) => {
                    println!("✅ 订单确认成功");

                    if let Some(order) = processor.get_order(order_id) {
                        println!(
                            "📊 订单详情: 状态={:?}, 总金额=${:.2}",
                            order.status, order.total
                        );
                    }
                }
                Err(e) => println!("❌ 订单确认失败: {}", e),
            }
        }
        Err(e) => println!("❌ 订单创建失败: {}", e),
    }

    // 测试场景2：空订单（应该失败）
    match processor.create_order(vec![]) {
        Ok(_) => println!("❌ 空订单不应该创建成功"),
        Err(e) => println!("✅ 空订单正确拒绝: {}", e),
    }

    println!("📊 集成测试完成");
}

/// 演示测试驱动开发（TDD）示例
pub fn test_driven_development_example() {
    println!("🔄 测试驱动开发（TDD）示例：");

    // 首先定义要测试的功能（计算器）
    pub struct Calculator {
        history: Vec<f64>,
    }

    impl Calculator {
        pub fn new() -> Self {
            Self {
                history: Vec::new(),
            }
        }

        pub fn add(&mut self, a: f64, b: f64) -> f64 {
            let result = a + b;
            self.history.push(result);
            result
        }

        pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
            let result = a - b;
            self.history.push(result);
            result
        }

        pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
            let result = a * b;
            self.history.push(result);
            result
        }

        pub fn divide(&mut self, a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                return Err("除数不能为零".to_string());
            }
            let result = a / b;
            self.history.push(result);
            Ok(result)
        }

        pub fn get_history(&self) -> &[f64] {
            &self.history
        }

        pub fn clear_history(&mut self) {
            self.history.clear();
        }
    }

    // TDD测试用例
    println!("🔬 TDD测试用例:");

    let mut calc = Calculator::new();

    // 测试基础运算
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

    // 测试错误情况
    assert!(calc.divide(10.0, 0.0).is_err());
    println!("✅ 除零错误处理测试通过");

    // 测试历史记录功能
    assert_eq!(calc.get_history().len(), 4);
    println!("✅ 历史记录功能测试通过");

    calc.clear_history();
    assert_eq!(calc.get_history().len(), 0);
    println!("✅ 清除历史记录测试通过");

    // 浮点数精度测试
    let result = calc.add(0.1, 0.2);
    assert!((result - 0.3).abs() < f64::EPSILON);
    println!("✅ 浮点数精度测试通过");

    println!("📊 TDD示例完成");
}

/// 演示边界条件和错误处理测试
pub fn boundary_and_error_testing() {
    println!("🎯 边界条件和错误处理测试：");

    // 数据验证函数
    fn validate_age(age: i32) -> Result<bool, String> {
        if age < 0 {
            return Err("年龄不能为负数".to_string());
        }
        if age > 150 {
            return Err("年龄超出合理范围".to_string());
        }
        Ok(true)
    }

    fn validate_username(username: &str) -> Result<bool, String> {
        if username.trim().is_empty() {
            return Err("用户名不能为空".to_string());
        }
        if username.len() < 3 {
            return Err("用户名长度至少3个字符".to_string());
        }
        if username.len() > 20 {
            return Err("用户名长度不能超过20个字符".to_string());
        }
        Ok(true)
    }

    fn validate_email(email: &str) -> Result<bool, String> {
        if !email.contains('@') {
            return Err("邮箱必须包含@符号".to_string());
        }
        if !email.contains('.') {
            return Err("邮箱必须包含域名".to_string());
        }
        Ok(true)
    }

    // 边界条件测试
    println!("🔍 年龄验证边界测试:");

    let age_test_cases = vec![
        (-1, "负数年龄"),
        (0, "零岁"),
        (1, "一岁"),
        (18, "成年年龄"),
        (65, "退休年龄"),
        (120, "高龄"),
        (150, "极限年龄"),
        (151, "超出上限"),
    ];

    for (age, desc) in age_test_cases {
        match validate_age(age) {
            Ok(_) => println!("  ✅ {}: 有效", desc),
            Err(e) => println!("  ❌ {}: {}", desc, e),
        }
    }

    // 用户名验证测试
    println!("\n🔍 用户名验证边界测试:");

    let username_test_cases = vec![
        ("", "空字符串"),
        ("  ", "纯空格"),
        ("ab", "太短"),
        ("abc", "最小有效长度"),
        ("user_name", "包含下划线"),
        ("UserName", "包含大写"),
    ];

    for (username, desc) in username_test_cases {
        match validate_username(username) {
            Ok(_) => println!("  ✅ {}: 有效", desc),
            Err(e) => println!("  ❌ {}: {}", desc, e),
        }
    }

    // 邮箱验证测试
    println!("\n🔍 邮箱验证边界测试:");

    let email_test_cases = vec![
        ("", "空字符串"),
        ("@", "只有@符号"),
        ("user@", "缺少域名"),
        ("user@domain", "缺少顶级域名"),
        ("user@domain.com", "有效邮箱"),
        ("user.name@domain.com", "包含点的用户名"),
    ];

    for (email, desc) in email_test_cases {
        match validate_email(email) {
            Ok(_) => println!("  ✅ {}: 有效", desc),
            Err(e) => println!("  ❌ {}: {}", desc, e),
        }
    }

    println!("📊 边界条件测试完成");
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

    match User::new("张三".to_string(), "zhangsan@example.com".to_string(), 25) {
        Ok(user) => {
            manager.add_user(user).expect("Failed to add user");
            println!("✅ 用户添加成功");
        }
        Err(e) => println!("❌ 用户创建失败: {}", e),
    }

    match User::new("李四".to_string(), "invalid-email".to_string(), 25) {
        Ok(user) => {
            manager.add_user(user).expect("Failed to add user");
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

/// 运行所有测试示例
pub fn run_all_testing_examples() {
    println!("🎯 === 全面测试示例 ===");
    println!();

    println!("=== 基础测试示例 ===");
    run_testing_examples();
    println!();

    println!("=== 企业级测试策略 ===");
    enterprise_testing_strategies();
    println!();

    println!("=== 属性测试基础 ===");
    property_based_testing_basics();
    println!();

    println!("=== 性能测试示例 ===");
    performance_testing_examples();
    println!();

    println!("=== 集成测试场景 ===");
    integration_testing_scenarios();
    println!();

    println!("=== TDD示例 ===");
    test_driven_development_example();
    println!();

    println!("=== 边界条件测试 ===");
    boundary_and_error_testing();

    println!("\n✅ 所有测试示例运行完成！");
}
