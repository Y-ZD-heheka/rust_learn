//! # 错误处理模块
//!
//! 这个模块演示了Rust的错误处理机制，包括可恢复错误和不可恢复错误。

use std::fs::File;
use std::io::{self, Read};

/// 演示panic!宏（不可恢复错误）
pub fn panics() {
    // panic!("crash and burn"); // 这会使程序崩溃

    // 数组越界访问会导致panic
    let v = vec![1, 2, 3];
    // v[99]; // 这会panic
    println!("Vector: {:?}", v);
}

/// 演示Result类型（可恢复错误）
pub fn results() {
    // Result<T, E> 枚举
    fn divide(x: f64, y: f64) -> Result<f64, String> {
        if y == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(x / y)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

/// 演示?操作符
pub fn question_mark() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }
}

/// 演示自定义错误类型
pub fn custom_errors() {
    #[derive(Debug)]
    enum CustomError {
        Io(std::io::Error),
        Parse(std::num::ParseIntError),
        Other(String),
    }

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                CustomError::Io(err) => write!(f, "IO Error: {}", err),
                CustomError::Parse(err) => write!(f, "Parse Error: {}", err),
                CustomError::Other(msg) => write!(f, "Other Error: {}", msg),
            }
        }
    }

    impl std::error::Error for CustomError {}

    impl From<std::io::Error> for CustomError {
        fn from(err: std::io::Error) -> Self {
            CustomError::Io(err)
        }
    }

    impl From<std::num::ParseIntError> for CustomError {
        fn from(err: std::num::ParseIntError) -> Self {
            CustomError::Parse(err)
        }
    }

    fn parse_number(s: &str) -> Result<i32, CustomError> {
        let num: i32 = s.parse()?;
        Ok(num)
    }

    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("not_a_number") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// 运行错误处理示例
pub fn run_error_handling_examples() {
    println!("=== 错误处理示例 ===");
    panics();
    results();
    question_mark();
    custom_errors();
}