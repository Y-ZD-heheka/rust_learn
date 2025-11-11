//! # 宏和元编程模块
//!
//! 这个模块演示了Rust的宏系统和元编程能力。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::format;

/// 现代化声明宏示例
macro_rules! say_hello {
    () => {
        println!("🎯 你好！");
    };
    ($name:expr) => {
        println!("🎯 你好，{}！", $name);
    };
    ($($name:expr),+ $(,)?) => {
        $(
            println!("🎯 你好，{}！", $name);
        )+
    };
}

/// 现代化字符串处理宏
macro_rules! format_greeting {
    (name: $name:expr, age: $age:expr) => {
        format!("👤 姓名: {}, 年龄: {} 岁", $name, $age)
    };
    (name: $name:expr) => {
        format!("👤 姓名: {}", $name)
    };
    ($($key:ident: $value:expr),+ $(,)?) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&format!("{}: {}, ", stringify!($key), $value));
            )+
            result
        }
    };
}

/// 现代化向量操作宏
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// 现代化数学操作宏
macro_rules! math_operations {
    ($($op:ident: $a:expr, $b:expr);+ $(;)?) => {
        {
            let mut results = Vec::new();
            $(
                let result = match stringify!($op) {
                    "add" => $a + $b,
                    "sub" => $a - $b,
                    "mul" => $a * $b,
                    "div" => $a / $b,
                    _ => 0,
                };
                results.push((stringify!($op), result));
            )+
            results
        }
    };
}

/// 现代化条件宏
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!("🐛 [DEBUG] {}", format!($($arg)*));
        }
    };
    ($($arg:tt)*) => {
        #[cfg(not(debug_assertions))]
        {
            println!($($arg)*);
        }
    };
}

/// 现代化类型安全的宏
macro_rules! create_point {
    (x: $x:expr, y: $y:expr) => {
        Point { x: $x, y: $y }
    };
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

/// 现代化声明宏使用
pub fn modern_declarative_macros() {
    println!("🎯 现代化声明宏：");
    
    // 基本使用
    say_hello!();
    say_hello!("Rust 2024");
    say_hello!("Alice", "Bob", "Charlie");
    
    println!();
    
    // 字符串格式化
    let greeting1 = format_greeting!(name: "张三", age: 25);
    let greeting2 = format_greeting!(name: "李四");
    let greeting3 = format_greeting!(name: "王五", age: 30, city: "北京");
    
    println!("{}", greeting1);
    println!("{}", greeting2);
    println!("{}", greeting3);
    
    println!();
    
    // 调试输出
    debug_println!("当前变量值为: {}", 42);
    debug_println!("复杂计算: {} + {} = {}", 10, 20, 30);
}

/// 现代化自定义宏
pub fn modern_custom_macros() {
    println!("🔧 现代化自定义宏：");
    
    // 现代化向量操作
    let numbers = my_vec![1, 2, 3, 4, 5];
    let strings = my_vec!["hello", "rust", "world"];
    let floats = my_vec![1.0, 2.0, 3.0];
    
    println!("数字向量: {:?}", numbers);
    println!("字符串向量: {:?}", strings);
    println!("浮点数向量: {:?}", floats);
    
    // 数学操作宏
    let ops = math_operations!(
        add: 10, 5;
        sub: 10, 5;
        mul: 10, 5;
        div: 10, 5;
    );
    
    println!("数学运算结果:");
    for (op, result) in ops {
        println!("  {}: {}", op, result);
    }
}

/// 现代化派生宏示例
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 现代化过程宏演示
pub fn modern_procedural_macros_demo() {
    println!("⚙️ 现代化过程宏演示：");
    
    // 演示派生宏
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 4.0, y: 6.0 };
    
    println!("点1: {}", p1);
    println!("点2: {}", p2);
    println!("点1 Debug: {:?}", p1);
    println!("点2 Clone: {:?}", p2.clone());
    println!("相等检查: {}", p1 == p1);
    
    // 使用宏创建点
    let p3 = create_point!(x: 10.0, y: 20.0);
    let p4 = create_point!(5.0, 15.0);
    
    println!("点3: {}", p3);
    println!("点4: {}", p4);
    println!("点3到点4距离: {:.2}", p3.distance(&p4));
}

/// 现代化宏卫生性和作用域
macro_rules! scoped_function {
    ($func_name:ident, $message:expr) => {
        fn $func_name() {
            println!("🔧 {}: {}", stringify!($func_name), $message);
        }
    };
}

macro_rules! data_structure {
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                pub $field: $ty,
            )*
        }
    };
    (enum $name:ident { $( $variant:ident $(($($field:ty),*))? ),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant $(($( $field ),*))?,
            )*
        }
    };
}

/// 现代化宏卫生性演示
pub fn modern_macro_hygiene() {
    println!("🧹 现代化宏卫生性：");
    
    // 作用域内的函数创建
    scoped_function!(greet_user, "用户问候");
    scoped_function!(show_banner, "显示横幅");
    
    greet_user();
    show_banner();
    
    // 数据结构宏
    data_structure!(
        struct User {
            name: String,
            age: u32,
            email: String,
        }
    );
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };
    
    println!("用户信息: {:?}", user);
    
    // 枚举宏
    data_structure!(
        enum Status {
            Active,
            Inactive,
            Pending(String),
        }
    );
    
    let status = Status::Active;
    let pending_status = Status::Pending("等待审核".to_string());
    
    println!("状态: {:?}", status);
    println!("待处理: {:?}", pending_status);
}

/// 现代化重复模式宏
macro_rules! generate_variants {
    (enum $name:ident { $($variant:ident),* $(,)? }) => {
        impl $name {
            $(
                pub fn $variant() -> Self {
                    Self::$variant
                }
            )*
            
            pub fn all_variants() -> &'static [Self] {
                &[
                    $( Self::$variant, )*
                ]
            }
        }
        
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
}

/// 现代化重复模式演示
pub fn modern_repetition_patterns() {
    println!("🔄 现代化重复模式：");
    
    generate_variants!(
        enum Color {
            Red,
            Green,
            Blue,
            Yellow,
        }
    );
    
    // 使用生成的变体
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    let yellow = Color::Yellow;
    
    println!("颜色变体: {:?}", [red, green, blue, yellow]);
    
    // 使用all_variants方法
    let all_colors = Color::all_variants();
    println!("所有颜色:");
    for (i, color) in all_colors.iter().enumerate() {
        println!("  {}: {:?}", i, color);
    }
}

/// 运行宏和元编程示例
pub fn run_macros_examples() {
    println!("🎯 === 现代化宏和元编程示例 ===");
    println!();
    
    modern_declarative_macros();
    println!();
    
    modern_custom_macros();
    println!();
    
    modern_procedural_macros_demo();
    println!();
    
    modern_macro_hygiene();
    println!();
    
    modern_repetition_patterns();
    
    println!("\n✅ 所有宏和元编程示例运行完成！");
}