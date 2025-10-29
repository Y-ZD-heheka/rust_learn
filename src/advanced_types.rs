//! # 高级类型和生命周期模块
//!
//! 这个模块演示了Rust的高级类型系统和生命周期概念。

/// 演示关联类型
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

/// 实现自定义迭代器
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// 演示关联类型
pub fn associated_types() {
    let mut counter = Counter { count: 0 };
    while let Some(value) = counter.next() {
        println!("Counter: {}", value);
    }
}

/// 演示泛型类型参数
pub fn generic_type_parameters() {
    // 泛型结构体
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);

    // 泛型枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let success: Result<i32, &str> = Result::Ok(42);
    let failure: Result<i32, &str> = Result::Err("Something went wrong");
}

/// 演示生命周期
pub fn lifetimes() {
    // 生命周期注解
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // 结构体中的生命周期
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt { part: first_sentence };
    println!("Excerpt: {}", i.part);
}

/// 演示生命周期省略规则
pub fn lifetime_elision() {
    // 这些函数签名使用了生命周期省略规则
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);
}

/// 演示高级特征
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// 实现Draw特征的结构体
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button: {} ({}x{})", self.label, self.width, self.height);
    }
}

/// 演示特征对象
pub fn trait_objects() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/// 运行高级类型和生命周期示例
pub fn run_advanced_types_examples() {
    println!("=== 高级类型和生命周期示例 ===");
    associated_types();
    generic_type_parameters();
    lifetimes();
    lifetime_elision();
    trait_objects();
}