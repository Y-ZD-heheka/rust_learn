# Rust 学习项目

一个面向学习者的 Rust 示例仓库，当前 crate 版本为 `0.1.0`，使用 Rust 2024 Edition。仓库目标不是把所有源码都包装成同一种 CLI 体验，而是提供一条从基础语法到工程实践、再到项目示例的学习路径。

当前阅读与使用时，建议把 [`src/main.rs`](src/main.rs) 视为 CLI 状态的唯一事实来源；其中 [`LearningStage`](src/main.rs:15)、[`MODULE_REGISTRY`](src/main.rs:68) 与 [`print_usage()`](src/main.rs:454) 共同定义了当前主学习路径、模块分组和帮助输出。

## 你会学到什么

- Rust 基础语法、控制流与常见数据结构操作
- 所有权、借用、生命周期与类型系统
- 错误处理、模块组织、测试、宏与并发
- 高级类型、设计模式、生态工具与常用库实践
- 工程最佳实践、安全编程与常见陷阱
- 项目实战示例：任务管理器
- 可选专题：数据库集成（库中开放，但不纳入主 CLI）

## 快速开始

### 环境要求

- Rust 1.85+
- Cargo

### 常用命令

```bash
# 按推荐学习路径运行所有已接入 CLI 的模块
cargo run

# 运行特定模块
cargo run -- basics
cargo run -- testing
cargo run -- ecosystem
cargo run -- advanced_patterns
cargo run -- popular_libraries
cargo run -- security

# 查看 CLI 帮助与阶段分组
cargo run -- --help

# 查看阶段数量与模块分布
cargo run -- --performance

# 生成 rustdoc 文档
cargo doc --open
```

## 当前 CLI 状态概览

根据 [`MODULE_REGISTRY`](src/main.rs:68) 与 [`NON_CLI_MODULES`](src/main.rs:162)，当前仓库状态如下：

- 主学习 CLI 已接入 **15 个**模块，按 **4 个**学习阶段组织。
- [`database`](src/lib.rs:29) 已在库入口导出，可作为库模块阅读和复用；但它**没有**纳入主 CLI 注册表。
- [`projects`](src/lib.rs:28) 也已在库入口导出，其中任务管理器示例保留为独立项目化阅读入口，不并入主 CLI。
- [`ecosystem`](src/main.rs:106) 仍可通过 CLI 运行，但定位是“说明性导览模块”，不应当作与其它可验证源码等价的稳定工程 API。
- [`popular_libraries`](src/main.rs:142)、[`best_practices`](src/main.rs:112)、[`security`](src/main.rs:148)、[`pitfalls`](src/main.rs:154) 都已纳入当前 CLI，可直接运行。

## 推荐学习路线

以下分组与 [`LearningStage`](src/main.rs:15) 保持一致。

### 1. 基础主线

对应阶段标题与目标见 [`LearningStage::title()`](src/main.rs:23) 和 [`LearningStage::focus()`](src/main.rs:32)。建议顺序：

1. `basics`
2. `ownership`
3. `types`
4. `error_handling`

这一阶段先建立语法、所有权、类型系统与错误处理的核心心智模型。

### 2. 语言与工程进阶

1. `modules`
2. `testing`
3. `ecosystem`
4. `best_practices`

这一阶段对应当前 CLI 的工程化主线：模块组织、测试分层、工具链导览与编码规范。

### 3. 高级主题

1. `concurrency`
2. `macros`
3. `advanced_types`
4. `advanced_patterns`

这一阶段聚焦更复杂的 Rust 抽象能力与设计表达。

### 4. 实践专题

1. `popular_libraries`
2. `security`
3. `pitfalls`

最后用常用库、安全与反模式回看前面主题，把知识串成更接近真实工程的实践经验。

## 模块与源码入口

| 分组 | 模块 | 源码入口 | CLI 状态 | 说明 |
| --- | --- | --- | --- | --- |
| 基础主线 | `basics` | `src/basics.rs` | 已接入 | 基础语法与核心概念 |
| 基础主线 | `ownership` | `src/ownership.rs` | 已接入 | 所有权、借用、生命周期 |
| 基础主线 | `types` | `src/types.rs` | 已接入 | 类型系统、结构体、枚举、trait |
| 基础主线 | `error_handling` | `src/error_handling.rs` + `src/error_handling/` | 已接入 | 门面入口 + 分层教学子模块 |
| 语言与工程进阶 | `modules` | `src/modules.rs` | 已接入 | 模块系统与包管理 |
| 语言与工程进阶 | `testing` | `src/testing.rs` + `src/testing/` | 已接入 | 门面入口 + 文档测试/策略/性能主题 |
| 语言与工程进阶 | `ecosystem` | `src/ecosystem.rs` | 已接入 | 说明性生态导览模块 |
| 语言与工程进阶 | `best_practices` | `src/best_practices.rs` | 已接入 | 工程实践与编码规范 |
| 高级主题 | `concurrency` | `src/concurrency.rs` | 已接入 | 并发与异步 |
| 高级主题 | `macros` | `src/macros.rs` | 已接入 | 宏系统与元编程 |
| 高级主题 | `advanced_types` | `src/advanced_types.rs` | 已接入 | 高级类型与生命周期 |
| 高级主题 | `advanced_patterns` | `src/advanced_patterns.rs` | 已接入 | 进阶设计模式 |
| 实践专题 | `popular_libraries` | `src/popular_libraries.rs` | 已接入 | Serde、Clap、Reqwest 等常用库聚合演示 |
| 实践专题 | `security` | `src/security.rs` | 已接入 | 安全编程实践与输入验证 |
| 实践专题 | `pitfalls` | `src/pitfalls.rs` | 已接入 | 常见陷阱与规避方式 |
| 非主 CLI | `projects` | `src/projects/mod.rs` + `src/projects/task_manager/` | 不纳入主 CLI | 独立项目实战入口 |
| 非主 CLI | `database` | `src/database.rs` | 不纳入主 CLI | 已由 [`src/lib.rs`](src/lib.rs) 导出，当前作为可选数据库专题保留 |

## 文档与导航入口

当前仓库中真实存在且值得优先参考的文档入口只有以下几处：

- [`README.md`](README.md)：项目总览、学习路线、模块入口与导航说明
- [`benches/README.md`](benches/README.md)：Criterion 基准测试说明与运行方式
- [`docs/src_code_review_reassessment.md`](docs/src_code_review_reassessment.md)：`src/` 目录整改后的复评归档文档
- `cargo doc --open`：基于源码注释生成的 rustdoc 文档入口

当前仓库**没有**单独维护总文档索引页或专题文档目录，因此阅读顺序建议直接从本页进入，再按源码文件跳转。

## 项目结构速览

```text
.
├── Cargo.toml
├── README.md
├── benches/
│   ├── README.md
│   └── mod.rs
├── docs/
│   └── src_code_review_reassessment.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── basics.rs
│   ├── ownership.rs
│   ├── types.rs
│   ├── error_handling.rs
│   ├── error_handling/
│   ├── modules.rs
│   ├── concurrency.rs
│   ├── macros.rs
│   ├── testing.rs
│   ├── testing/
│   ├── advanced_types.rs
│   ├── advanced_patterns.rs
│   ├── ecosystem.rs
│   ├── popular_libraries.rs
│   ├── best_practices.rs
│   ├── security.rs
│   ├── pitfalls.rs
│   ├── database.rs
│   └── projects/
│       ├── mod.rs
│       └── task_manager/
└── tests/
```

几个关键入口：

- [`src/main.rs`](src/main.rs)：主学习 CLI、学习阶段定义、模块注册与帮助输出
- [`src/lib.rs`](src/lib.rs)：库导出面；当前导出 `projects` 与 `database`
- [`src/error_handling.rs`](src/error_handling.rs)：错误处理门面入口，子模块位于 `src/error_handling/`
- [`src/testing.rs`](src/testing.rs)：测试主题门面入口，子模块位于 `src/testing/`
- [`src/projects/task_manager/mod.rs`](src/projects/task_manager/mod.rs)：项目实战入口

## 关于 `projects` 与 `database`

### `projects`

[`projects`](src/lib.rs:28) 面向“项目实战”而不是单一知识点，因此保留为库中的专题入口。它更适合在完成主学习路径后，再从状态建模、持久化、终端交互和测试组织角度回看。

### `database`

[`database`](src/lib.rs:29) 当前已经导出为库模块，并且源码已统一到 SQLite + SQLx 教学路线；但按照 [`NON_CLI_MODULES`](src/main.rs:162) 的定义，它仍不纳入主学习 CLI。这样做是为了避免把数据库环境要求引入默认学习主路径。

## 依赖与主题概览

以下概览仅基于 [`Cargo.toml`](Cargo.toml) 当前已经声明的依赖：

- 异步与网络：`tokio`、`futures`、`tokio-stream`、`reqwest`
- 序列化与数据交换：`serde`、`serde_json`
- CLI、错误与日志：`clap`、`anyhow`、`thiserror`、`tracing`、`tracing-subscriber`
- 时间与标识：`chrono`、`uuid`
- 安全与编码：`rand`、`ring`、`sha2`、`hmac`、`subtle`、`base64`、`hex`、`regex`、`pbkdf2`
- 数据库：`sqlx`
- 项目示例支持：`colored`、`dirs`
- 测试与基准：`tempfile`、`proptest`、`criterion`

这些依赖与当前模块主题大体对应：主学习 CLI 关注语言与工程能力，项目示例与数据库专题则承担更接近真实工程的扩展内容。

## 学习建议

- 先用 `cargo run -- --help` 查看当前阶段分组，再选单个模块深入阅读
- 对已接入 CLI 的模块，优先“读源码 + 跑示例”结合学习
- 对 [`projects`](src/lib.rs:28) 与 [`database`](src/lib.rs:29) 这类未纳入主 CLI 的内容，优先直接阅读源码入口
- 若想理解基准测试组织方式，再配合阅读 [`benches/README.md`](benches/README.md) 与 [`benches/mod.rs`](benches/mod.rs)

---

这份仓库当前更适合作为“带可运行 CLI 的 Rust 学习索引 + 若干项目化专题示例”，而不是一套完全统一抽象层级的框架型代码库。
