# Rust 学习项目

一个面向学习者的 Rust 示例仓库，当前 crate 版本为 `0.1.0`，使用 Rust 2024 Edition。项目目标不是把所有源码都包装成同一种 CLI 体验，而是提供一条从基础语法到工程实践、再到项目实战的学习路径。

当前阅读与使用时，建议始终把 [`src/main.rs`](src/main.rs) 视为 CLI 状态的唯一事实来源，把 [`docs/README.md`](docs/README.md) 视为专题文档入口，把 [`MODULE_DOCUMENTATION.md`](MODULE_DOCUMENTATION.md) 视为源码地图。

## 你会学到什么

- Rust 基础语法、控制流与常见算法
- 所有权、借用、生命周期与类型系统
- 错误处理、模块组织、宏、测试与并发
- 高级类型设计、设计模式、生态系统与常用库
- 工程最佳实践、安全编程、常见陷阱
- 项目实战示例：任务管理器（带有 CLI 思维的终端案例）

## 先从哪里进入

如果你是第一次进入这个仓库，建议按下面顺序阅读：

1. 先看 [`docs/README.md`](docs/README.md)，了解文档入口和学习顺序
2. 再从 [`README.md`](README.md) 的学习导航矩阵选择主题
3. 对于已接入 CLI 的主题，优先一边阅读源码、一边使用 `cargo run -- <module>` 观察输出
4. 对于未纳入主学习 CLI 的内容，先读专题文档与源码，再决定是否深入扩展

## 当前 CLI 与模块状态

根据 [`src/main.rs`](src/main.rs) 当前的 [`MODULE_REGISTRY`](src/main.rs:26) 与 [`NON_CLI_MODULES`](src/main.rs:105)：

- 当前已接入主学习 CLI 的主题共有 15 个：`basics`、`ownership`、`types`、`error_handling`、`concurrency`、`modules`、`macros`、`advanced_types`、`testing`、`ecosystem`、`advanced_patterns`、`popular_libraries`、`best_practices`、`security`、`pitfalls`
- `popular_libraries` 已经是可直接运行的聚合演示入口，不再只是占位提示
- `best_practices`、`security`、`pitfalls` 已接入 CLI，可直接通过 `cargo run -- <module>` 运行
- `projects` 已在库入口导出，但当前不纳入主学习 CLI 注册表
- `database` 源码文件存在，但仍未在 [`src/lib.rs`](src/lib.rs) 中启用导出，当前保持禁用

## 快速开始

### 环境要求

- Rust 1.85+
- Cargo

### 常用命令

```bash
# 运行所有已接入主学习 CLI 的模块
cargo run

# 运行特定模块
cargo run -- basics
cargo run -- ownership
cargo run -- modules
cargo run -- advanced_types
cargo run -- popular_libraries
cargo run -- security

# 查看 CLI 帮助
cargo run -- --help

# 生成项目文档
cargo doc --open
```

> 说明：本仓库中并不是每个源码主题都在当前 CLI 中暴露为独立命令。请优先参考 [`docs/README.md`](docs/README.md) 中的文档索引和状态说明。

## 学习导航矩阵

下表用于回答 6 个问题：学什么、先学什么、需要什么前置知识、源码入口在哪里、有没有专题文档、能不能直接从 CLI 运行。

| 主题 | 推荐顺序 | 前置知识 | 源码入口 | 文档页 | CLI 可运行状态 |
|------|----------|----------|----------|--------|----------------|
| 基础语法 | 1 | 无 | `src/basics.rs` | `docs/basics.md` | 已接入 |
| 所有权 | 2 | 基础语法 | `src/ownership.rs` | 暂无独立专题页 | 已接入 |
| 类型系统 | 3 | 基础语法、所有权 | `src/types.rs` | `docs/types.md` | 已接入 |
| 错误处理 | 4 | 类型系统、Result 基础 | `src/error_handling.rs`（聚合入口） + `src/error_handling/`（教学分层） | `docs/error_handling.md` | 已接入 |
| 模块系统 | 5 | 基础语法、类型系统 | `src/modules.rs` | `docs/modules.md` | 已接入 |
| 并发编程 | 6 | 所有权、错误处理 | `src/concurrency.rs` | `docs/concurrency.md` | 已接入 |
| 宏系统 | 7 | 模块系统、模式匹配 | `src/macros.rs` | `docs/macros.md` | 已接入 |
| 测试 | 8 | 错误处理、模块组织 | `src/testing.rs`（聚合入口） + `src/testing/`（教学分层） | `docs/testing.md` | 已接入 |
| 高级类型 | 9 | 所有权、类型系统、trait | `src/advanced_types.rs` | `docs/advanced_types.md` | 已接入 |
| 设计模式 | 10 | 模块系统、trait、泛型 | `src/advanced_patterns.rs` | `docs/advanced_patterns.md` | 已接入 |
| 生态系统 | 11 | Cargo 基础、测试基础 | `src/ecosystem.rs` | `docs/ecosystem.md` | 已接入 |
| 常用库实践 | 12 | 生态系统、错误处理 | `src/popular_libraries.rs` | `docs/popular_libraries.md` | 已接入 |
| 最佳实践 | 13 | 主学习路径前 12 项 | `src/best_practices.rs` | 暂无独立专题页 | 已接入 |
| 安全编程 | 14 | 错误处理、常用库基础 | `src/security.rs` | `docs/security.md` | 已接入 |
| 常见陷阱 | 15 | 所有权、并发、类型系统 | `src/pitfalls.rs` | 暂无独立专题页 | 已接入 |
| 项目实战：任务管理器 | 16 | 主学习路径前 12 项 | `src/projects/task_manager/mod.rs` + `src/projects/task_manager/` | `docs/projects_task_manager.md` | 项目实战示例，不纳入主学习 CLI |
| 数据库专题 | 可选 | 生态系统、错误处理、异步基础 | `src/database.rs` | 暂无独立专题页 | 当前禁用 |

## 推荐学习路线

### 阶段一：建立 Rust 基础心智

1. `basics`
2. `ownership`
3. `types`
4. `error_handling`

这四个主题决定了你后面读模块、并发和高级类型时是否轻松。

### 阶段二：建立工程化理解

5. `modules`
6. `concurrency`
7. `macros`
8. `testing`

这一阶段重点不是记住所有 API，而是理解 Rust 项目如何组织、如何验证、如何扩展。

### 阶段三：进入高阶语言与生态主题

9. `advanced_types`
10. `advanced_patterns`
11. `ecosystem`
12. `popular_libraries`

这一阶段建议把“源码结构”和“CLI 输出”配合起来看，理解抽象设计和真实工程工具链如何配合。

### 阶段四：补齐工程专题与实战视角

13. `best_practices`
14. `security`
15. `pitfalls`
16. `projects/task_manager`

这些内容更适合在已经能读懂前面模块后再进入，否则容易只记结论、看不懂原因。

## 文档入口

### 文档总索引

- [`docs/README.md`](docs/README.md)：文档导航入口与完整学习矩阵
- [`MODULE_DOCUMENTATION.md`](MODULE_DOCUMENTATION.md)：源码主题总览与状态对照
- [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md)：仓库结构与入口关系
- [`IMPROVEMENTS.md`](IMPROVEMENTS.md)：本轮改进与后续待办记录

### 当前已提供的专题文档

- `docs/basics.md`
- `docs/error_handling.md`
- `docs/testing.md`
- `docs/security.md`
- `docs/types.md`
- `docs/concurrency.md`
- `docs/macros.md`
- `docs/advanced_patterns.md`
- `docs/modules.md`
- `docs/advanced_types.md`
- `docs/ecosystem.md`
- `docs/popular_libraries.md`
- `docs/projects_task_manager.md`

## 项目结构速览

```text
src/
├── main.rs                    # 主学习 CLI 入口与模块注册表
├── lib.rs                     # 库导出入口，database 当前仍注释禁用
├── basics.rs                  # 基础语法
├── ownership.rs               # 所有权与生命周期
├── types.rs                   # 类型系统
├── error_handling.rs          # 错误处理聚合入口
├── error_handling/
│   ├── fundamentals.rs        # panic / Result / ? 基础
│   ├── io_config.rs           # 文件 IO 与配置错误
│   ├── domain.rs              # 业务校验、恢复与日志
│   └── infrastructure.rs      # 资源加载与外部服务错误
├── modules.rs                 # 模块系统
├── concurrency.rs             # 并发与异步
├── macros.rs                  # 宏系统
├── testing.rs                 # 测试主题聚合入口
├── testing/
│   ├── domain.rs              # 被测对象与基础断言
│   ├── documentation.rs       # 文档测试与基础演示
│   ├── strategies.rs          # 属性测试、TDD、边界测试
│   └── performance.rs         # 性能测试与集成场景
├── advanced_types.rs          # 高级类型
├── advanced_patterns.rs       # 设计模式
├── ecosystem.rs               # 生态系统与工具链
├── popular_libraries.rs       # 常用库聚合演示
├── best_practices.rs          # 最佳实践
├── security.rs                # 安全编程
├── pitfalls.rs                # 常见陷阱
├── projects/
│   └── task_manager/          # 任务管理器项目实战示例（多文件拆分）
└── database.rs                # 数据库专题，当前禁用
```

## 关于 `projects` 与 `database`

### 为什么 `projects` 不在主学习 CLI 中

[`src/projects/mod.rs`](src/projects/mod.rs) 与 [`src/projects/task_manager/mod.rs`](src/projects/task_manager/mod.rs) 更像项目实战材料，而不是单一知识点示例。它适合在学习者完成主学习路径后，再从结构设计、状态建模、持久化语义和终端交互体验角度回看，而不是把它理解为已并入主学习路径的完整 CLI 工具。

### 为什么 `database` 当前不纳入主学习路径

[`src/database.rs`](src/database.rs) 的主题依赖数据库环境、额外依赖配置与更稳定的启用策略，而 [`src/lib.rs`](src/lib.rs) 当前仍将它保持为注释禁用状态。为了不让初学者在主路径中提前陷入环境配置问题，当前文档把它明确标记为“可选且禁用”。

## 学习建议

- 先用 [`docs/README.md`](docs/README.md) 选主题，再看对应源码文件
- 对已接入 CLI 的主题，优先使用 `cargo run -- <module>` 验证理解
- 对没有独立专题页的主题，先参考 [`MODULE_DOCUMENTATION.md`](MODULE_DOCUMENTATION.md) 的模块说明，再回到源码
- 对项目实战内容，先读 [`docs/projects_task_manager.md`](docs/projects_task_manager.md)，再从 [`src/projects/task_manager/mod.rs`](src/projects/task_manager/mod.rs) 跳转到各职责文件阅读

---

愿这份仓库既能作为 Rust 入门地图，也能作为后续工程化学习的索引。
