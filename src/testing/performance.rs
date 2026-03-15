//! 教学主题四：性能测试、基准思路与集成场景。
//!
//! 这一层聚焦“当基础断言已经掌握后，如何理解更复杂的测试形态”：
//! - 性能测试与基准思路 `performance_testing_examples`
//! - 集成测试场景 `integration_testing_scenarios`
//! - 汇总所有教学分层入口 `run_all_testing_examples`

use std::time::Instant;

use super::documentation::run_testing_examples;
use super::strategies::{
    boundary_and_error_testing, enterprise_testing_strategies, property_thinking_basics,
    test_driven_development_example,
};

/// 演示性能测试和基准思路。
pub fn performance_testing_examples() {
    println!("⚡ 性能测试与基准思路演示：");
    println!("ℹ️ 下列输出只用于帮助理解性能观察方式，不构成可复现、可比较的正式 benchmark。\n");

    let large_dataset: Vec<i64> = (1..100000).collect();

    let start_time = Instant::now();
    let result: i64 = large_dataset
        .iter()
        .filter(|&&value| value % 2 == 0)
        .map(|&value| value * value)
        .sum();
    let processing_time = start_time.elapsed();

    println!("📈 大数据集处理性能:");
    println!("  数据量: {} 个元素", large_dataset.len());
    println!("  处理结果: {}", result);
    println!("  处理时间: {:.2}ms", processing_time.as_millis());

    string_operations_performance();

    println!("\n📊 数据结构性能对比:");
    let data: Vec<i32> = (1..10000).collect();
    let hash_map: std::collections::HashMap<i32, i32> = data
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, value)| (value, index as i32))
        .collect();

    let start_vec = Instant::now();
    let found_in_vec = data.iter().find(|&&value| value == 5000);
    let vec_time = start_vec.elapsed();

    let start_map = Instant::now();
    let found_in_map = hash_map.get(&5000);
    let map_time = start_map.elapsed();

    println!(
        "  Vec 查找 5000: {} (耗时: {:.2}μs)",
        if found_in_vec.is_some() { "找到" } else { "未找到" },
        vec_time.as_micros()
    );
    println!(
        "  HashMap 查找 5000: {} (耗时: {:.2}μs)",
        if found_in_map.is_some() { "找到" } else { "未找到" },
        map_time.as_micros()
    );

    println!("\n📊 排序算法耗时观察:");
    let mut bubble_data = (1..1000).rev().collect::<Vec<_>>();
    let baseline_data = bubble_data.clone();

    let start_bubble = Instant::now();
    bubble_sort(&mut bubble_data);
    let bubble_time = start_bubble.elapsed();
    println!("  冒泡排序: {:.2}ms", bubble_time.as_millis());

    let start_quick = Instant::now();
    let mut quick_data = baseline_data.clone();
    let len = quick_data.len();
    quick_sort(&mut quick_data, 0, len - 1);
    let quick_time = start_quick.elapsed();
    println!("  快速排序: {:.2}ms", quick_time.as_millis());

    let start_std = Instant::now();
    let mut std_data = baseline_data.clone();
    std_data.sort();
    let std_time = start_std.elapsed();
    println!("  标准库排序: {:.2}ms", std_time.as_millis());

    println!("\n📊 内存使用性能测试:");
    memory_performance_test();

    println!("📊 性能测试演示完成");
}

/// 演示集成测试场景。
pub fn integration_testing_scenarios() {
    println!("🔗 集成测试场景：");

    let mut processor = OrderProcessor::new();

    println!("📦 订单处理系统集成测试:");

    let test_items = vec![
        OrderItem {
            product_id: 1,
            quantity: 2,
            price: 29.99,
        },
        OrderItem {
            product_id: 2,
            quantity: 1,
            price: 99.99,
        },
    ];

    match processor.create_order(test_items) {
        Ok(order_id) => {
            println!("✅ 订单创建成功，ID: {}", order_id);

            match processor.confirm_order(order_id) {
                Ok(()) => {
                    println!("✅ 订单确认成功");

                    if let Some(order) = processor.get_order(order_id) {
                        println!(
                            "📊 订单详情: 状态={:?}, 总金额=${:.2}",
                            order.status, order.total
                        );
                    }
                }
                Err(error) => println!("❌ 订单确认失败: {}", error),
            }
        }
        Err(error) => println!("❌ 订单创建失败: {}", error),
    }

    match processor.create_order(vec![]) {
        Ok(_) => println!("❌ 空订单不应该创建成功"),
        Err(error) => println!("✅ 空订单正确拒绝: {}", error),
    }

    println!("📊 集成测试完成");
}

/// 运行所有测试示例。
pub fn run_all_testing_examples() {
    println!("🎯 === 全面测试示例 ===");
    println!();

    println!("=== 基础测试示例 ===");
    run_testing_examples();
    println!();

    println!("=== 企业级测试策略 ===");
    enterprise_testing_strategies();
    println!();

    println!("=== 属性思维基础 ===");
    property_thinking_basics();
    println!();

    println!("=== 性能测试示例 ===");
    performance_testing_examples();
    println!();

    println!("=== 集成测试场景 ===");
    integration_testing_scenarios();
    println!();

    println!("=== TDD示例 ===");
    test_driven_development_example();
    println!();

    println!("=== 边界条件测试 ===");
    boundary_and_error_testing();

    println!("\n✅ 所有测试示例运行完成！");
}

fn string_operations_performance() {
    let start = Instant::now();

    let mut result = String::with_capacity(10000 * 7);
    for i in 0..10000 {
        result.push_str("Item ");
        result.push_str(&i.to_string());
        result.push(' ');
    }

    let processing_time = start.elapsed();
    println!("📈 字符串操作性能:");
    println!("  操作次数: 10000");
    println!("  结果长度: {} 字符", result.len());
    println!("  处理时间: {:.2}ms", processing_time.as_millis());
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        if pivot_index > 0 {
            quick_sort(arr, low, pivot_index - 1);
        }
        quick_sort(arr, pivot_index + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn memory_performance_test() {
    let start_stack = Instant::now();
    let stack_array: [i32; 10000] = [0; 10000];
    let stack_time = start_stack.elapsed();

    let start_heap = Instant::now();
    let heap_array = vec![0i32; 10000];
    let heap_time = start_heap.elapsed();

    println!("  栈分配 10000 个 i32: {:.2}μs", stack_time.as_micros());
    println!("  堆分配 10000 个 i32: {:.2}μs", heap_time.as_micros());
    println!("  栈数组第一个元素: {}", stack_array[0]);
    println!("  堆数组第一个元素: {}", heap_array[0]);
}

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    items: Vec<OrderItem>,
    total: f64,
    status: OrderStatus,
}

#[derive(Debug, Clone)]
struct OrderItem {
    product_id: u32,
    quantity: u32,
    price: f64,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
enum OrderProcessingError {
    #[error("订单不能为空")]
    EmptyOrder,
    #[error("订单状态不允许确认")]
    InvalidStatusTransition,
    #[error("订单不存在")]
    OrderNotFound,
}

struct OrderProcessor {
    orders: std::collections::HashMap<u32, Order>,
    next_id: u32,
}

impl OrderProcessor {
    fn new() -> Self {
        Self {
            orders: std::collections::HashMap::new(),
            next_id: 1,
        }
    }

    fn create_order(&mut self, items: Vec<OrderItem>) -> Result<u32, OrderProcessingError> {
        if items.is_empty() {
            return Err(OrderProcessingError::EmptyOrder);
        }

        let total: f64 = items
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum();

        let order = Order {
            id: self.next_id,
            items,
            total,
            status: OrderStatus::Pending,
        };

        self.orders.insert(self.next_id, order);
        let order_id = self.next_id;
        self.next_id += 1;

        Ok(order_id)
    }

    fn confirm_order(&mut self, order_id: u32) -> Result<(), OrderProcessingError> {
        if let Some(order) = self.orders.get_mut(&order_id) {
            match order.status {
                OrderStatus::Pending => {
                    order.status = OrderStatus::Confirmed;
                    Ok(())
                }
                _ => Err(OrderProcessingError::InvalidStatusTransition),
            }
        } else {
            Err(OrderProcessingError::OrderNotFound)
        }
    }

    fn get_order(&self, order_id: u32) -> Option<&Order> {
        self.orders.get(&order_id)
    }
}

#[cfg(test)]
mod timing_observation_tests {
    use super::*;

    #[test]
    fn observe_string_building_time() {
        let iterations = 1000;

        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(300);
            for i in 0..100 {
                s.push_str(&i.to_string());
            }
        }
        let push_str_time = start.elapsed();

        let start = Instant::now();
        for _ in 0..iterations {
            let mut s = String::with_capacity(300);
            for i in 0..100 {
                use std::fmt::Write;
                write!(&mut s, "{i}").expect("write! to String should not fail");
            }
        }
        let write_macro_time = start.elapsed();

        println!("push_str 构建: {:?}", push_str_time);
        println!("write! 构建: {:?}", write_macro_time);

        assert!(push_str_time >= std::time::Duration::ZERO);
        assert!(write_macro_time >= std::time::Duration::ZERO);
    }

    #[test]
    fn observe_data_structure_build_time() {
        let data_size = 10000;

        let start = Instant::now();
        let mut vec = Vec::new();
        for i in 0..data_size {
            vec.push(i);
        }
        let vec_insert_time = start.elapsed();

        let start = Instant::now();
        let mut hashmap = std::collections::HashMap::new();
        for i in 0..data_size {
            hashmap.insert(i, i);
        }
        let hashmap_insert_time = start.elapsed();

        println!("Vec 插入 {} 元素: {:?}", data_size, vec_insert_time);
        println!("HashMap 插入 {} 元素: {:?}", data_size, hashmap_insert_time);

        assert_eq!(vec.len(), data_size as usize);
        assert_eq!(hashmap.len(), data_size as usize);
    }

    #[test]
    fn observe_iteration_patterns() {
        let data: Vec<i32> = (1..=10000).collect();

        let start = Instant::now();
        let mut sum1: i64 = 0;
        for &item in &data {
            sum1 += item as i64;
        }
        let for_loop_time = start.elapsed();

        let start = Instant::now();
        let sum2: i64 = data.iter().map(|&x| x as i64).sum();
        let sum_time = start.elapsed();

        let start = Instant::now();
        let sum3: i64 = data.iter().fold(0i64, |acc, &x| acc + x as i64);
        let fold_time = start.elapsed();

        println!("for 循环求和: {:?}, 结果: {}", for_loop_time, sum1);
        println!("iter().sum() 求和: {:?}, 结果: {}", sum_time, sum2);
        println!("fold 求和: {:?}, 结果: {}", fold_time, sum3);

        assert_eq!(sum1, sum2);
        assert_eq!(sum1, sum3);
    }
}

#[cfg(test)]
mod conditional_tests {
    #[test]
    #[cfg(target_os = "windows")]
    fn test_platform_windows() {
        println!("在 Windows 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_platform_linux() {
        println!("在 Linux 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_platform_macos() {
        println!("在 macOS 平台运行测试");
        assert!(true);
    }

    #[test]
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    fn test_platform_other() {
        println!("在未知平台运行测试");
        assert!(true);
    }
}
