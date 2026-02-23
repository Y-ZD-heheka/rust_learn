# 宏和元编程模块 (macros)

## 📖 模块概述

Rust 的宏系统是其强大的元编程工具，允许在编译时生成代码。本模块讲解声明宏和过程宏的使用方法，以及如何构建领域特定语言（DSL）。

## 🎯 学习目标

- 理解宏与函数的区别
- 掌握声明宏的定义和使用
- 了解过程宏的类型和用途
- 学会构建简单的 DSL
- 掌握宏的调试技巧

## 📚 内容目录

### 1. 声明宏基础 (`modern_declarative_macros`)

```rust
// 基本宏定义
macro_rules! say_hello {
    () => {
        println!("你好！");
    };
    ($name:expr) => {
        println!("你好，{}！", $name);
    };
    ($($name:expr),+) => {
        $(
            println!("你好，{}！", $name);
        )+
    };
}

// 使用
say_hello!();
say_hello!("Rust");
say_hello!("Alice", "Bob", "Charlie");
```

### 2. 自定义宏 (`modern_custom_macros`)

```rust
// 向量创建宏
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

let numbers = my_vec![1, 2, 3, 4, 5];

// 数学操作宏
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
```

### 3. 派生宏 (`modern_procedural_macros_demo`)

```rust
// 使用内置派生宏
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// 自定义派生宏（需要 proc-macro crate）
// #[derive(CustomTrait)]
// struct MyStruct { ... }
```

### 4. 宏卫生性 (`modern_macro_hygiene`)

```rust
// 函数生成宏
macro_rules! scoped_function {
    ($func_name:ident, $message:expr) => {
        fn $func_name() {
            println!("{}: {}", stringify!($func_name), $message);
        }
    };
}

scoped_function!(greet, "欢迎");
greet();  // 调用生成的函数

// 数据结构生成宏
macro_rules! data_structure {
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                pub $field: $ty,
            )*
        }
    };
}

data_structure!(
    struct User {
        name: String,
        age: u32,
    }
);
```

### 5. 重复模式 (`modern_repetition_patterns`)

```rust
// 生成枚举和方法
macro_rules! generate_variants {
    (enum $name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant,
            )*
        }
        
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
    };
}

generate_variants!(
    enum Color {
        Red,
        Green,
        Blue,
    }
);
```

### 6. HTML 构建 DSL (`html_builder_dsl`)

```rust
// HTML 元素结构
pub struct HtmlElement {
    tag: String,
    content: Vec<HtmlContent>,
    attributes: HashMap<String, String>,
}

// HTML 宏
macro_rules! html {
    ($tag:ident { $($attr:ident: $val:expr),* } $($child:tt)*) => {
        HtmlElement::new(stringify!($tag))
            $(.attr(stringify!($attr), $val))*
            // ... 处理子元素
    };
}

// 使用
let page = html! {
    html {
        head {
            title { "我的页面" }
        }
        body {
            h1 { "欢迎" }
            p { "这是一个段落" }
        }
    }
};
```

### 7. 配置管理 DSL (`configuration_dsl`)

```rust
// 配置宏
macro_rules! create_config {
    (development) => {{
        Config {
            database: DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
        }
    }};
    (production) => {{
        Config {
            database: DatabaseConfig {
                host: "prod-db.example.com".to_string(),
                port: 5432,
            },
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
        }
    }};
}

let dev_config = create_config!(development);
let prod_config = create_config!(production);
```

### 8. API 路由 DSL (`api_routing_dsl`)

```rust
// 路由定义宏
macro_rules! routes {
    ($($method:ident $path:expr => $handler:expr),* $(,)?) => {
        vec![
            $(
                Route::new(Method::$method, $path, $handler),
            )*
        ]
    };
}

let api_routes = routes![
    GET "/api/users" => get_users,
    POST "/api/users" => create_user,
    GET "/api/users/:id" => get_user_by_id,
    DELETE "/api/users/:id" => delete_user,
];
```

## 🚀 运行示例

```bash
# 运行宏模块
cargo run macros

# 展开宏（查看生成的代码）
cargo expand
```

## 📊 宏类型对比

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust 宏类型                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   声明宏 (macro_rules!)              │   │
│  │                                                     │   │
│  │  • 模式匹配                                         │   │
│  │  • 代码生成                                         │   │
│  │  • 编译时展开                                       │   │
│  │  • 示例: vec!, println!, format!                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   过程宏 (Procedural)                │   │
│  │                                                     │   │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐   │   │
│  │  │ 派生宏      │ │ 属性宏      │ │ 函数宏      │   │   │
│  │  │ #[derive]  │ │ #[attr]     │ │ func!()     │   │   │
│  │  └─────────────┘ └─────────────┘ └─────────────┘   │   │
│  │                                                     │   │
│  │  • 操作 AST                                         │   │
│  │  • 更强大的代码生成                                 │   │
│  │  • 示例: serde, thiserror, async-trait            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 编写一个宏 `double!`，接受一个数字并返回其两倍
2. 编写一个宏 `print_type!`，打印变量的类型名
3. 编写一个宏 `hashmap!`，创建 HashMap

### 中级
1. 编写一个宏生成 getter 和 setter 方法
2. 实现一个简单的测试框架宏
3. 编写一个 DSL 用于定义状态机

### 高级
1. 实现一个完整的 HTML 模板宏
2. 编写一个过程宏实现自动序列化
3. 实现一个编译时计算宏

## 🔗 相关资源

- [Rust 宏小书](https://veykril.github.io/tlborm/)
- [Rust 过程宏](https://doc.rust-lang.org/reference/procedural-macros.html)
- [syn 和 quote 库](https://docs.rs/syn)

## ⚠️ 常见陷阱

### 1. 变量捕获问题
```rust
// ❌ 可能的问题
macro_rules! bad {
    ($x:expr) => {
        let x = $x;  // 如果 $x 包含变量名，可能冲突
        println!("{}", x);
    };
}

// ✅ 使用卫生宏
macro_rules! good {
    ($x:expr) => {
        println!("{}", $x);
    };
}
```

### 2. 重复模式错误
```rust
// ❌ 错误的分号位置
macro_rules! wrong {
    ($($x:expr);*) => { ... };  // 分号分隔
}

// ✅ 正确的语法
macro_rules! right {
    ($($x:expr),*) => { ... };  // 逗号分隔
}
```

### 3. 类型推断问题
```rust
// ❌ 可能导致类型推断失败
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b  // 类型可能不明确
    };
}

// ✅ 明确类型
macro_rules! add_i32 {
    ($a:expr, $b:expr) => {
        ($a as i32) + ($b as i32)
    };
}
```

## 📊 学习检查清单

- [ ] 理解宏与函数的区别
- [ ] 掌握基本宏语法
- [ ] 会使用模式匹配
- [ ] 理解重复模式
- [ ] 会使用 stringify! 和 concat!
- [ ] 理解宏卫生性
- [ ] 了解过程宏
- [ ] 能够构建简单 DSL
