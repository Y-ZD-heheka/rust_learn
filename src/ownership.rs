//! # 所有权系统模块
//!
//! 这个模块演示了Rust的所有权系统，包括所有权、借用和生命周期的概念。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fmt;

/// 演示现代所有权基本概念
pub fn ownership_basics() {
    println!("🏠 所有权基础概念：");
    
    // 字符串字面量是不可变的，存储在栈上
    let s1: &str = "hello";
    println!("字符串切片 s1: {}", s1);

    // String 类型存储在堆上，可变
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("可变字符串 s2: {}", s2);

    // 所有权转移
    let s3 = s2; // s2 的所有权转移给 s3，s2 不再有效
    // println!("s2: {}", s2); // 这行会编译错误
    println!("接收所有权 s3: {}", s3);

    // 现代移动语义示例
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1; // 所有权移动
    // println!("vec1: {:?}", vec1); // 这会编译错误
    println!("vec2: {:?}", vec2);
}

/// 演示现代化借用和引用
pub fn borrowing() {
    println!("🔗 现代化借用和引用：");
    
    let s1 = String::from("hello rust");

    // 不可变借用 - 现代化函数签名
    let len = calculate_length(&s1);
    println!("字符串'{}'的长度: {}", s1, len);

    // 可变借用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后的字符串: {}", s2);

    // 现代借用规则演示 - 多个不可变引用
    let r1 = &s2;
    let r2 = &s2;
    let r3 = &s2; // Rust 2021允许更多不可变引用
    println!("多个不可变引用: {}, {}, {}", r1, r2, r3);

    // 借用检查的作用域演示
    {
        let r4 = &s2; // 新的不可变引用作用域
        println!("作用域内的引用: {}", r4);
    } // r4 在这里被丢弃

    // 现在可以创建可变引用了
    let mut s3 = String::from("mutable");
    append_text(&mut s3);
    println!("可变借用后: {}", s3);
}

/// 使用现代化生命周期语法计算字符串长度
fn calculate_length(s: &str) -> usize {
    s.len()
}

/// 现代化可变借用函数
fn change(s: &mut String) {
    s.push_str(", world");
}

/// 追加文本的现代函数
fn append_text(s: &mut String) {
    s.push_str(" - modified");
}

/// 演示现代化切片操作
pub fn slices() {
    println!("✂️ 现代化切片操作：");
    
    let s = String::from("hello world rust programming");
    
    // 字符串切片的现代方法
    let words: Vec<&str> = s.split_whitespace().collect();
    let first_word = words.get(0).copied().unwrap_or("");
    let last_word = words.last().copied().unwrap_or("");
    
    println!("原字符串: {}", s);
    println!("第一个词: '{}', 最后一个词: '{}'", first_word, last_word);
    
    // 使用切片索引
    let hello = &s[..5];
    let world = &s[6..11];
    let rust = &s[12..];
    
    println!("切片演示: '{}', '{}', '{}'", hello, world, rust);
    
    // 数组切片的现代化操作
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let middle_slice = &arr[3..7];
    let even_numbers: Vec<_> = arr
        .iter()
        .enumerate()
        .filter(|(i, &x)| x % 2 == 0)
        .map(|(i, &x)| (i, x))
        .collect();
    
    println!("中间切片: {:?}", middle_slice);
    println!("偶数及其索引: {:?}", even_numbers);
}

/// 演示高级生命周期和泛型
pub fn advanced_lifetimes() {
    println!("⏰ 高级生命周期和泛型：");
    
    // 现代化泛型函数，使用trait bound
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        announcement: T,
    ) -> &'a str
    where
        T: fmt::Display,
    {
        println!("公告: {}", announcement);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        &string1,
        string2,
        "最长字符串是..."
    );
    
    println!("结果是: {}", result);
}

/// 演示现代Box和智能指针
pub fn modern_pointers() {
    println!("🎯 现代智能指针：");
    
    // 使用Box进行堆分配
    let boxed_value = Box::new(42);
    println!("Box中的值: {}", boxed_value);
    
    // Box的所有权转移
    let transferred_box = boxed_value; // 移动所有权
    // println!("boxed_value: {}", boxed_value); // 编译错误
    println!("转移后的Box值: {}", transferred_box);
    
    // 现代递归结构
    #[derive(Debug)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil)))
        ))
    );
    
    println!("递归列表: {:?}", list);
}

/// 运行所有权系统示例
pub fn run_ownership_examples() {
    println!("🎯 === 现代化所有权系统示例 ===");
    println!();
    
    ownership_basics();
    println!();
    
    borrowing();
    println!();
    
    slices();
    println!();
    
    advanced_lifetimes();
    println!();
    
    modern_pointers();
    
    println!("\n✅ 所有所有权示例运行完成！");
}