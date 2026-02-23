# 高级设计模式模块 (advanced_patterns)

## 📖 模块概述

本模块介绍 Rust 中常用的高级设计模式和架构模式，帮助开发者编写更加优雅、可维护的代码。

## 🎯 学习目标

- 掌握构建器模式
- 理解策略模式和命令模式
- 学会使用访问者模式
- 掌握工厂模式及其变体
- 理解 RAII 模式

## 📚 内容目录

### 1. 构建器模式 (Builder Pattern)

```rust
// 基本构建器
pub struct RequestBuilder {
    url: String,
    method: Method,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: Method::GET,
            headers: HashMap::new(),
            body: None,
        }
    }
    
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }
    
    pub fn build(self) -> Result<Request, String> {
        if self.url.is_empty() {
            return Err("URL 不能为空".to_string());
        }
        Ok(Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        })
    }
}

// 使用
let request = RequestBuilder::new("https://api.example.com")
    .method(Method::POST)
    .header("Content-Type", "application/json")
    .body(r#"{"name": "test"}"#)
    .build()?;
```

### 2. 类型状态构建器

```rust
// 使用类型状态模式
struct UrlNotSet;
struct UrlSet;

struct RequestBuilder<State> {
    url: Option<String>,
    _state: PhantomData<State>,
}

impl RequestBuilder<UrlNotSet> {
    pub fn new() -> Self {
        Self {
            url: None,
            _state: PhantomData,
        }
    }
    
    pub fn url(self, url: &str) -> RequestBuilder<UrlSet> {
        RequestBuilder {
            url: Some(url.to_string()),
            _state: PhantomData,
        }
    }
}

impl RequestBuilder<UrlSet> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.unwrap(),
        }
    }
}

// 编译时保证 URL 已设置
let request = RequestBuilder::new()
    .url("https://example.com")  // 必须调用
    .build();
```

### 3. 策略模式 (Strategy Pattern)

```rust
// 排序策略
trait SortStrategy {
    fn sort(&self, data: &mut [i32]);
}

struct BubbleSort;
impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        // 冒泡排序实现
    }
}

struct QuickSort;
impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        // 快速排序实现
    }
}

// 上下文
struct Sorter {
    strategy: Box<dyn SortStrategy>,
}

impl Sorter {
    fn new(strategy: Box<dyn SortStrategy>) -> Self {
        Self { strategy }
    }
    
    fn sort(&self, data: &mut [i32]) {
        self.strategy.sort(data);
    }
}

// 使用
let sorter = Sorter::new(Box::new(QuickSort));
sorter.sort(&mut data);
```

### 4. 命令模式 (Command Pattern)

```rust
trait Command {
    fn execute(&self);
    fn undo(&self);
}

struct AddTextCommand {
    document: Rc<RefCell<Document>>,
    text: String,
}

impl Command for AddTextCommand {
    fn execute(&self) {
        self.document.borrow_mut().add_text(&self.text);
    }
    
    fn undo(&self) {
        self.document.borrow_mut().remove_text(&self.text);
    }
}

struct CommandManager {
    history: Vec<Box<dyn Command>>,
}

impl CommandManager {
    fn execute(&mut self, command: Box<dyn Command>) {
        command.execute();
        self.history.push(command);
    }
    
    fn undo(&mut self) {
        if let Some(command) = self.history.pop() {
            command.undo();
        }
    }
}
```

### 5. 访问者模式 (Visitor Pattern)

```rust
// AST 节点
trait AstNode {
    fn accept(&self, visitor: &dyn Visitor);
}

struct NumberNode(i32);
struct AddNode {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

// 访问者
trait Visitor {
    fn visit_number(&self, node: &NumberNode);
    fn visit_add(&self, node: &AddNode);
}

struct Evaluator {
    result: i32,
}

impl Visitor for Evaluator {
    fn visit_number(&self, node: &NumberNode) {
        self.result = node.0;
    }
    
    fn visit_add(&self, node: &AddNode) {
        node.left.accept(self);
        let left = self.result;
        node.right.accept(self);
        let right = self.result;
        self.result = left + right;
    }
}
```

### 6. 工厂模式 (Factory Pattern)

```rust
// 简单工厂
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> String { "汪汪".to_string() }
}

impl Animal for Cat {
    fn speak(&self) -> String { "喵喵".to_string() }
}

fn create_animal(kind: &str) -> Box<dyn Animal> {
    match kind {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => panic!("未知动物类型"),
    }
}

// 抽象工厂
trait AnimalFactory {
    fn create_dog(&self) -> Box<dyn Animal>;
    fn create_cat(&self) -> Box<dyn Animal>;
}

struct ConcreteFactory;

impl AnimalFactory for ConcreteFactory {
    fn create_dog(&self) -> Box<dyn Animal> {
        Box::new(Dog)
    }
    
    fn create_cat(&self) -> Box<dyn Animal> {
        Box::new(Cat)
    }
}
```

### 7. RAII 模式

```rust
// 资源获取即初始化
struct FileGuard {
    file: File,
}

impl FileGuard {
    fn open(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self { file })
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        // 自动清理资源
        let _ = self.file.sync_all();
    }
}

// 使用 - 离开作用域自动关闭
{
    let guard = FileGuard::open("test.txt")?;
    // 使用 guard...
}  // 自动调用 Drop

// 锁守卫
struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // 自动释放锁
    }
}
```

### 8. 观察者模式 (Observer Pattern)

```rust
trait Observer {
    fn update(&self, message: &str);
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    
    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

struct LoggerObserver;

impl Observer for LoggerObserver {
    fn update(&self, message: &str) {
        println!("日志: {}", message);
    }
}

// 使用
let mut subject = Subject::new();
subject.attach(Box::new(LoggerObserver));
subject.notify("事件发生");
```

## 🚀 运行示例

```bash
# 运行高级模式模块
cargo run advanced_patterns
```

## 📊 设计模式分类

```
┌─────────────────────────────────────────────────────────────┐
│                     设计模式分类                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  创建型模式                    行为型模式                    │
│  ┌─────────────┐              ┌─────────────┐              │
│  │ 工厂模式    │              │ 策略模式    │              │
│  │ 构建器模式  │              │ 命令模式    │              │
│  │ 单例模式    │              │ 观察者模式  │              │
│  │ 原型模式    │              │ 访问者模式  │              │
│  └─────────────┘              └─────────────┘              │
│                                                             │
│  结构型模式                    Rust 特有模式                 │
│  ┌─────────────┐              ┌─────────────┐              │
│  │ 适配器模式  │              │ RAII 模式   │              │
│  │ 装饰器模式  │              │ 类型状态    │              │
│  │ 组合模式    │              │ NewType     │              │
│  │ 代理模式    │              │ 借用模式    │              │
│  └─────────────┘              └─────────────┘              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 实现一个简单的构建器模式
2. 使用策略模式实现不同的排序算法
3. 实现一个简单的工厂模式

### 中级
1. 实现类型状态构建器
2. 实现命令模式支持撤销/重做
3. 实现观察者模式的事件系统

### 高级
1. 实现完整的访问者模式处理 AST
2. 设计一个插件系统使用工厂模式
3. 实现一个响应式系统使用观察者模式

## 🔗 相关资源

- [Rust 设计模式](https://rust-unofficial.github.io/patterns/)
- [Effective Rust](https://www.lurklurk.org/effective-rust/)
- [Rust API 指南](https://rust-lang.github.io/api-guidelines/)

## ⚠️ 常见陷阱

### 1. 过度设计
```rust
// ❌ 简单问题复杂化
trait SimpleAdder {
    type Input;
    type Output;
    fn add(&self, a: Self::Input, b: Self::Input) -> Self::Output;
}

// ✅ 保持简单
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 2. 忽略 Rust 特性
```rust
// ❌ 模仿 OOP 语言
struct Singleton {
    instance: Option<Box<Self>>,
}

// ✅ 使用 Rust 特性
static INSTANCE: OnceCell<MyType> = OnceCell::new();
```

## 📊 学习检查清单

- [ ] 掌握构建器模式
- [ ] 理解类型状态模式
- [ ] 会使用策略模式
- [ ] 掌握命令模式
- [ ] 理解访问者模式
- [ ] 会使用工厂模式
- [ ] 理解 RAII 模式
- [ ] 掌握观察者模式
