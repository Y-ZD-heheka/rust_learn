# Rust学习项目

这是一个全面的Rust编程语言学习项目，按照系统化的学习路径组织代码示例和概念演示。

## 项目概述

本项目基于以下Rust学习大纲设计：
1. 基础语法
2. 所有权系统
3. 类型系统
4. 错误处理
5. 并发编程
6. 模块和包管理
7. 宏和元编程
8. 高级类型和生命周期
9. 测试和文档
10. 生态系统和工具

## 项目结构

```
rust_learn/
├── Cargo.toml          # 项目配置文件
├── src/
│   ├── main.rs         # 主程序入口
│   ├── lib.rs          # 库入口文件
│   ├── basics.rs       # 基础语法示例
│   ├── ownership.rs    # 所有权系统示例
│   ├── types.rs        # 类型系统示例
│   ├── error_handling.rs # 错误处理示例
│   ├── concurrency.rs  # 并发编程示例
│   ├── modules.rs      # 模块和包管理示例
│   ├── macros.rs       # 宏和元编程示例
│   ├── advanced_types.rs # 高级类型和生命周期示例
│   ├── testing.rs      # 测试和文档示例
│   └── ecosystem.rs    # 生态系统和工具示例
└── tests/
    └── integration_test.rs # 集成测试
```

## 快速开始

### 环境要求

- Rust 1.70.0 或更高版本
- Cargo（随Rust安装自动包含）

### 安装和运行

1. 克隆或下载项目
2. 进入项目目录
3. 运行所有示例：
   ```bash
   cargo run
   ```
4. 运行特定模块示例：
   ```bash
   cargo run <module_name>
   ```

### 可用模块

- `basics` - 基础语法
- `ownership` - 所有权系统
- `types` - 类型系统
- `error_handling` - 错误处理
- `concurrency` - 并发编程
- `modules` - 模块和包管理
- `macros` - 宏和元编程
- `advanced_types` - 高级类型和生命周期
- `testing` - 测试和文档
- `ecosystem` - 生态系统和工具

## 学习路径

建议按照以下顺序学习各个模块：

1. **基础语法** (`cargo run basics`)
   - 变量和类型
   - 函数定义
   - 控制流

2. **所有权系统** (`cargo run ownership`)
   - 所有权规则
   - 借用和引用
   - 切片

3. **类型系统** (`cargo run types`)
   - 结构体和枚举
   - 特征（Traits）
   - 泛型

4. **错误处理** (`cargo run error_handling`)
   - panic! 宏
   - Result 和 Option
   - 自定义错误类型

5. **并发编程** (`cargo run concurrency`)
   - 线程创建
   - 消息传递
   - 共享状态

6. **模块和包管理** (`cargo run modules`)
   - 模块定义
   - 包结构
   - 可见性

7. **宏和元编程** (`cargo run macros`)
   - 声明宏
   - 过程宏
   - 宏的卫生性

8. **高级类型和生命周期** (`cargo run advanced_types`)
   - 关联类型
   - 生命周期注解
   - 高级特征

9. **测试和文档** (`cargo run testing`)
   - 单元测试
   - 集成测试
   - 文档测试

10. **生态系统和工具** (`cargo run ecosystem`)
    - Cargo 工具链
    - crates.io 生态
    - 开发工具

## 测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test test_name
```

运行文档测试：
```bash
cargo test --doc
```

## 文档

生成项目文档：
```bash
cargo doc --open
```

## 开发工具

- **Cargo** - 包管理器和构建工具
- **rustc** - Rust编译器
- **rustup** - 工具链管理器
- **Clippy** - 代码检查工具
- **Rustfmt** - 代码格式化工具

## 贡献

欢迎提交问题和改进建议！

## 许可证

本项目仅用于学习目的。