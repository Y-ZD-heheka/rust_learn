//! # 所有权系统模块
//!
//! 这个模块演示了Rust的所有权系统，包括所有权、借用和生命周期的概念。

/// 演示所有权的基本概念
pub fn ownership_basics() {
    // 字符串字面量是不可变的，存储在栈上
    let s1 = "hello"; // &str 类型
    println!("s1: {}", s1);

    // String 类型存储在堆上，可变
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("s2: {}", s2);

    // 所有权转移
    let s3 = s2; // s2 的所有权转移给 s3，s2 不再有效
    // println!("s2: {}", s2); // 这行会编译错误
    println!("s3: {}", s3);
}

/// 演示借用
pub fn borrowing() {
    let s1 = String::from("hello");

    // 不可变借用
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变借用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed string: {}", s2);

    // 借用规则：任意时刻，只能有一个可变引用，或多个不可变引用
    let r1 = &s2; // 不可变引用
    let r2 = &s2; // 另一个不可变引用
    // let r3 = &mut s2; // 这会编译错误，因为已有不可变引用
    println!("r1: {}, r2: {}", r1, r2);
}

/// 计算字符串长度（不可变借用）
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 修改字符串（可变借用）
fn change(s: &mut String) {
    s.push_str(", world");
}

/// 演示切片
pub fn slices() {
    let s = String::from("hello world");

    // 字符串切片
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}, world: {}", hello, world);

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}

/// 运行所有权系统示例
pub fn run_ownership_examples() {
    println!("=== 所有权系统示例 ===");
    ownership_basics();
    borrowing();
    slices();
}