# Rust 学习项目 - 基准测试套件

这个目录包含项目的 Criterion 基准测试套件，用于测量和比较各种 Rust 操作与示例场景的性能。

## 基本结构说明

当前基准测试在 [`Cargo.toml`](Cargo.toml:47) 中只注册了 **一个** bench target：[`mod`](Cargo.toml:48)。实际内容集中定义在 [`benches/mod.rs`](benches/mod.rs) 中，并通过 [`criterion_group!`](benches/mod.rs:37) 把多个 benchmark group 统一注册到同一个 target 下。

这意味着：

- `cargo bench` 会运行同一个 bench target 中的全部 group
- `cargo bench <filter>` 的过滤语义是**按名称匹配 benchmark / group**，而不是切换到多个独立的 bench target
- 像 `data_structures`、`algorithms`、`collections`、`strings`、`concurrency`、`projects` 这些名称，实际对应的是 [`c.benchmark_group()`](benches/mod.rs:57) / [`c.benchmark_group()`](benches/mod.rs:128) / [`c.benchmark_group()`](benches/mod.rs:236) / [`c.benchmark_group()`](benches/mod.rs:324) / [`c.benchmark_group()`](benches/mod.rs:384) / [`c.benchmark_group()`](benches/mod.rs:489) 创建的 group 名称

## 基准测试类别

### 1. 数据结构基准测试：`data_structures`

对应 [`data_structure_benchmarks()`](benches/mod.rs:56)。主要注册名包括：

- `vec_push`
- `vec_with_capacity`
- `vec_insert/<size>`
- `stack_operations`
- `queue_operations`

### 2. 算法基准测试：`algorithms`

对应 [`algorithm_benchmarks()`](benches/mod.rs:127)。主要注册名包括：

- `std_sort/<size>`
- `std_sort_unstable/<size>`
- `linear_search`
- `binary_search`
- `search_linear/<size>`
- `search_binary/<size>`
- `fibonacci_recursive`
- `fibonacci_iterative`

### 3. 集合操作基准测试：`collections`

对应 [`collection_benchmarks()`](benches/mod.rs:235)。主要注册名包括：

- `hashmap_insert/<size>`
- `btreemap_insert/<size>`
- `hashmap_lookup/<size>`
- `btreemap_lookup/<size>`
- `hashset_operations`
- `vec_random_access`

### 4. 字符串处理基准测试：`strings`

对应 [`string_benchmarks()`](benches/mod.rs:323)。主要注册名包括：

- `string_push_str`
- `string_join`
- `string_with_capacity`
- `string_find`
- `string_contains`
- `string_split`

### 5. 并发性能基准测试：`concurrency`

对应 [`concurrency_benchmarks()`](benches/mod.rs:383)。主要注册名包括：

- `single_threaded_sum`
- `multi_threaded_sum`
- `mutex_contention`
- `rwlock_read_heavy`
- `channel_throughput`

### 6. 实战项目基准测试：`projects`

对应 [`project_benchmarks()`](benches/mod.rs:488)。主要注册名包括：

- `task_manager_add`
- `task_manager_list`
- `task_manager_search`
- `json_serialize`
- `json_deserialize`

## 运行基准测试

### 运行全部基准

```bash
cargo bench
```

### 按 group 名称过滤

下面这些命令会在同一个 bench target 中，按名称过滤出对应 group：

```bash
cargo bench data_structures
cargo bench algorithms
cargo bench collections
cargo bench strings
cargo bench concurrency
cargo bench projects
```

### 按 benchmark 名称过滤

下面这些命令会匹配具体 benchmark 名称；如果该名称出现在某个 group 下，就会只运行匹配项：

```bash
cargo bench vec_push
cargo bench vec_with_capacity
cargo bench std_sort
cargo bench hashmap_insert
cargo bench task_manager_search
cargo bench json_deserialize
```

对于带输入规模的 benchmark，Criterion 最终显示的名称通常类似：

- `data_structures/vec_insert/1000`
- `algorithms/std_sort/10000`
- `collections/hashmap_lookup/100`

因此也可以使用更细粒度的过滤关键字，例如：

```bash
cargo bench vec_insert
cargo bench std_sort_unstable
cargo bench hashmap_lookup
cargo bench search_binary
```

## 保存和比较基线

```bash
# 保存当前结果作为基线
cargo bench -- --save-baseline main

# 与基线比较
cargo bench -- --baseline main

# 列出所有基线
cargo bench -- --list-baselines
```

## 其他常用选项

```bash
# 列出 Criterion 识别到的 benchmark 名称
cargo bench -- --list

# 生成 HTML 报告（项目已启用 html_reports）
cargo bench

# 设置样本数量
cargo bench -- --sample-size 200

# 设置测量时间（单位：秒）
cargo bench -- --measurement-time 10

# 快速验证一次
cargo bench -- --test
```

> 说明：当前 [`criterion`](Cargo.toml:45) 已启用 `html_reports`，常规运行后即可在 `target/criterion/` 中查看报告。

## 理解结果

典型输出示例：

```text
data_structures/vec_push
                        time:   [2.3456 µs 2.3891 µs 2.4321 µs]
                        change: [-5.234% -3.456% -1.678%] (p = 0.01 < 0.05)
                        Performance has improved.
```

- `time`：执行时间区间（通常可理解为统计后的范围）
- `change`：与基线相比的变化百分比
- `p`：统计显著性；`p < 0.05` 常表示变化具有统计意义

## 性能观察提示

### Vec 与字符串

- 若已知容量上界，`Vec::with_capacity` / `String::with_capacity` 往往更稳
- 插入与拼接类 benchmark 往往会明显体现预分配带来的差异

### 集合选择

- `HashMap` 常适合平均 O(1) 查找场景
- `BTreeMap` 适合需要有序键或范围查询的场景
- `Vec` 的随机访问通常非常快，但不适合替代所有映射结构

### 并发测试

- 并发组里的结果很容易受机器核数、调度与后台负载影响
- `mutex_contention`、`rwlock_read_heavy`、`channel_throughput` 更适合理解同步模式差异，而不是直接当成生产环境结论

## 配置位置

基准测试配置位于 [`criterion_group!`](benches/mod.rs:37)：

```rust
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
```

当前默认配置为：

- 预热时间：1 秒
- 测量时间：3 秒
- 样本数量：100

## 添加新的基准测试

1. 在 [`benches/mod.rs`](benches/mod.rs) 中找到合适的 group 函数
2. 在对应的 [`group.bench_function()`](benches/mod.rs:60) 或 [`group.bench_with_input()`](benches/mod.rs:82) 调用附近追加 benchmark
3. 使用名称过滤验证新增项是否能被匹配

示例：

```rust
group.bench_function("my_benchmark", |b| {
    let data = prepare_data();

    b.iter(|| {
        operation(&data)
    });
});
```

验证方式：

```bash
cargo bench my_benchmark
```

## 故障排除

### 基准测试运行太慢

- 调低 `measurement_time`
- 调低 `sample_size`
- 先用 `cargo bench -- --test` 做快速验证

### 结果不稳定

- 关闭其他高负载程序
- 多次重复运行做对比
- 使用 `--baseline` 与已保存结果比较

### 编译或识别问题

- 确认 [`Cargo.toml`](Cargo.toml) 中存在 [`[[bench]]`](Cargo.toml:47) 配置
- 确认当前只有一个 bench target：[`mod`](Cargo.toml:48)
- 如果过滤无结果，优先用 `cargo bench -- --list` 查看实际 benchmark 名称

## 报告位置

基准测试报告会生成在 `target/criterion/` 目录，结构通常与 group / benchmark 名称对应，例如：

```text
target/criterion/
├── data_structures/
│   ├── vec_push/
│   ├── vec_insert/
│   └── ...
├── algorithms/
├── collections/
├── strings/
├── concurrency/
└── projects/
```

打开其中的 HTML 报告即可查看更详细的图表和对比结果。

---

**最后更新**: 2026-03-15
