
//! # 基准测试套件
//!
//! 这个模块包含项目的完整基准测试，用于测量各种操作和算法的性能。
//!
//! ## 运行基准测试
//!
//! ```bash
//! # 运行所有基准测试
//! cargo bench
//!
//! # 运行特定基准测试
//! cargo bench --bench data_structures
//! cargo bench --bench algorithms
//! cargo bench --bench collections
//!
//! # 运行并保存结果
//! cargo bench -- --save-baseline main
//!
//! # 与基线比较
//! cargo bench -- --baseline main
//! ```
//!
//! ## 基准测试类别
//!
//! - **data_structures** - 基础数据结构操作性能
//! - **algorithms** - 算法性能（排序、搜索等）
//! - **collections** - 集合类型性能对比
//! - **strings** - 字符串处理性能
//! - **concurrency** - 并发操作性能
//! - **projects** - 实战项目性能

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

// 配置基准测试
criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .sample_size(100);
    targets = 
        data_structure_benchmarks,
        algorithm_benchmarks,
        collection_benchmarks,
        string_benchmarks,
        concurrency_benchmarks,
        project_benchmarks
);

criterion_main!(benches);

// ==================== 数据结构基准测试 ====================

fn data_structure_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");
    
    // Vec 操作基准测试
    group.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }
            vec
        });
    });
    
    group.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i);
            }
            vec
        });
    });
    
    // 不同大小数据的 Vec 操作
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("vec_insert", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::new();
                    for i in 0..size {
                        vec.push(i);
                    }
                    vec
                });
            },
        );
    }
    
    // 栈操作
    group.bench_function("stack_operations", |b| {
        b.iter(|| {
            let mut stack: Vec<i32> = Vec::new();
            for i in 0..1000 {
                stack.push(i);
            }
            while let Some(_) = stack.pop() {}
            stack
        });
    });
    
    // 队列操作（使用 VecDeque）
    group.bench_function("queue_operations", |b| {
        use std::collections::VecDeque;
        b.iter(|| {
            let mut queue: VecDeque<i32> = VecDeque::new();
            for i in 0..1000 {
                queue.push_back(i);
            }
            while let Some(_) = queue.pop_front() {}
            queue
        });
    });
    
    group.finish();
}

// ==================== 算法基准测试 ====================

fn algorithm_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithms");
    
    // 排序算法对比
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("std_sort", size),
            size,
            |b, &size| {
                b.iter_with_setup(
                    || (0..size).rev().collect::<Vec<i32>>(),
                    |mut data| {
                        data.sort();
                        data
                    },
                );
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("std_sort_unstable", size),
            size,
            |b, &size| {
                b.iter_with_setup(
                    || (0..size).rev().collect::<Vec<i32>>(),
                    |mut data| {
                        data.sort_unstable();
                        data
                    },
                );
            },
        );
    }
    
    // 搜索算法
    group.bench_function("linear_search", |b| {
        let data: Vec<i32> = (0..10000).collect();
        b.iter(|| {
            data.iter().find(|&&x| x == 9999)
        });
    });
    
    group.bench_function("binary_search", |b| {
        let data: Vec<i32> = (0..10000).collect();
        b.iter(|| {
            data.binary_search(&9999)
        });
    });
    
    // 二分查找 vs 线性查找对比
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("search_linear", size),
            size,
            |b, &size| {
                let data: Vec<i32> = (0..size).collect();
                b.iter(|| {
                    data.iter().find(|&&x| x == size - 1)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("search_binary", size),
            size,
            |b, &size| {
                let data: Vec<i32> = (0..size).collect();
                b.iter(|| {
                    data.binary_search(&(size - 1))
                });
            },
        );
    }
    
    // 斐波那契数列计算
    group.bench_function("fibonacci_recursive", |b| {
        fn fib(n: u64) -> u64 {
            match n {
                0 => 0,
                1 => 1,
                _ => fib(n - 1) + fib(n - 2),
            }
        }
        b.iter(|| fib(20));
    });
    
    group.bench_function("fibonacci_iterative", |b| {
        fn fib(n: u64) -> u64 {
            if n <= 1 {
                return n;
            }
            let mut prev = 0;
            let mut curr = 1;
            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
        b.iter(|| fib(20));
    });
    
    group.finish();
}

// ==================== 集合操作基准测试 ====================

fn collection_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");
    
    // HashMap vs BTreeMap 插入性能
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("hashmap_insert", size),
            size,
            |b, &size| {
                use std::collections::HashMap;
                b.iter(|| {
                    let mut map = HashMap::new();
                    for i in 0..size {
                        map.insert(i, i * 2);
                    }
                    map
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("btreemap_insert", size),
            size,
            |b, &size| {
                use std::collections::BTreeMap;
                b.iter(|| {
                    let mut map = BTreeMap::new();
                    for i in 0..size {
                        map.insert(i, i * 2);
                    }
                    map
                });
            },
        );
    }
    
    // HashMap vs BTreeMap 查找性能
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("hashmap_lookup", size),
            size,
            |b, &size| {
                use std::collections::HashMap;
                let map: HashMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();
                b.iter(|| {
                    map.get(&(size - 1))
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("btreemap_lookup", size),
            size,
            |b, &size| {
                use std::collections::BTreeMap;
                let map: BTreeMap<i32, i32> = (0..size).map(|i| (i, i * 2)).collect();
                b.iter(|| {
                    map.get(&(size - 1))
                });
            },
        );
    }
    
    // HashSet 操作
    group.bench_function("hashset_operations", |b| {
        use std::collections::HashSet;
        b.iter(|| {
            let mut set = HashSet::new();
            for i in 0..1000 {
                set.insert(i);
            }
            set.contains(&500)
        });
    });
    
    // Vec vs LinkedList 性能对比
    group.bench_function("vec_random_access", |b| {
        let vec: Vec<i32> = (0..10000).collect();
        b.iter(|| {
            vec[5000]
        });
    });
    
    group.finish();
}

// ==================== 字符串处理基准测试 ====================

fn string_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("strings");
    
    // 字符串拼接方法对比
    group.bench_function("string_push_str", |b| {
        b.iter(|| {
            let mut s = String::new();
            for i in 0..100 {
                s.push_str(&format!("item{}", i));
            }
            s
        });
    });
    
    group.bench_function("string_join", |b| {
        b.iter(|| {
            let items: Vec<String> = (0..100).map(|i| format!("item{}", i)).collect();
            items.join(", ")
        });
    });
    
    group.bench_function("string_with_capacity", |b| {
        b.iter(|| {
            let mut s = String::with_capacity(1000);
            for i in 0..100 {
                s.push_str(&format!("item{}", i));
            }
            s
        });
    });
    
    // 字符串搜索
    let text = "The quick brown fox jumps over the lazy dog".to_string();
    group.bench_function("string_find", |b| {
        b.iter(|| {
            text.find("fox")
        });
    });
    
    // 正则表达式 vs 简单搜索
    group.bench_function("string_contains", |b| {
        let text = "The quick brown fox jumps over the lazy dog";
        b.iter(|| {
            text.contains("fox")
        });
    });
    
    // 字符串分割
    group.bench_function("string_split", |b| {
        let text = "apple,banana,cherry,date,elderberry";
        b.iter(|| {
            text.split(',').collect::<Vec<_>>()
        });
    });
    
    group.finish();
}

// ==================== 并发性能基准测试 ====================

fn concurrency_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency");
    
    // 单线程 vs 多线程计算
    group.bench_function("single_threaded_sum", |b| {
        b.iter(|| {
            let data: Vec<i32> = (0..1000000).collect();
            data.iter().sum::<i32>()
        });
    });
    
    group.bench_function("multi_threaded_sum", |b| {
        use std::thread;
        b.iter_with_setup(
            || (0..1000000).collect::<Vec<i32>>(),
            |data| {
                let mid = data.len() / 2;
                let left_data: Vec<i32> = data[..mid].to_vec();
                let right_data: Vec<i32> = data[mid..].to_vec();
                
                let left_sum = thread::spawn(move || {
                    left_data.iter().sum::<i32>()
                });
                
                let right_sum = thread::spawn(move || {
                    right_data.iter().sum::<i32>()
                });
                
                left_sum.join().unwrap() + right_sum.join().unwrap()
            },
        );
    });
    
    // Mutex vs RwLock 性能对比
    group.bench_function("mutex_contention", |b| {
        use std::sync::{Arc, Mutex};
        b.iter(|| {
            let data = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            
            for _ in 0..10 {
                let data = Arc::clone(&data);
                handles.push(std::thread::spawn(move || {
                    for _ in 0..100 {
                        let mut num = data.lock().unwrap();
                        *num += 1;
                    }
                }));
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
            
            *data.lock().unwrap()
        });
    });
    
    group.bench_function("rwlock_read_heavy", |b| {
        use std::sync::{Arc, RwLock};
        b.iter(|| {
            let data = Arc::new(RwLock::new(0));
            let mut handles = vec![];
            
            // 多个读操作
            for _ in 0..10 {
                let data = Arc::clone(&data);
                handles.push(std::thread::spawn(move || {
                    for _ in 0..100 {
                        let _ = *data.read().unwrap();
                    }
                }));
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        });
    });
    
    // 通道性能
    group.bench_function("channel_throughput", |b| {
        use std::sync::mpsc;
        b.iter(|| {
            let (tx, rx) = mpsc::channel();
            
            std::thread::spawn(move || {
                for i in 0..1000 {
                    tx.send(i).unwrap();
                }
            });
            
            let mut sum = 0;
            for _ in 0..1000 {
                sum += rx.recv().unwrap();
            }
            sum
        });
    });
    
    group.finish();
}

// ==================== 实战项目基准测试 ====================

fn project_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("projects");
    
    // 任务管理器性能测试
    group.bench_function("task_manager_add", |b| {
        use rust_learn::projects::task_manager::{Task, TaskManager, Priority};
        b.iter(|| {
            let mut manager = TaskManager::new().unwrap();
            for i in 0..100 {
                let task = Task::new(0, format!("Task {}", i), Priority::Medium);
                manager.add_task(task);
            }
            manager
        });
    });
    
    group.bench_function("task_manager_list", |b| {
        use rust_learn::projects::task_manager::{Task, TaskManager, Priority};
        let mut manager = TaskManager::new().unwrap();
        for i in 0..100 {
            let task = Task::new(0, format!("Task {}", i), Priority::Medium);
            manager.add_task(task);
        }
        
        b.iter(|| {
            manager.list_tasks(None)
        });
    });
    
    group.bench_function("task_manager_search", |b| {
        use rust_learn::projects::task_manager::{Task, TaskManager, Priority};
        let mut manager = TaskManager::new().unwrap();
        for i in 0..100 {
            let task = Task::new(0, format!("Task {}", i), Priority::Medium);
            manager.add_task(task);
        }
        
        b.iter(|| {
            manager.search_tasks("Task 50")
        });
    });
    
    // JSON 序列化/反序列化性能
    group.bench_function("json_serialize", |b| {
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, Clone)]
        struct Data {
            id: u64,
            name: String,
            values: Vec<i32>,
        }
        
        let data = Data {
            id: 1,
            name: "Test".to_string(),
            values: (0..1000).collect(),
        };
        
        b.iter(|| {
            serde_json::to_string(&data).unwrap()
        });
    });
    
    group.bench_function("json_deserialize", |b| {
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, Clone)]
        struct Data {
            id: u64,
            name: String,
            values: Vec<i32>,
        }
        
        let data = Data {
            id: 1,
            name: "Test".to_string(),
            values: (0..1000).collect(),
        };
        let json = serde_json::to_string(&data).unwrap();
        
        b.iter(|| {
            let _: Data = serde_json::from_str(&json).unwrap();
        });
    });
    
    group.finish();
}
