//! # 类型系统模块
//!
//! 这个模块演示了Rust的类型系统，包括结构体、枚举、特征等。

/// 演示结构体
pub fn structs() {
    // 定义结构体
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 创建结构体实例
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {} <{}>, active: {}, sign-ins: {}",
             user1.username, user1.email, user1.active, user1.sign_in_count);

    // 元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black: ({}, {}, {})", black.0, black.1, black.2);

    // 单元结构体
    struct UnitStruct;
    let _unit = UnitStruct;
}

/// 演示枚举
pub fn enums() {
    // 定义枚举
    enum IpAddrKind {
        V4,
        V6,
    }

    // 使用枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 带数据的枚举
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // Option 枚举
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}, some_string: {:?}, absent_number: {:?}",
             some_number, some_string, absent_number);
}

/// 演示特征
pub fn traits() {
    // 定义特征
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 实现特征
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());
}

/// 演示泛型
pub fn generics() {
    // 泛型函数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// 运行类型系统示例
pub fn run_types_examples() {
    println!("=== 类型系统示例 ===");
    structs();
    enums();
    traits();
    generics();
}