//! # 宏和元编程模块
//!
//! 这个模块演示了Rust的宏系统和元编程能力。

/// 声明宏
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

/// 演示声明宏的使用
pub fn declarative_macros() {
    say_hello!();
    say_hello!("Rust");
}

/// 自定义vec!宏
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// 演示自定义宏
pub fn custom_macros() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("Custom vec: {:?}", v);
}

/// 过程宏示例（这里只是演示语法，实际过程宏需要单独的crate）
pub fn procedural_macros_demo() {
    // 派生宏示例
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    println!("Point: {:?}", p);

    // 属性宏示例（这里只是语法演示）
    // #[my_attribute]
    // fn my_function() {}

    // 函数式宏示例（这里只是语法演示）
    // my_macro!(input);
}

/// 宏的卫生性
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {} called", stringify!($func_name));
        }
    };
}

/// 演示宏卫生性
pub fn macro_hygiene() {
    create_function!(foo);
    create_function!(bar);

    foo();
    bar();
}

/// 重复模式
macro_rules! repeat_pattern {
    ( $x:expr ; $count:expr ) => {
        {
            let mut result = Vec::new();
            for _ in 0..$count {
                result.push($x);
            }
            result
        }
    };
}

/// 演示重复模式
pub fn repetition() {
    let v = repeat_pattern!(42; 5);
    println!("Repeated: {:?}", v);
}

/// 运行宏和元编程示例
pub fn run_macros_examples() {
    println!("=== 宏和元编程示例 ===");
    declarative_macros();
    custom_macros();
    procedural_macros_demo();
    macro_hygiene();
    repetition();
}