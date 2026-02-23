# Rust 学习项目文档索引

## 📚 项目概述

这是一个全面的 Rust 学习项目，包含 15 个模块，覆盖从基础到高级的 Rust 编程概念。每个模块都有详细的文档、示例代码和练习题。

## 🗂️ 模块文档

### 基础模块

| 模块 | 文档 | 描述 |
|------|------|------|
| 基础语法 | [basics.md](basics.md) | 变量、函数、控制流、数据结构 |
| 所有权系统 | [ownership.md](ownership.md) | 所有权、借用、生命周期、智能指针 |
| 类型系统 | [types.md](types.md) | 结构体、枚举、特征、泛型 |
| 错误处理 | [error_handling.md](error_handling.md) | Result、Option、自定义错误类型 |

### 进阶模块

| 模块 | 文档 | 描述 |
|------|------|------|
| 并发编程 | [concurrency.md](concurrency.md) | 线程、消息传递、共享状态 |
| 宏系统 | [macros.md](macros.md) | 声明宏、过程宏、DSL |
| 测试 | [testing.md](testing.md) | 单元测试、集成测试、TDD |
| 高级模式 | [advanced_patterns.md](advanced_patterns.md) | 设计模式、架构模式 |

### 实战模块

| 模块 | 文档 | 描述 |
|------|------|------|
| 安全编程 | [security.md](security.md) | 密码学、安全数据处理 |
| 最佳实践 | [best_practices.md](best_practices.md) | 代码风格、API 设计 |
| 常见陷阱 | [pitfalls.md](pitfalls.md) | 常见错误和解决方案 |

## 🚀 快速开始

### 运行项目

```bash
# 运行所有模块
cargo run

# 运行特定模块
cargo run basics
cargo run ownership
cargo run types

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### 项目结构

```
rust_learn/
├── Cargo.toml              # 项目配置
├── src/
│   ├── lib.rs              # 库入口
│   ├── main.rs             # 主程序
│   ├── basics.rs           # 基础语法
│   ├── ownership.rs        # 所有权系统
│   ├── types.rs            # 类型系统
│   ├── error_handling.rs   # 错误处理
│   ├── concurrency.rs      # 并发编程
│   ├── macros.rs           # 宏系统
│   ├── testing.rs          # 测试
│   ├── advanced_patterns.rs # 高级模式
│   ├── security.rs         # 安全编程
│   ├── best_practices.rs   # 最佳实践
│   └── pitfalls.rs         # 常见陷阱
├── tests/
│   └── integration_test.rs # 集成测试
└── docs/
    ├── README.md           # 本文档
    ├── basics.md
    ├── ownership.md
    ├── types.md
    ├── error_handling.md
    ├── concurrency.md
    ├── macros.md
    ├── testing.md
    ├── advanced_patterns.md
    ├── security.md
    ├── best_practices.md
    └── pitfalls.md
```

## 📖 学习路径

### 初学者路径

```
basics → ownership → types → error_handling
```

1. **基础语法** - 学习 Rust 的基本语法和概念
2. **所有权系统** - 理解 Rust 的核心特性
3. **类型系统** - 掌握结构体、枚举和特征
4. **错误处理** - 学会正确处理错误

### 进阶路径

```
concurrency → macros → testing → advanced_patterns
```

1. **并发编程** - 学习多线程和异步编程
2. **宏系统** - 掌握元编程技术
3. **测试** - 编写高质量的测试
4. **高级模式** - 应用设计模式

### 实战路径

```
security → best_practices → pitfalls
```

1. **安全编程** - 编写安全的代码
2. **最佳实践** - 遵循社区规范
3. **常见陷阱** - 避免常见错误

## 🛠️ 开发工具

### 推荐工具

- **rustfmt** - 代码格式化
- **clippy** - 代码检查
- **rust-analyzer** - IDE 支持
- **cargo-expand** - 宏展开
- **cargo-watch** - 自动编译

### 常用命令

```bash
# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 运行测试
cargo test

# 生成文档
cargo doc --open

# 检查编译
cargo check

# 发布构建
cargo build --release
```

## 📚 推荐资源

### 官方资源

- [Rust 官网](https://www.rust-lang.org/)
- [Rust 官方文档](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库](https://doc.rust-lang.org/std/)

### 学习资源

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust 设计模式](https://rust-unofficial.github.io/patterns/)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)

### 社区资源

- [Rust 用户论坛](https://users.rust-lang.org/)
- [Rust Reddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

## 🤝 贡献指南

欢迎贡献代码、报告问题或提出建议！

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](../LICENSE) 文件。

## 📊 学习进度追踪

使用以下清单追踪你的学习进度：

- [ ] 基础语法 (basics)
- [ ] 所有权系统 (ownership)
- [ ] 类型系统 (types)
- [ ] 错误处理 (error_handling)
- [ ] 并发编程 (concurrency)
- [ ] 宏系统 (macros)
- [ ] 测试 (testing)
- [ ] 高级模式 (advanced_patterns)
- [ ] 安全编程 (security)
- [ ] 最佳实践 (best_practices)
- [ ] 常见陷阱 (pitfalls)

---

**Happy Coding with Rust! 🦀**
