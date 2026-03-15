//! 教学主题二：从文档测试到基础演示入口。
//!
//! 这一层关注“学习者第一次接触测试模块时会看到什么”：
//! - 文档测试函数 `documented_function`
//! - 轻量耗时观察 `timed_operations_demo`
//! - 聚合基础示例的 `run_testing_examples`

use std::time::Instant;

use super::domain::{User, UserManager, add_two, greeting};

/// 现代化文档测试。
///
/// # 示例
///
/// ```
/// let result = rust_learn::testing::greeting("小王");
/// assert_eq!(result, "你好，小王！");
///
/// let user = rust_learn::testing::User::new(
///     "小李".to_string(),
///     "xiaoli@example.com".to_string(),
///     25
/// ).unwrap();
///
/// assert_eq!(user.name, "小李");
/// assert!(user.is_adult());
/// assert_eq!(user.greet(), "你好，小李！");
/// ```
pub fn documented_function() {
    println!("这个函数包含文档测试示例");
}

/// 轻量耗时观察示例。
///
/// 这里故意不称为 benchmark：它只演示如何在教学代码中
/// 用 `Instant` 做粗粒度耗时观察，结果会受到环境与运行状态影响。
pub fn timed_operations_demo() {
    println!("🧪 轻量耗时观察：");

    let large_data: Vec<i32> = (1..10000).collect();

    let start = Instant::now();
    let even_count = large_data.iter().filter(|&&value| value % 2 == 0).count();
    let even_count_elapsed = start.elapsed();
    println!(
        "偶数数量: {}（耗时观察: {:?}）",
        even_count, even_count_elapsed
    );

    let start = Instant::now();
    let squared: Vec<i32> = large_data.iter().map(|&value| value * value).take(10).collect();
    let squared_elapsed = start.elapsed();
    println!(
        "前10个平方数: {:?}（耗时观察: {:?}）",
        squared, squared_elapsed
    );

    let target = 5000;
    let start = Instant::now();
    let found = large_data.iter().find(|&&value| value == target);
    let search_elapsed = start.elapsed();
    println!(
        "查找 {} 结果: {:?}（耗时观察: {:?}）",
        target,
        found.is_some(),
        search_elapsed
    );
}

/// 运行测试和文档示例。
pub fn run_testing_examples() {
    println!("🧪 === 现代化测试和文档示例 ===");
    println!();

    println!("数学运算:");
    println!("5 + 2 = {}", add_two(5));
    println!("-3 + 2 = {}", add_two(-3));
    println!();

    println!("用户管理:");
    let mut manager = UserManager::new();

    match User::new("张三".to_string(), "zhangsan@example.com".to_string(), 25) {
        Ok(user) => {
            manager.add_user(user).expect("Failed to add user");
            println!("✅ 用户添加成功");
        }
        Err(error) => println!("❌ 用户创建失败: {}", error),
    }

    match User::new("李四".to_string(), "invalid-email".to_string(), 25) {
        Ok(user) => {
            manager.add_user(user).expect("Failed to add user");
            println!("✅ 用户添加成功");
        }
        Err(error) => println!("❌ 用户创建失败: {}", error),
    }

    println!("当前用户数量: {}", manager.user_count());
    println!();

    println!("问候测试:");
    println!("问候: {}", greeting("世界"));
    println!("问候: {}", greeting("Rust 爱好者"));
    println!();

    documented_function();
    println!();

    timed_operations_demo();
    println!();

    #[cfg(test)]
    {
        println!("💡 提示：使用 'cargo test' 运行所有测试");
        println!("💡 提示：使用 'cargo test --doc' 运行文档测试");
    }

    println!("\n✅ 所有测试和文档示例运行完成！");
}
