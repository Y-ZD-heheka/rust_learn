//! # 基础语法模块
//!
//! 这个模块演示了Rust的基础语法概念，包括变量、函数、控制流等。

/// 演示变量声明和基本类型
pub fn variables_and_types() {
    // 变量声明
    let x = 5; // 不可变变量
    let mut y = 10; // 可变变量

    println!("x = {}, y = {}", x, y);

    // 基本类型
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    println!("integer: {}, float: {}, boolean: {}, character: {}",
             integer, float, boolean, character);
}

/// 演示函数定义和调用
pub fn functions() {
    fn add(a: i32, b: i32) -> i32 {
        a + b // 隐式返回，最后一行没有分号
    }

    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}

/// 演示控制流
pub fn control_flow() {
    let number = 6;

    // if 表达式
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    // 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break 可以返回值
        }
    };
    println!("The result is {}", result);

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

/// 运行基础语法示例
pub fn run_basics_examples() {
    println!("=== 基础语法示例 ===");
    variables_and_types();
    functions();
    control_flow();
}