# Rust 学习项目 - 基准测试套件

这个目录包含项目的完整基准测试套件，用于测量和比较各种 Rust 操作和算法的性能。

## 📊 基准测试类别

### 1. 数据结构基准测试 (`data_structures`)
测试基础数据结构的操作性能：

- **Vec 操作**
  - `vec_push` - 动态扩容 vs 预分配容量
  - `vec_insert` - 不同数据规模的插入性能
  
- **栈和队列**
  - `stack_operations` - Vec 作为栈的性能
  - `queue_operations` - VecDeque 作为队列的性能

### 2. 算法基准测试 (`algorithms`)
测试常用算法的性能：

- **排序算法**
  - `std_sort` - 标准库稳定排序
  - `std_sort_unstable` - 标准库非稳定排序
  
- **搜索算法**
  - `linear_search` - 线性查找
  - `binary_search` - 二分查找
  - 不同数据规模的查找对比
  
- **递归 vs 迭代**
  - `fibonacci_recursive` - 递归斐波那契
  - `fibonacci_iterative` - 迭代斐波那契

### 3. 集合操作基准测试 (`collections`)
测试不同集合类型的性能：

- **HashMap vs BTreeMap**
  - `hashmap_insert` / `btreemap_insert` - 插入性能
  - `hashmap_lookup` / `btreemap_lookup` - 查找性能
  
- **HashSet**
  - `hashset_operations` - 集合操作性能
  
- **Vec 随机访问**
  - `vec_random_access` - 随机访问性能

### 4. 字符串处理基准测试 (`strings`)
测试字符串操作的性能：

- **字符串拼接**
  - `string_push_str` - 逐个 push_str
  - `string_join` - 使用 join 方法
  - `string_with_capacity` - 预分配容量
  
- **字符串搜索**
  - `string_find` - 查找子串
  - `string_contains` - 包含检查
  - `string_split` - 分割字符串

### 5. 并发性能基准测试 (`concurrency`)
测试并发操作的性能：

- **单线程 vs 多线程**
  - `single_threaded_sum` - 单线程求和
  - `multi_threaded_sum` - 多线程求和
  
- **同步原语**
  - `mutex_contention` - Mutex 竞争性能
  - `rwlock_read_heavy` - RwLock 读多写少场景
  
- **通道**
  - `channel_throughput` - 通道吞吐量

### 6. 实战项目基准测试 (`projects`)
测试实战项目的性能：

- **任务管理器**
  - `task_manager_add` - 添加任务性能
  - `task_manager_list` - 列出任务性能
  - `task_manager_search` - 搜索任务性能
  
- **JSON 处理**
  - `json_serialize` - JSON 序列化
  - `json_deserialize` - JSON 反序列化

## 🚀 运行基准测试

### 运行所有基准测试
```bash
cargo bench
```

### 运行特定类别的基准测试
```bash
# 只运行数据结构测试
cargo bench data_structures

# 只运行算法测试
cargo bench algorithms

# 只运行集合测试
cargo bench collections

# 只运行字符串测试
cargo bench strings

# 只运行并发测试
cargo bench concurrency

# 只运行项目测试
cargo bench projects
```

### 运行特定测试
```bash
# 运行特定的基准测试
cargo bench vec_push
cargo bench std_sort
cargo bench hashmap_insert
```

### 保存和比较基线
```bash
# 保存当前结果作为基线
cargo bench -- --save-baseline main

# 与基线比较
cargo bench -- --baseline main

# 列出所有基线
cargo bench -- --list-baselines
```

### 其他选项
```bash
# 不运行基准测试，只列出所有测试
cargo bench -- --list

# 运行基准测试并生成 HTML 报告
cargo bench -- --plotting-backend plotters

# 设置样本数量
cargo bench -- --sample-size 200

# 设置测量时间
cargo bench -- --measurement-time 10

# 禁用输出
cargo bench -- --noplot

# 只运行一次（用于快速测试）
cargo bench -- --test
```

## 📈 理解结果

基准测试输出示例：
```
data_structures/vec_push  time:   [2.3456 µs 2.3891 µs 2.4321 µs]
                        change: [-5.234% -3.456% -1.678%] (p = 0.01 < 0.05)
                        Performance has improved.
```

- **time**: 执行时间的范围（最小、平均、最大）
- **change**: 与基线相比的变化百分比
- **p**: 统计显著性（p < 0.05 表示显著）

## 📊 性能优化建议

### Vec 操作
- 如果知道大致大小，使用 `with_capacity` 预分配
- 避免频繁的重新分配

### 集合选择
- **HashMap**: 平均 O(1) 查找，适合大多数场景
- **BTreeMap**: O(log n) 查找，保持键有序
- **Vec**: 适合小数据集或需要保持插入顺序

### 字符串处理
- 频繁拼接时使用 `with_capacity`
- 大量拼接时考虑使用 `join` 或 `write!` 宏

### 并发
- 读多写少场景使用 `RwLock`
- 写多读少场景使用 `Mutex`
- 考虑使用无锁数据结构（如 `crossbeam`）

## 🔧 配置

基准测试配置在 `benches/mod.rs` 中：

```rust
criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))  // 预热时间
        .measurement_time(Duration::from_secs(3))  // 测量时间
        .sample_size(100);  // 样本数量
    targets = ...
);
```

## 📚 学习资源

- [Criterion.rs 文档](https://bheisler.github.io/criterion.rs/book/)
- [Rust 性能优化](https://nnethercote.github.io/perf-book/)
- [Rust 基准测试指南](https://bencher.dev/learn/benchmarking/rust/bench/)

## 📝 添加新的基准测试

1. 在 `benches/mod.rs` 中找到合适的类别
2. 添加新的基准测试函数：

```rust
group.bench_function("my_benchmark", |b| {
    // 设置代码
    let data = prepare_data();
    
    b.iter(|| {
        // 被测试的代码
        operation(&data)
    });
});
```

3. 运行测试验证：
```bash
cargo bench my_benchmark
```

## 🐛 故障排除

### 基准测试运行太慢
- 减少 `measurement_time`
- 减少 `sample_size`
- 使用 `--test` 选项快速验证

### 结果不稳定
- 增加 `warm_up_time`
- 关闭其他应用程序
- 使用 `--baseline` 进行比较

### 编译错误
- 确保 `Cargo.toml` 中的 `criterion` 依赖正确
- 检查 `[[bench]]` 配置

## 📄 报告位置

基准测试报告生成在 `target/criterion/` 目录：

```
target/criterion/
├── data_structures/
│   ├── vec_push/
│   │   ├── new/
│   │   │   ├── index.html
│   │   │   ├── raw.csv
│   │   │   └── sample.json
│   │   └── report/
│   └── ...
├── algorithms/
└── ...
```

打开 `index.html` 可以查看详细的可视化报告。

---

**最后更新**: 2026-02-23
