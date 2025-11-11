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
    
    error_handling_patterns();
    println!();
    
    modern_enums_and_patterns();
    
    println!("\n✅ 所有基础示例运行完成！");
}