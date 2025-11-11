//! # 类型系统模块
//!
//! 这个模块演示了Rust的类型系统，包括结构体、枚举、特征等。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fmt::Display;
use std::fmt;

/// 现代化结构体演示
pub fn structs() {
    println!("🏗️ 现代化结构体：");
    
    // 使用现代化的结构体定义
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    // 现代化结构体实现
    impl User {
        fn new(username: String, email: String) -> Self {
            Self {
                username,
                email,
                sign_in_count: 1,
                active: true,
            }
        }
        
        fn activate(&mut self) {
            self.active = true;
        }
        
        fn deactivate(&mut self) {
            self.active = false;
        }
        
        fn get_display_name(&self) -> &str {
            if self.active {
                &self.username
            } else {
                "[停用]"
            }
        }
    }

    // 创建结构体实例
    let mut user1 = User::new(
        String::from("rust_learner"),
        String::from("rust@example.com")
    );
    
    println!("用户信息: {}", user1.get_display_name());
    println!("详情: {:?}", user1);
    
    // 使用更新语法
    let user2 = User {
        username: String::from("another_learner"),
        email: String::from("learn@example.com"),
        ..user1 // 使用其他字段的默认值
    };
    
    println!("用户2: {:?}", user2);

    // 元组结构体的现代化用法
    #[derive(Debug, Clone, Copy)]
    struct Color(u8, u8, u8);
    
    impl Color {
        fn new(r: u8, g: u8, b: u8) -> Self {
            Self(r, g, b)
        }
        
        fn to_hex_string(&self) -> String {
            format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
        }
    }
    
    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);
    println!("黑色: {:?} -> {}", black, black.to_hex_string());
    println!("白色: {:?} -> {}", white, white.to_hex_string());

    // 单元结构体用于特征实现
    #[derive(Debug, Clone)]
    struct Empty;
    
    impl Display for Empty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Empty struct")
        }
    }
    
    let unit = Empty;
    println!("单元结构体: {}", unit);
}

/// 现代化枚举演示
pub fn enums() {
    println!("🎯 现代化枚举：");
    
    // 现代化IP地址枚举
    #[derive(Debug, Clone)]
    enum IpAddr {
        V4 { addr: [u8; 4] },
        V6(String),
    }
    
    impl IpAddr {
        fn get_description(&self) -> String {
            match self {
                Self::V4 { addr } => format!("IPv4: {}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]),
                Self::V6(addr) => format!("IPv6: {}", addr),
            }
        }
        
        fn is_localhost(&self) -> bool {
            match self {
                Self::V4 { addr } => *addr == [127, 0, 0, 1],
                Self::V6(addr) => addr == "::1",
            }
        }
    }
    
    let home = IpAddr::V4 { addr: [192, 168, 1, 1] };
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("本地地址: {}", home.get_description());
    println!("环回地址: {}", loopback.get_description());
    println!("localhost检查: {} | {}", home.is_localhost(), loopback.is_localhost());

    // 现代化Option枚举使用
    let some_number = Some(42);
    let some_string = Some("Hello Rust".to_string());
    let absent_number: Option<i32> = None;
    
    // 使用模式匹配进行安全解包
    match some_number {
        Some(n) if n > 40 => println!("大数: {}", n),
        Some(n) => println!("小数: {}", n),
        None => println!("没有值"),
    }
    
    // 现代化Result类型使用
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }
    
    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
    
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("除法结果: {}", result),
        Err(MathError::DivisionByZero) => println!("除零错误"),
        Err(MathError::NegativeSquareRoot) => println!("负数平方根错误"),
    }
}

/// 现代化特征系统
pub fn traits() {
    println!("🎨 现代化特征系统：");
    
    // 定义高级特征
    trait Summary {
        fn summarize(&self) -> String {
            "No summary available".to_string()
        }
        
        fn summarize_author(&self) -> String {
            "Unknown author".to_string()
        }
        
        fn detailed_summary(&self) -> String {
            format!("{} - by {}", self.summarize(), self.summarize_author())
        }
    }
    
    trait Display: Summary {
        fn display_format(&self) -> String;
    }
    
    // 实现特征
    #[derive(Debug, Clone)]
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("头条: {}, 地点: {}, 作者: {}",
                    self.headline, self.location, self.author)
        }
        
        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }
    
    impl Display for NewsArticle {
        fn display_format(&self) -> String {
            format!("📰 {}", self.detailed_summary())
        }
    }
    
    // 现代特征对象
    trait Drawable {
        fn draw(&self) -> String;
        fn area(&self) -> f64;
    }
    
    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }
    
    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }
    
    impl Drawable for Circle {
        fn draw(&self) -> String {
            format!("🔵 圆形(半径: {:.1})", self.radius)
        }
        
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    impl Drawable for Rectangle {
        fn draw(&self) -> String {
            format!("🟦 矩形({:.1} x {:.1})", self.width, self.height)
        }
        
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }
    
    let article = NewsArticle {
        headline: "Rust 2024新特性发布".to_string(),
        location: "中国".to_string(),
        author: "Rust团队".to_string(),
        content: "Rust编程语言发布了2024版本，包含了众多新特性...".to_string(),
    };
    
    let shapes: Vec<&dyn Drawable> = vec![
        &Circle { radius: 5.0 },
        &Rectangle { width: 4.0, height: 6.0 },
    ];
    
    println!("文章摘要: {}", article.display_format());
    println!("绘图示例:");
    
    for shape in shapes {
        println!("  {} - 面积: {:.2}", shape.draw(), shape.area());
    }
}

/// 现代化泛型系统
pub fn generics() {
    println!("🔧 现代化泛型系统：");
    
    // 现代化泛型函数
    fn largest<T: PartialOrd + Clone + fmt::Display>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        
        for item in list {
            if item > &largest {
                largest = item.clone();
            }
        }
        
        largest
    }
    
    // 现代化泛型结构体
    #[derive(Debug, Clone)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    impl<T: fmt::Display + Copy + PartialOrd> Point<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        
        fn distance_from_origin(&self) -> T {
            // 对于数值类型，这可能会很有用
            if self.x > self.y {
                self.x
            } else {
                self.y
            }
        }
    }
    
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 3.2);
    
    println!("整数点: {:?}", int_point);
    println!("浮点数点: {:?}", float_point);
    
    // 现代化泛型枚举
    #[derive(Debug, Clone)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    // 泛型特征约束示例
    trait Maximum {
        fn get_max(&self) -> &Self;
    }
    
    impl<T: PartialOrd + Clone> Maximum for Vec<T> {
        fn get_max(&self) -> &Self {
            if self.is_empty() {
                return self;
            }
            
            let mut max_index = 0;
            for (i, item) in self.iter().enumerate() {
                if item > &self[max_index] {
                    max_index = i;
                }
            }
            
            self // 返回整个vec而不是切片
        }
    }
    
    let numbers = vec![34, 50, 25, 100, 65];
    if let Some(max) = numbers.get_max().first() {
        println!("最大值: {}", max);
    }
}

/// 运行类型系统示例
pub fn run_types_examples() {
    println!("🎯 === 现代化类型系统示例 ===");
    println!();
    
    structs();
    println!();
    
    enums();
    println!();
    
    traits();
    println!();
    
    generics();
    
    println!("\n✅ 所有类型系统示例运行完成！");
}