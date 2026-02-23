# 基础语法模块 (basics)

## 📖 模块概述

本模块涵盖了 Rust 编程语言的基础语法和核心概念，是学习 Rust 的起点。通过丰富的示例代码，帮助学习者快速掌握 Rust 的基本编程技能。

## 🎯 学习目标

- 理解 Rust 的变量声明和基本类型
- 掌握函数定义和调用方式
- 熟悉控制流语句（if、match、循环）
- 了解数据结构的基本实现
- 学习闭包和高阶函数的使用
- 掌握文件操作和 IO 处理

## 📚 内容目录

### 1. 变量和类型 (`variables_and_types`)

```rust
// 不可变变量
let x = 5;

// 可变变量
let mut y = 10;
y += 5;

// 基本类型
let integer: i32 = 42;
let float: f64 = 3.14159;
let boolean: bool = true;
let character: char = 'R';
let string_slice: &str = "Hello, Rust!";
let string: String = String::from("Hello, World!");
```

### 2. 函数定义 (`functions`)

```rust
// 泛型函数
fn add<T: Display + Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 高阶函数
fn apply_operation<F>(x: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(x)
}

// 使用示例
let doubled = apply_operation(5, |x| x * 2);
```

### 3. 控制流 (`control_flow`)

```rust
// match 模式匹配
match number {
    n if n % 4 == 0 => println!("能被4整除"),
    n if n % 3 == 0 => println!("能被3整除"),
    _ => println!("其他数字"),
}

// 迭代器循环
let fruits = vec!["🍎", "🍊", "🍌"];
for (index, fruit) in fruits.iter().enumerate() {
    println!("位置{}: {}", index, fruit);
}

// 函数式操作链
let even_squares: Vec<i32> = (1..=10)
    .filter(|&x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

### 4. 数据结构 (`modern_data_structures`)

```rust
// 栈实现
struct ModernStack {
    items: Vec<i32>,
}

impl ModernStack {
    fn new() -> Self { Self { items: Vec::new() } }
    fn push(&mut self, item: i32) { self.items.push(item); }
    fn pop(&mut self) -> Option<i32> { self.items.pop() }
}

// 队列实现
struct ModernQueue {
    items: Vec<String>,
    index: usize,
}
```

### 5. 高级算法 (`advanced_algorithms`)

包含以下算法实现：
- **快速排序** - 原地排序，时间复杂度 O(n log n)
- **二分查找** - 有序数组查找，时间复杂度 O(log n)
- **斐波那契数列** - 矩阵快速幂实现
- **背包问题** - 动态规划解法
- **Dijkstra 最短路径** - 图算法

### 6. 闭包和高阶函数 (`closures_and_higher_order_functions`)

```rust
// 闭包定义
let add = |a: i32, b: i32| a + b;
let multiply = |x: i32| x * 2;

// 状态闭包
fn create_counter(start: i32) -> impl FnMut() -> i32 {
    let mut current = start;
    move || {
        current += 1;
        current - 1
    }
}
```

### 7. 文件操作 (`file_operations`)

```rust
use std::fs;

// 写入文件
fs::write("test.txt", "Hello, Rust!")?;

// 读取文件
let contents = fs::read_to_string("test.txt")?;

// 获取文件元数据
let metadata = fs::metadata("test.txt")?;
println!("文件大小: {} 字节", metadata.len());
```

### 8. 错误处理模式 (`error_handling_patterns`)

```rust
// Result 类型处理
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 使用 ? 操作符
fn complex_calculation(x: f64) -> Result<f64, String> {
    let result = divide(x, 2.0)?;
    Ok(result * 2.0)
}
```

## 🚀 运行示例

```bash
# 运行基础语法模块
cargo run basics

# 运行所有模块
cargo run
```

## 📝 练习题

### 初级
1. 声明一个可变变量 `count`，初始值为 0，然后将其增加 10
2. 编写一个函数 `is_even`，判断一个整数是否为偶数
3. 使用 `for` 循环打印 1 到 10 的所有数字

### 中级
1. 实现一个简单的链表结构
2. 编写一个函数，计算斐波那契数列的第 n 项（递归实现）
3. 使用迭代器实现：找出向量中所有大于平均值的元素

### 高级
1. 实现一个泛型的二叉搜索树
2. 编写一个宏来简化向量创建
3. 实现一个简单的表达式解析器

## 🔗 相关资源

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## ⚠️ 常见陷阱

1. **变量遮蔽**：Rust 允许同名变量遮蔽，但可能导致意外行为
2. **整数溢出**：debug 模式会检查溢出，release 模式不会
3. **浮点数比较**：不要直接使用 `==` 比较浮点数

## 📊 学习检查清单

- [ ] 理解变量可变性和不可变性
- [ ] 掌握基本数据类型
- [ ] 能够定义和调用函数
- [ ] 熟悉 match 模式匹配
- [ ] 理解所有权基础概念
- [ ] 会使用迭代器和闭包
- [ ] 掌握基本文件操作
- [ ] 理解 Result 错误处理
