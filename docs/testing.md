# 测试模块 (testing)

## 📖 模块概述

测试是保证代码质量的重要手段。Rust 内置了强大的测试框架，支持单元测试、集成测试和文档测试。本模块讲解 Rust 测试的最佳实践。

## 🎯 学习目标

- 掌握单元测试的编写
- 理解集成测试的组织
- 学会编写文档测试
- 掌握测试驱动开发（TDD）
- 了解属性测试和基准测试

## 📚 内容目录

### 1. 单元测试基础

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("计算错误".to_string())
        }
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("这个测试应该 panic");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 耗时测试，默认跳过
    }
}
```

### 2. 断言宏

```rust
// 相等断言
assert_eq!(actual, expected);
assert_ne!(a, b);

// 布尔断言
assert!(condition);
assert!(!condition);

// 带消息断言
assert_eq!(a, b, "a 应该等于 b");
assert!(value > 0, "值必须为正数，实际是 {}", value);

// 浮点数比较
let result = 0.1 + 0.2;
assert!((result - 0.3).abs() < f64::EPSILON);
```

### 3. 文档测试

```rust
/// 计算两个数的和
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 忽略文档测试中的某些行
///
/// ```ignore
/// let result = expensive_operation();
/// ```
pub fn expensive_operation() -> i32 {
    42
}
```

### 4. 集成测试

```rust
// tests/integration_test.rs
use my_crate::*;

#[test]
fn test_integration() {
    // 测试公共 API
    let result = public_function();
    assert!(result.is_ok());
}

// 测试模块组织
mod user_tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Alice".to_string());
        assert_eq!(user.name, "Alice");
    }
}
```

### 5. 测试组织结构

```
project/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── integration_test.rs    # 集成测试
│   └── common/
│       └── mod.rs             # 测试辅助模块
└── Cargo.toml
```

### 6. 属性测试 (`property_based_testing_basics`)

```rust
// 使用 proptest 库
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reverse_twice(ref s in ".*") {
        let reversed: String = s.chars().rev().collect();
        let double_reversed: String = reversed.chars().rev().collect();
        assert_eq!(s, &double_reversed);
    }
    
    #[test]
    fn test_addition_commutative(a in 0..1000i32, b in 0..1000i32) {
        assert_eq!(a + b, b + a);
    }
}

// 手动属性测试
fn test_properties() {
    // 交换律
    for a in 0..100 {
        for b in 0..100 {
            assert_eq!(a + b, b + a);
        }
    }
    
    // 结合律
    for a in 0..50 {
        for b in 0..50 {
            for c in 0..50 {
                assert_eq!((a + b) + c, a + (b + c));
            }
        }
    }
}
```

### 7. 性能测试 (`performance_testing_examples`)

```rust
use std::time::Instant;

#[test]
fn benchmark_sort() {
    let data: Vec<i32> = (0..10000).rev().collect();
    
    let start = Instant::now();
    let mut sorted = data.clone();
    sorted.sort();
    let duration = start.elapsed();
    
    println!("排序耗时: {:?}", duration);
    assert!(duration.as_millis() < 100);
}

// 使用 criterion 库
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 8. 测试驱动开发 (TDD)

```rust
// 1. 先写测试
#[test]
fn test_calculator_add() {
    let mut calc = Calculator::new();
    assert_eq!(calc.add(2, 3), 5);
}

// 2. 实现功能
pub struct Calculator {
    history: Vec<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }
    
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = a + b;
        self.history.push(result);
        result
    }
}

// 3. 重构和优化
impl Calculator {
    pub fn get_history(&self) -> &[f64] {
        &self.history
    }
    
    pub fn clear(&mut self) {
        self.history.clear();
    }
}
```

### 9. 边界条件测试 (`boundary_and_error_testing`)

```rust
#[test]
fn test_boundary_conditions() {
    // 边界值
    assert_eq!(divide(i32::MAX, 1), Ok(i32::MAX));
    assert_eq!(divide(i32::MIN, 1), Ok(i32::MIN));
    assert!(divide(1, 0).is_err());
    
    // 空输入
    assert!(process_empty_vec(&[]).is_ok());
    
    // 极端情况
    assert!(handle_large_input(&vec![0; 1000000]).is_ok());
}

#[test]
fn test_error_cases() {
    // 无效输入
    assert!(parse_number("abc").is_err());
    assert!(parse_number("").is_err());
    
    // 边界外值
    assert!(validate_age(-1).is_err());
    assert!(validate_age(151).is_err());
}
```

## 🚀 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行特定模块测试
cargo test module_name

# 运行文档测试
cargo test --doc

# 运行被忽略的测试
cargo test -- --ignored

# 显示测试输出
cargo test -- --nocapture

# 并行运行测试
cargo test -- --test-threads=4

# 运行基准测试
cargo bench
```

## 📊 测试金字塔

```
┌─────────────────────────────────────────────────────────────┐
│                     测试金字塔                               │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                        /\                                   │
│                       /  \                                  │
│                      / UI \                                 │
│                     /------\                                │
│                    / 集成测试 \                              │
│                   /----------\                              │
│                  /   单元测试   \                            │
│                 /--------------\                            │
│                /    数量递增     \                          │
│               /------------------\                          │
│                                                             │
│  单元测试: 测试单个函数/方法                                 │
│  集成测试: 测试模块间交互                                    │
│  UI测试:  测试用户界面                                       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 为一个加法函数编写单元测试
2. 编写一个测试验证字符串反转函数
3. 使用 `#[should_panic]` 测试 panic 情况

### 中级
1. 编写一个完整的测试套件，包含正常和边界情况
2. 使用 proptest 编写属性测试
3. 编写集成测试测试公共 API

### 高级
1. 实现一个自定义测试框架
2. 编写基准测试比较不同算法性能
3. 使用 mock 对象测试外部依赖

## 🔗 相关资源

- [Rust 测试文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [proptest 库](https://docs.rs/proptest)
- [criterion 库](https://docs.rs/criterion)
- [mockall 库](https://docs.rs/mockall)

## ⚠️ 常见陷阱

### 1. 测试依赖顺序
```rust
// ❌ 测试不应该依赖执行顺序
#[test]
fn test_a() {
    // 修改全局状态
}

#[test]
fn test_b() {
    // 依赖 test_a 的结果
}

// ✅ 每个测试应该独立
#[test]
fn test_a() {
    let mut state = create_fresh_state();
    // ...
}

#[test]
fn test_b() {
    let mut state = create_fresh_state();
    // ...
}
```

### 2. 忽略测试失败
```rust
// ❌ 不要忽略失败的断言
#[test]
fn test_something() {
    let result = some_function();
    if result.is_err() {
        println!("失败了");  // 测试仍然通过！
    }
}

// ✅ 正确处理失败
#[test]
fn test_something() {
    let result = some_function();
    assert!(result.is_ok(), "函数应该成功");
}
```

### 3. 测试覆盖不足
```rust
// ❌ 只测试正常情况
#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);
}

// ✅ 测试所有情况
#[test]
fn test_divide_normal() {
    assert_eq!(divide(10, 2), 5);
}

#[test]
fn test_divide_zero() {
    assert!(divide(10, 0).is_err());
}

#[test]
fn test_divide_negative() {
    assert_eq!(divide(-10, 2), -5);
}
```

## 📊 学习检查清单

- [ ] 掌握单元测试编写
- [ ] 理解各种断言宏
- [ ] 会编写文档测试
- [ ] 理解集成测试组织
- [ ] 掌握 TDD 流程
- [ ] 了解属性测试
- [ ] 会编写基准测试
- [ ] 能够测试边界条件
