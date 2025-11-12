//! # 基础语法模块
//!
//! 这个模块演示了Rust的基础语法概念，包括变量、函数、控制流等。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::collections::HashMap;
use std::fmt;

/// 演示变量声明和基本类型
pub fn variables_and_types() {
    println!("🔢 变量声明和基本类型：");
    
    // 使用现代化类型注解和推导
    let x = 5; // 不可变变量
    let mut y = 10; // 可变变量
    y += 5;

    println!("x = {}, y = {}", x, y);

    // 使用更明确的类型声明
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = 'R';
    let string_slice: &str = "Hello, Rust!";
    let string: String = String::from("Hello, World!");
    
    // 使用HashMap展示现代集合类型
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Team A", 10);
    scores.insert("Team B", 20);

    println!("类型演示：整数={}, 浮点数={}, 布尔值={}, 字符={}, 切片={}, 字符串={}",
             integer, float, boolean, character, string_slice, string);
    
    println!("哈希映射示例: {:?}", scores);
}

/// 演示函数定义和现代化调用模式
pub fn functions() {
    println!("🔧 函数定义和调用：");
    
    // 使用泛型和特征约束的现代函数
    fn add<T: fmt::Display + std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    
    // 函数调用和结果处理
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    
    // 演示更复杂的函数类型
    fn apply_operation<F>(x: i32, operation: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        operation(x)
    }
    
    let doubled = apply_operation(5, |x| x * 2);
    let squared = apply_operation(5, |x| x * x);
    
    println!("应用操作：翻倍={}, 平方={}", doubled, squared);
}

/// 演示现代化的控制流
pub fn control_flow() {
    println!("🔄 现代化控制流：");
    
    let number = 42;
    
    // 使用if let进行模式匹配 (使用可用的浮点数方法)
    let cube_root = (number as f64).cbrt();
    if cube_root.fract() < 0.001 { // 近似整数立方根检查
        println!("数字{}的立方根约为: {}", number, cube_root);
    }
    
    // 使用模式匹配的现代化条件判断
    match number {
        n if n % 4 == 0 => println!("{}能被4整除", n),
        n if n % 3 == 0 => println!("{}能被3整除", n),
        n if n % 2 == 0 => println!("{}是偶数", n),
        _ => println!("{}不是特殊数字", number),
    }
    
    // 使用迭代器的现代循环模式
    let fruits = vec!["🍎", "🍊", "🍌", "🍇", "🍓"];
    for (index, fruit) in fruits.iter().enumerate() {
        println!("位置{}的水果: {}", index + 1, fruit);
    }
    
    // 使用现代化的高阶函数模式
    let numbers: Vec<i32> = (1..=10).collect();
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    
    println!("偶数的平方: {:?}", even_squares);
}

/// 演示现代化数据结构实现
pub fn modern_data_structures() {
    println!("🏗️ 现代化数据结构：");
    
    // 现代化整数栈实现
    #[derive(Debug)]
    struct ModernStack {
        items: Vec<i32>,
    }
    
    impl ModernStack {
        fn new() -> Self {
            Self { items: Vec::new() }
        }
        
        fn push(&mut self, item: i32) {
            self.items.push(item);
            println!("📦 压入: {}", item);
        }
        
        fn pop(&mut self) -> Option<i32> {
            self.items.pop()
        }
        
        fn peek(&self) -> Option<&i32> {
            self.items.last()
        }
        
        fn is_empty(&self) -> bool {
            self.items.is_empty()
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut stack = ModernStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈顶元素: {:?}", stack.peek());
    println!("栈大小: {}", stack.len());
    
    while let Some(item) = stack.pop() {
        println!("弹出: {}", item);
    }
    
    // 现代化字符串队列实现
    #[derive(Debug)]
    struct ModernQueue {
        items: Vec<String>,
        index: usize,
    }
    
    impl ModernQueue {
        fn new() -> Self {
            Self { items: Vec::new(), index: 0 }
        }
        
        fn enqueue(&mut self, item: &str) {
            self.items.push(item.to_string());
            println!("➕ 入队: {}", item);
        }
        
        fn dequeue(&mut self) -> Option<String> {
            if self.index < self.items.len() {
                let item = Some(self.items[self.index].clone());
                self.index += 1;
                
                // 清理已出队的元素
                if self.index * 2 > self.items.len() {
                    self.items = self.items[self.index..].to_vec();
                    self.index = 0;
                }
                
                item
            } else {
                None
            }
        }
        
        fn is_empty(&self) -> bool {
            self.index >= self.items.len()
        }
    }
    
    let mut queue = ModernQueue::new();
    queue.enqueue("任务1");
    queue.enqueue("任务2");
    queue.enqueue("任务3");
    
    while let Some(item) = queue.dequeue() {
        println!("处理: {}", item);
    }
}

/// 演示高级算法实现
pub fn advanced_algorithms() {
    println!("🔬 高级算法实现：");
    
    // 现代化排序算法 - 适用可比较类型
    fn quick_sort<T: PartialOrd + std::fmt::Display + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        
        let pivot = arr.len() / 2;
        let pivot_value = arr[pivot].clone();
        
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for (i, item) in arr.iter().enumerate() {
            if i == pivot {
                continue;
            }
            
            if item < &pivot_value {
                left.push(item.clone());
            } else {
                right.push(item.clone());
            }
        }
        
        // 递归排序
        quick_sort(&mut left);
        quick_sort(&mut right);
        
        // 合并结果
        let mut result = left;
        result.push(pivot_value);
        result.extend(right);
        
        // 复制回原数组
        for (i, item) in result.iter().enumerate() {
            arr[i] = item.clone();
        }
    }
    
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("排序前: {:?}", numbers);
    quick_sort(&mut numbers);
    println!("排序后: {:?}", numbers);
    
    // 二分查找算法
    fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = arr.len();
        
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] == target {
                return Some(mid);
            } else if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        None
    }
    
    match binary_search(&numbers, 25) {
        Some(index) => println!("找到目标25在索引: {}", index),
        None => println!("未找到目标"),
    }
    
    // 现代化斐波那契数列（记忆化）
    fn fibonacci_memo(n: usize, memo: &mut Vec<u64>) -> u64 {
        if n <= 1 {
            return n as u64;
        }
        
        if memo[n] != 0 {
            return memo[n];
        }
        
        memo[n] = fibonacci_memo(n - 1, memo) + fibonacci_memo(n - 2, memo);
        memo[n]
    }
    
    let n = 10;
    let mut memo = vec![0; n + 1];
    let result = fibonacci_memo(n, &mut memo);
    println!("斐波那契数列第{}项: {}", n, result);
}

/// 演示闭包和高阶函数
pub fn closures_and_higher_order_functions() {
    println!("🎯 闭包和高阶函数：");
    
    // 现代化闭包使用
    let add = |a: i32, b: i32| -> i32 { a + b };
    let multiply = |x: i32| { x * 2 };
    let greet = |name: &str| format!("你好, {}", name);
    
    println!("加法: {}", add(5, 3));
    println!("乘法: {}", multiply(7));
    println!("问候: {}", greet("世界"));
    
    // 高阶函数示例
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 函数式编程操作链
    let result: Vec<String> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // 过滤偶数
        .map(|x| x * x) // 平方
        .enumerate() // 添加索引
        .map(|(i, x)| format!("位置{}: {}", i + 1, x)) // 格式化
        .collect();
    
    println!("处理结果: {:?}", result);
    
    // 现代化回调函数
    fn process_data<F>(data: Vec<i32>, processor: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32,
    {
        data.into_iter().map(processor).collect()
    }
    
    let original = vec![1, 2, 3, 4, 5];
    let processed = process_data(original, |x| {
        if x % 2 == 0 {
            x * 2
        } else {
            x * 3
        }
    });
    
    println!("处理后的数据: {:?}", processed);
    
    // 现代化状态闭包
    fn create_counter(start: i32) -> impl FnMut() -> i32 {
        let mut current = start;
        move || {
            current += 1;
            current - 1
        }
    }
    
    let mut counter = create_counter(0);
    println!("计数器: {}", counter());
    println!("计数器: {}", counter());
    println!("计数器: {}", counter());
}

/// 演示文件操作和IO
pub fn file_operations() {
    println!("📁 文件操作和IO：");
    
    use std::fs;
    // 创建测试文件
    let test_file = "test_data.txt";
    let content = "Hello, Rust!\n这是一个测试文件\n包含多行数据";
    
    match fs::write(test_file, content) {
        Ok(_) => println!("✅ 创建测试文件成功"),
        Err(e) => println!("❌ 创建文件失败: {}", e),
    }
    
    // 读取文件内容
    match fs::read_to_string(test_file) {
        Ok(contents) => {
            println!("📖 文件内容:");
            for (i, line) in contents.lines().enumerate() {
                println!("  {}: {}", i + 1, line);
            }
        }
        Err(e) => println!("❌ 读取文件失败: {}", e),
    }
    
    // 检查文件是否存在
    if fs::metadata(test_file).is_ok() {
        println!("📊 文件信息:");
        let metadata = fs::metadata(test_file).unwrap();
        println!("  大小: {} 字节", metadata.len());
        println!("  权限: {:?}", metadata.permissions());
    }
    
    // 清理测试文件
    if let Err(e) = fs::remove_file(test_file) {
        println!("⚠️ 清理文件失败: {}", e);
    }
    
    // 演示目录操作
    if let Ok(entries) = fs::read_dir(".") {
        println!("📂 当前目录内容:");
        for entry in entries.take(5) { // 只显示前5个
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    println!("  📄 {}", name);
                }
            }
        }
    }
}

/// 演示错误处理的现代化模式
pub fn error_handling_patterns() {
    println!("⚡ 现代化错误处理：");
    
    // 使用Result类型进行错误处理
    fn divide_with_result(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    // 使用?操作符简化错误传播
    fn complex_calculation(x: f64) -> Result<f64, String> {
        let squared = divide_with_result(x, 2.0)?; // 错误会立即返回
        let cubed = divide_with_result(squared * x, 3.0)?;
        Ok(cubed)
    }
    
    // 处理结果
    match complex_calculation(12.0) {
        Ok(result) => println!("复杂计算结果: {}", result),
        Err(e) => println!("计算错误: {}", e),
    }
    
    // 现代化错误处理链
    fn parse_number(s: &str) -> Result<i32, String> {
        s.trim().parse()
            .map_err(|e| format!("数字解析失败: {}", e))
    }
    
    let test_cases = vec!["42", "abc", ""];
    
    for case in test_cases {
        match parse_number(case) {
            Ok(num) => println!("✅ '{}' -> {}", case, num),
            Err(e) => println!("❌ '{}' -> 错误: {}", case, e),
        }
    }
}

/// 演示现代枚举和模式匹配
pub fn modern_enums_and_patterns() {
    println!("🎯 现代枚举和模式匹配：");
    
    // 使用更丰富的枚举类型
    #[derive(Debug, Clone)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { a: f64, b: f64, c: f64 },
    }
    
    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
                Shape::Rectangle { width, height } => width * height,
                Shape::Triangle { a, b, c } => {
                    // 使用海伦公式计算三角形面积
                    let s = (a + b + c) / 2.0;
                    (s * (s - a) * (s - b) * (s - c)).sqrt()
                },
            }
        }
    }
    
    // 创建各种形状并计算面积
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
    ];
    
    for (i, shape) in shapes.iter().enumerate() {
        println!("形状{}的面积: {:.2}", i + 1, shape.area());
    }
    
    // 复杂的状态机枚举
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    impl TrafficLight {
        fn next(&self) -> Self {
            match self {
                TrafficLight::Red => TrafficLight::Green,
                TrafficLight::Green => TrafficLight::Yellow,
                TrafficLight::Yellow => TrafficLight::Red,
            }
        }
        
        fn get_duration(&self) -> u8 {
            match self {
                TrafficLight::Red => 30,
                TrafficLight::Yellow => 5,
                TrafficLight::Green => 25,
            }
        }
    }
    
    let mut light = TrafficLight::Red;
    for i in 0..4 {
        println!("第{}阶段: {:?} (持续{}秒)", i + 1, light, light.get_duration());
        light = light.next();
    }
}

/// 运行基础语法示例
pub fn run_basics_examples() {
    println!("🎯 === 现代化基础语法示例 ===");
    println!();
    
    variables_and_types();
    println!();
    
    functions();
    println!();
    
    control_flow();
    println!();
    
    modern_data_structures();
    println!();
    
    advanced_algorithms();
    println!();
    
    closures_and_higher_order_functions();
    println!();
    
    file_operations();
    println!();
    
    error_handling_patterns();
    println!();
    
    modern_enums_and_patterns();
    
    println!("\n✅ 所有基础示例运行完成！");
}