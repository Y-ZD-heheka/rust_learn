# 类型系统模块 (types)

## 📖 模块概述

Rust 的类型系统是其核心特性之一，提供了强大的类型安全和零成本抽象能力。本模块深入讲解结构体、枚举、特征、泛型等类型系统概念。

## 🎯 学习目标

- 掌握结构体的定义和使用
- 理解枚举和模式匹配
- 学会特征的定义和实现
- 掌握泛型编程
- 理解高级类型模式

## 📚 内容目录

### 1. 结构体 (`structs`)

```rust
// 基本结构体
#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 构造方法
impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}

// 元组结构体
struct Color(u8, u8, u8);
let black = Color(0, 0, 0);

// 单元结构体
struct Empty;
```

### 2. 枚举 (`enums`)

```rust
// 带数据的枚举
#[derive(Debug, Clone)]
enum IpAddr {
    V4 { addr: [u8; 4] },
    V6(String),
}

// Option 和 Result
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 状态机枚举
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn next(&self) -> Self {
        match self {
            Self::Red => Self::Green,
            Self::Green => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }
}
```

### 3. 特征 (`traits`)

```rust
// 定义特征
trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn detailed_summary(&self) -> String {
        format!("摘要: {}", self.summarize())
    }
}

// 实现特征
struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.author)
    }
}

// 特征对象
fn notify(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

### 4. 泛型 (`generics`)

```rust
// 泛型函数
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if item > &largest {
            largest = item.clone();
        }
    }
    largest
}

// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 多类型参数
struct Pair<K, V> {
    key: K,
    value: V,
}
```

### 5. 关联类型 (`associated_types`)

```rust
// 特征中的关联类型
trait Graph {
    type Node;
    type Edge;
    
    fn add_node(&mut self, node: Self::Node);
    fn add_edge(&mut self, from: Self::Node, to: Self::Node, edge: Self::Edge);
}

// 实现
struct SimpleGraph {
    nodes: Vec<City>,
    edges: Vec<(usize, usize, Route)>,
}

impl Graph for SimpleGraph {
    type Node = City;
    type Edge = Route;
    
    fn add_node(&mut self, node: City) {
        self.nodes.push(node);
    }
    
    fn add_edge(&mut self, from: City, to: City, edge: Route) {
        // ...
    }
}
```

### 6. NewType 模式 (`newtype_pattern`)

```rust
// 类型安全的包装
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SessionId(String);

impl UserId {
    fn new(id: u64) -> Self {
        Self(id)
    }
    
    fn as_u64(&self) -> u64 {
        self.0
    }
}

// 防止类型混淆
fn process_user(user_id: UserId, session_id: SessionId) {
    // 编译器会确保类型正确
}
```

### 7. 类型级编程 (`type_level_programming`)

```rust
// 类型状态模式
struct Enabled;
struct Disabled;

struct LightSwitch<State> {
    is_on: bool,
    _state: PhantomData<State>,
}

impl LightSwitch<Disabled> {
    fn new() -> Self {
        Self { is_on: false, _state: PhantomData }
    }
    
    fn enable(self) -> LightSwitch<Enabled> {
        LightSwitch { is_on: true, _state: PhantomData }
    }
}

impl LightSwitch<Enabled> {
    fn is_enabled(&self) -> bool {
        self.is_on
    }
}

// 类型级数值
struct One;
struct Two;
struct Three;

trait TypeAdd<Rhs = Self> {
    type Output;
}
```

### 8. 组合模式 (`composition_patterns`)

```rust
// 组件特征
trait Component {
    fn operation(&self) -> String;
}

// 叶节点
struct Leaf {
    name: String,
}

impl Component for Leaf {
    fn operation(&self) -> String {
        format!("叶子: {}", self.name)
    }
}

// 组合节点
struct Composite {
    children: Vec<Box<dyn Component>>,
}

impl Component for Composite {
    fn operation(&self) -> String {
        self.children.iter()
            .map(|c| c.operation())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
```

## 🚀 运行示例

```bash
# 运行类型系统模块
cargo run types

# 运行高级类型示例
cargo run advanced_types
```

## 📊 类型系统图解

```
┌─────────────────────────────────────────────────────────────┐
│                     Rust 类型系统                            │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │  标量类型    │    │  复合类型    │    │  自定义类型  │     │
│  ├─────────────┤    ├─────────────┤    ├─────────────┤     │
│  │ • i8-i128   │    │ • 元组      │    │ • struct    │     │
│  │ • u8-u128   │    │ • 数组      │    │ • enum      │     │
│  │ • f32, f64  │    │ • 切片      │    │ • trait     │     │
│  │ • bool      │    │ • String    │    │ • type alias│     │
│  │ • char      │    │ • Vec<T>    │    │ • newtype   │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │  引用类型    │    │  智能指针    │    │  泛型类型    │     │
│  ├─────────────┤    ├─────────────┤    ├─────────────┤     │
│  │ • &T        │    │ • Box<T>    │    │ • Vec<T>    │     │
│  │ • &mut T    │    │ • Rc<T>     │    │ • Option<T> │     │
│  │ • &'a T     │    │ • Arc<T>    │    │ • Result<T> │     │
│  │             │    │ • RefCell<T>│    │ • Cow<T>    │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 定义一个 `Rectangle` 结构体，包含宽度和高度，并实现计算面积的方法
2. 定义一个 `Color` 枚举，包含红、绿、蓝三种颜色
3. 为 `Rectangle` 实现 `Display` 特征

### 中级
1. 实现一个泛型函数，找出切片中的最大值和最小值
2. 定义一个 `Iterator` 特征的自定义实现
3. 使用 NewType 模式创建类型安全的 `Meters` 和 `Kilometers` 类型

### 高级
1. 实现一个类型状态模式的构建器
2. 使用关联类型实现一个容器特征
3. 实现一个类型级的布尔代数系统

## 🔗 相关资源

- [Rust 类型系统](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [枚举和模式匹配](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [特征](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [高级类型](https://doc.rust-lang.org/book/ch19-04-advanced-types.html)

## ⚠️ 常见陷阱

### 1. 特征对象大小未知
```rust
// ❌ 错误：特征对象大小未知
fn returns_trait() -> dyn Summary {
    NewsArticle { ... }
}

// ✅ 正确：使用 Box
fn returns_trait() -> Box<dyn Summary> {
    Box::new(NewsArticle { ... })
}
```

### 2. 孤儿规则
```rust
// ❌ 错误：不能为外部类型实现外部特征
impl Display for Vec<i32> { }

// ✅ 正确：使用 NewType 模式
struct MyVec(Vec<i32>);
impl Display for MyVec { }
```

### 3. 生命周期省略
```rust
// ❌ 可能需要显式生命周期
struct Parser<'a> {
    text: &'a str,  // 必须标注生命周期
}
```

## 📊 学习检查清单

- [ ] 掌握结构体的定义和方法
- [ ] 理解枚举和模式匹配
- [ ] 会定义和实现特征
- [ ] 掌握泛型编程
- [ ] 理解特征边界
- [ ] 会使用特征对象
- [ ] 理解关联类型
- [ ] 掌握 NewType 模式
- [ ] 理解类型状态模式
