//! # 测试和文档模块
//!
//! 这个模块演示了Rust的测试功能和文档生成。

/// 一个简单的函数，用于测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// 另一个函数，用于测试私有函数
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

/// 演示单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
        assert_eq!(add_two(-1), 1);
    }

    #[test]
    fn test_internal_adder() {
        assert_eq!(internal_adder(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test should panic");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

/// 演示集成测试（这个函数会被集成测试调用）
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

/// 演示文档测试
/// ```
/// let result = rust_learn::testing::greeting("Carol");
/// assert_eq!(result, "Hello Carol!");
/// ```
pub fn documented_function() {
    println!("This function has documentation tests");
}

/// 演示基准测试（需要nightly Rust和criterion crate）
#[cfg(feature = "bench")]
pub fn benchmark_example() {
    // 这里可以放置基准测试代码
    // 需要启用bench feature和criterion依赖
}

/// 演示条件编译测试
#[cfg(test)]
mod conditional_tests {
    #[test]
    #[cfg(target_os = "windows")]
    fn test_on_windows() {
        assert!(true);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_not_on_windows() {
        assert!(true);
    }
}

/// 运行测试和文档示例
pub fn run_testing_examples() {
    println!("=== 测试和文档示例 ===");
    println!("Add two to 5: {}", add_two(5));
    println!("Greeting: {}", greeting("World"));
    documented_function();

    // 运行单元测试
    #[cfg(test)]
    {
        println!("Running tests...");
        // 在实际运行时，cargo test会运行这些测试
    }
}