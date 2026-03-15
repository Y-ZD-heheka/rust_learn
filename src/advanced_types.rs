//! # 高级类型和生命周期模块
//!
//! 这个模块演示了Rust的高级类型系统和生命周期概念。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fmt::Display;

/// 现代化关联类型示例
pub trait ModernIterator {
    type Item: Display + Clone;

    fn next(&mut self) -> Option<Self::Item>;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

/// 现代化实现自定义迭代器
#[derive(Debug)]
pub struct ModernCounter {
    current: usize,
    max: usize,
}

impl ModernCounter {
    pub fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
}

impl ModernIterator for ModernCounter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.max.saturating_sub(self.current);
        (remaining, Some(remaining))
    }
}

/// 现代化关联类型使用
pub fn modern_associated_types() {
    println!("🔄 现代化关联类型：");
    
    let mut counter = ModernCounter::new(5);
    println!("初始大小提示（剩余元素数）: {:?}", counter.size_hint());
    while let Some(value) = counter.next() {
        println!("计数器: {}", value);
    }
    
    let hint = counter.size_hint();
    println!("遍历结束后的大小提示: {:?}", hint);
}

/// 现代化泛型类型参数
pub fn modern_generic_parameters() {
    println!("🔧 现代化泛型类型：");
    
    // 现代化泛型结构体
    #[derive(Debug, Clone)]
    struct ModernPoint<T: Display + Copy> {
        x: T,
        y: T,
    }
    
    impl<T: Display + Copy> ModernPoint<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
        
        pub fn distance_from_origin(&self) -> f64
        where
            T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Into<f64>,
        {
            let sum_squares = (self.x * self.x + self.y * self.y).into();
            sum_squares.sqrt()
        }
    }
    
    let integer_point = ModernPoint::new(5, 10);
    let float_point = ModernPoint::new(1.5, 2.5);
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    println!("整数点到原点距离: {}", integer_point.distance_from_origin());
    println!("浮点数点到原点距离: {:.2}", float_point.distance_from_origin());
    
    // 现代化泛型枚举
    #[derive(Debug)]
    enum ModernResult<T, E> {
        Ok(T),
        Err(E),
    }
    
    let success: ModernResult<i32, &str> = ModernResult::Ok(42);
    let failure: ModernResult<i32, &str> = ModernResult::Err("错误信息");
    
    println!("成功结果: {:?}", success);
    println!("失败结果: {:?}", failure);
    
    // 现代化泛型函数
    fn find_first<T: PartialEq>(items: &[T], target: &T) -> Option<usize> {
        items.iter().position(|item| item == target)
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    let words = vec!["hello", "world", "rust"];
    
    if let Some(index) = find_first(&numbers, &3) {
        println!("找到数字3在索引: {}", index);
    }
    
    if let Some(index) = find_first(&words, &"rust") {
        println!("找到字符串'rust'在索引: {}", index);
    }
}

/// 现代化生命周期系统
pub fn modern_lifetimes() {
    println!("⏰ 现代化生命周期：");
    
    // 现代化生命周期注解
    fn longest_with_info<'a>(
        x: &'a str,
        y: &'a str
    ) -> &'a str {
        println!("比较字符串: '{}' 和 '{}'", x, y);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("长的字符串");
    let string2 = "短的";
    
    let result = longest_with_info(&string1, string2);
    println!("最长字符串: {}", result);
    
    // 现代化结构体中的生命周期
    #[derive(Debug)]
    struct ModernExcerpt<'a> {
        part: &'a str,
        chapter: usize,
    }
    
    let novel = String::from("第一章 开始的故事...\n第二章 更多内容...\n第三章 结局");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ModernExcerpt {
        part: first_sentence,
        chapter: 1
    };
    println!("片段: {}", excerpt.part);
    println!("章节: {}", excerpt.chapter);
    
    // 现代化生命周期省略
    fn modern_first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    
    let s = String::from("hello world rust");
    let word = modern_first_word(&s);
    println!("第一个词: {}", word);
    
    // 静态生命周期
    static GLOBAL_MESSAGE: &str = "全局静态消息";
    println!("静态消息: {}", GLOBAL_MESSAGE);
}

/// 现代化高级特征系统
pub fn modern_trait_system() {
    println!("🎨 现代化特征系统：");
    
    // 现代化特征定义
    pub trait ModernDraw {
        type Color;
        type Style;
        
        fn draw(&self) -> String;
        #[allow(dead_code)]
        fn get_color(&self) -> Self::Color;
        #[allow(dead_code)]
        fn get_style(&self) -> Self::Style;
    }
    
    // 现代化特征实现
    #[derive(Debug)]
    pub struct ModernButton {
        pub width: u32,
        pub height: u32,
        pub label: String,
        pub color: String,
        pub style: String,
    }
    
    impl ModernDraw for ModernButton {
        type Color = String;
        type Style = String;
        
        fn draw(&self) -> String {
            format!("绘制按钮: {} ({}x{}, {}-{})",
                    self.label, self.width, self.height, self.color, self.style)
        }
        
        fn get_color(&self) -> Self::Color {
            self.color.clone()
        }
        
        fn get_style(&self) -> Self::Style {
            self.style.clone()
        }
    }
    
    #[derive(Debug)]
    pub struct ModernText {
        pub text: String,
        pub font_size: u32,
        pub color: String,
        pub style: String,
    }
    
    impl ModernDraw for ModernText {
        type Color = String;
        type Style = String;
        
        fn draw(&self) -> String {
            format!("绘制文本: '{}' (大小:{}, {}-{})",
                    self.text, self.font_size, self.color, self.style)
        }
        
        fn get_color(&self) -> Self::Color {
            self.color.clone()
        }
        
        fn get_style(&self) -> Self::Style {
            self.style.clone()
        }
    }
    
    // 现代化特征对象
    pub struct ModernScreen {
        pub components: Vec<Box<dyn ModernDraw<Color = String, Style = String>>>,
    }
    
    impl ModernScreen {
        pub fn new() -> Self {
            Self { components: Vec::new() }
        }
        
        pub fn add_component(&mut self, component: Box<dyn ModernDraw<Color = String, Style = String>>) {
            self.components.push(component);
        }
        
        pub fn render(&self) {
            println!("🎨 渲染屏幕组件:");
            for (i, component) in self.components.iter().enumerate() {
                println!("  {}: {}", i + 1, component.draw());
            }
        }
    }
    
    // 使用现代化特征系统
    let mut screen = ModernScreen::new();
    
    screen.add_component(Box::new(ModernButton {
        width: 100,
        height: 30,
        label: "确定".to_string(),
        color: "蓝色".to_string(),
        style: "圆角".to_string(),
    }));
    
    screen.add_component(Box::new(ModernText {
        text: "欢迎使用现代化Rust程序".to_string(),
        font_size: 16,
        color: "黑色".to_string(),
        style: "加粗".to_string(),
    }));
    
    screen.render();
}

/// 现代化泛型约束和特征对象
pub fn modern_generic_constraints() {
    println!("🔗 现代化泛型约束：");
    
    // 多个特征约束
    fn process_item<T>(item: &T) -> String
    where
        T: Clone + Display + PartialEq,
    {
        format!("处理项目: {} (克隆: {}, 相等: {})",
                item, item.clone(), true)
    }
    
    let text = "Hello Rust";
    println!("{}", process_item(&text));
    
    // 现代化关联类型和特征对象
    trait Container {
        type Item: Clone + Display;
        
        fn insert(&mut self, item: Self::Item);
        fn get(&self, index: usize) -> Option<&Self::Item>;
        fn len(&self) -> usize;
    }
    
    #[derive(Debug)]
    struct ModernVec<T> {
        items: Vec<T>,
    }
    
    impl<T: Clone + Display> ModernVec<T> {
        pub fn new() -> Self {
            Self { items: Vec::new() }
        }
    }
    
    impl<T: Clone + Display> Container for ModernVec<T> {
        type Item = T;
        
        fn insert(&mut self, item: Self::Item) {
            self.items.push(item);
        }
        
        fn get(&self, index: usize) -> Option<&Self::Item> {
            self.items.get(index)
        }
        
        fn len(&self) -> usize {
            self.items.len()
        }
    }
    
    let mut container = ModernVec::new();
    container.insert("项目1".to_string());
    container.insert("项目2".to_string());
    
    println!("容器长度: {}", container.len());
    if let Some(item) = container.get(0) {
        println!("第一个项目: {}", item);
    }
}

/// 现代化NewType模式
pub fn modern_newtype_pattern() {
    println!("📦 现代化NewType模式：");
    
    // NewType用于类型安全
    #[derive(Debug, Clone)]
    pub struct UserId(u64);
    
    #[derive(Debug, Clone)]
    pub struct SessionId(String);
    
    impl UserId {
        pub fn new(id: u64) -> Self {
            Self(id)
        }
        
        pub fn as_u64(&self) -> u64 {
            self.0
        }
    }
    
    impl SessionId {
        pub fn new(session: String) -> Self {
            Self(session)
        }
        
        pub fn as_str(&self) -> &str {
            &self.0
        }
    }
    
    // 类型安全的使用
    let user_id = UserId::new(12345);
    let session_id = SessionId::new("abc-def-ghi".to_string());
    
    println!("用户ID: {} ({})", user_id.as_u64(), user_id.0);
    println!("会话ID: {}", session_id.as_str());
    
    // 避免混淆
    fn process_user(user_id: UserId, session_id: SessionId) {
        println!("处理用户ID: {}, 会话ID: {}", user_id.as_u64(), session_id.as_str());
    }
    
    process_user(user_id, session_id);
}

/// 演示工厂模式
pub fn factory_pattern() {
    println!("🏭 工厂模式：");
    
    // 产品特征
    trait Product {
        fn operation(&self) -> String;
    }
    
    // 具体产品
    #[derive(Debug)]
    struct ConcreteProductA;
    
    impl Product for ConcreteProductA {
        fn operation(&self) -> String {
            "产品A的操作".to_string()
        }
    }
    
    #[derive(Debug)]
    struct ConcreteProductB;
    
    impl Product for ConcreteProductB {
        fn operation(&self) -> String {
            "产品B的操作".to_string()
        }
    }
    
    // 工厂特征
    trait ProductFactory {
        type Product: Product;
        
        fn create_product(&self) -> Self::Product;
    }
    
    // 具体工厂
    struct ConcreteFactoryA;
    
    impl ProductFactory for ConcreteFactoryA {
        type Product = ConcreteProductA;
        
        fn create_product(&self) -> Self::Product {
            ConcreteProductA
        }
    }
    
    struct ConcreteFactoryB;
    
    impl ProductFactory for ConcreteFactoryB {
        type Product = ConcreteProductB;
        
        fn create_product(&self) -> Self::Product {
            ConcreteProductB
        }
    }
    
    // 客户端代码
    let factories: Vec<Box<dyn ProductFactory<Product = ConcreteProductA>>> = vec![
        Box::new(ConcreteFactoryA) as Box<dyn ProductFactory<Product = ConcreteProductA>>,
        // 对于第二个工厂，我们需要使用不同的方法
    ];
    
    // 分别调用每个工厂
    let factory_a = Box::new(ConcreteFactoryA) as Box<dyn ProductFactory<Product = ConcreteProductA>>;
    let factory_b = Box::new(ConcreteFactoryB) as Box<dyn ProductFactory<Product = ConcreteProductB>>;
    
    // 测试工厂A
    let product_a = factory_a.create_product();
    println!("创建产品: {}", product_a.operation());
    
    // 测试工厂B
    let product_b = factory_b.create_product();
    println!("创建产品: {}", product_b.operation());
    
    // 使用向量分别存储不同类型的工厂
    let _factories_a = vec![
        Box::new(ConcreteFactoryA) as Box<dyn ProductFactory<Product = ConcreteProductA>>,
    ];
    
    let _factories_b = vec![
        Box::new(ConcreteFactoryB) as Box<dyn ProductFactory<Product = ConcreteProductB>>,
    ];
    
    for factory in factories {
        let product = factory.create_product();
        println!("创建产品: {}", product.operation());
    }
}

/// 演示策略模式
pub fn strategy_pattern() {
    println!("🎯 策略模式：");
    
    // 策略特征
    trait SortStrategy {
        fn sort(&self, data: &mut [i32]);
    }
    
    // 具体策略
    struct BubbleSort;
    
    impl SortStrategy for BubbleSort {
        fn sort(&self, data: &mut [i32]) {
            for i in 0..data.len() {
                for j in 0..data.len() - 1 - i {
                    if data[j] > data[j + 1] {
                        data.swap(j, j + 1);
                    }
                }
            }
        }
    }
    
    struct QuickSort;
    
    impl SortStrategy for QuickSort {
        fn sort(&self, data: &mut [i32]) {
            if data.len() <= 1 {
                return;
            }
            
            let pivot = data.len() / 2;
            let mut left = Vec::new();
            let mut right = Vec::new();
            
            for (i, &item) in data.iter().enumerate() {
                if i == pivot {
                    continue;
                }
                if item < data[pivot] {
                    left.push(item);
                } else {
                    right.push(item);
                }
            }
            
            self.sort(&mut left);
            self.sort(&mut right);
            
            let mut result = Vec::new();
            result.extend(left);
            result.push(data[pivot]);
            result.extend(right);
            
            data.copy_from_slice(&result);
        }
    }
    
    // 上下文
    struct SortContext {
        strategy: Box<dyn SortStrategy>,
    }
    
    impl SortContext {
        fn new(strategy: Box<dyn SortStrategy>) -> Self {
            Self { strategy }
        }
        
        fn execute_sort(&self, data: &mut [i32]) {
            println!("使用策略排序前的数据: {:?}", data);
            self.strategy.sort(data);
            println!("使用策略排序后的数据: {:?}", data);
        }
        
        #[allow(dead_code)]
        fn set_strategy(&mut self, strategy: Box<dyn SortStrategy>) {
            self.strategy = strategy;
        }
    }
    
    let data = vec![64, 34, 25, 12, 22, 11, 90];
    
    let bubble_sort = SortContext::new(Box::new(BubbleSort));
    bubble_sort.execute_sort(&mut data.clone());
    
    let quick_sort = SortContext::new(Box::new(QuickSort));
    quick_sort.execute_sort(&mut data.clone());
}

/// 演示装饰器模式
pub fn decorator_pattern() {
    println!("🎨 装饰器模式：");
    
    // 组件特征
    trait DataSource {
        fn write_data(&self, data: &str);
        #[allow(dead_code)]
        fn read_data(&self) -> String;
    }
    
    // 具体组件
    #[allow(dead_code)]
    struct FileDataSource {
        filename: String,
        #[allow(dead_code)]
        content: String,
    }
    
    impl FileDataSource {
        fn new(filename: String) -> Self {
            Self {
                filename,
                content: String::new(),
            }
        }
    }
    
    impl DataSource for FileDataSource {
        fn write_data(&self, data: &str) {
            println!("写入文件 {}: {}", self.filename, data);
        }
        
        fn read_data(&self) -> String {
            self.content.clone()
        }
    }
    
    // 装饰器基类
    #[allow(dead_code)]
    struct DataSourceDecorator {
        wrappee: Box<dyn DataSource>,
    }
    
    impl DataSourceDecorator {
        #[allow(dead_code)]
        fn new(source: Box<dyn DataSource>) -> Self {
            Self { wrappee: source }
        }
    }
    
    impl DataSource for DataSourceDecorator {
        fn write_data(&self, data: &str) {
            self.wrappee.write_data(data);
        }
        
        fn read_data(&self) -> String {
            self.wrappee.read_data()
        }
    }
    
    // 具体装饰器
    struct EncryptionDecorator {
        wrappee: Box<dyn DataSource>,
    }
    
    impl EncryptionDecorator {
        fn new(source: Box<dyn DataSource>) -> Self {
            Self { wrappee: source }
        }
        
        fn encrypt(&self, data: &str) -> String {
            // 这里使用 ROT13 作为“可逆变换”示例，便于演示装饰器链式处理。
            // 它不是安全加密算法，因此仅用于教学场景。
            data.chars()
                .map(|c| match c {
                    'a'..='z' => {
                        let offset = c as u8 - b'a';
                        ((offset + 13) % 26 + b'a') as char
                    }
                    'A'..='Z' => {
                        let offset = c as u8 - b'A';
                        ((offset + 13) % 26 + b'A') as char
                    }
                    _ => c,
                })
                .collect()
        }
        
        #[allow(dead_code)]
        fn decrypt(&self, data: &str) -> String {
            self.encrypt(data) // ROT13 对英文字母是对称的
        }
    }
    
    impl DataSource for EncryptionDecorator {
        fn write_data(&self, data: &str) {
            let encrypted = self.encrypt(data);
            println!("使用演示性的 ROT13 编码后写入（仅用于教学示例）");
            self.wrappee.write_data(&encrypted);
        }
        
        fn read_data(&self) -> String {
            let data = self.wrappee.read_data();
            let decrypted = self.decrypt(&data);
            println!("数据解密读取");
            decrypted
        }
    }
    
    struct CompressionDecorator {
        wrappee: Box<dyn DataSource>,
    }
    
    impl CompressionDecorator {
        fn new(source: Box<dyn DataSource>) -> Self {
            Self { wrappee: source }
        }
        
        fn compress(&self, data: &str) -> String {
            // 模拟压缩
            format!("[压缩] {}", data)
        }
        
        #[allow(dead_code)]
        fn decompress(&self, data: &str) -> String {
            // 模拟解压
            if data.starts_with("[压缩] ") {
                data.strip_prefix("[压缩] ").unwrap().to_string()
            } else {
                data.to_string()
            }
        }
    }
    
    impl DataSource for CompressionDecorator {
        fn write_data(&self, data: &str) {
            let compressed = self.compress(data);
            println!("压缩数据写入");
            self.wrappee.write_data(&compressed);
        }
        
        fn read_data(&self) -> String {
            let data = self.wrappee.read_data();
            self.decompress(&data)
        }
    }
    
    // 客户端代码
    let file_source = Box::new(FileDataSource::new("data.txt".to_string()));
    
    let encrypted_source = Box::new(EncryptionDecorator::new(file_source));
    encrypted_source.write_data("敏感数据");
    
    let compressed_encrypted_source = Box::new(CompressionDecorator::new(encrypted_source));
    compressed_encrypted_source.write_data("重要数据");
}

/// 演示观察者模式
pub fn observer_pattern() {
    println!("👀 观察者模式：");
    
    use std::collections::HashMap;
    
    // 观察者特征
    trait Observer {
        fn update(&self, event: &str, data: &str);
    }
    
    #[derive(Debug)]
    struct ConcreteObserver {
        id: u32,
        name: String,
    }
    
    impl Observer for ConcreteObserver {
        fn update(&self, event: &str, data: &str) {
            println!("观察者 {} ({}) 收到通知 - 事件: {}, 数据: {}",
                     self.id, self.name, event, data);
        }
    }
    
    // 主题特征
    trait Subject {
        fn attach(&mut self, observer: Box<dyn Observer>);
        fn detach(&mut self, observer_id: u32);
        fn notify(&self, event: &str, data: &str);
    }
    
    // 具体主题
    struct NewsAgency {
        observers: HashMap<u32, Box<dyn Observer>>,
        next_id: u32,
    }
    
    impl NewsAgency {
        fn new() -> Self {
            Self {
                observers: HashMap::new(),
                next_id: 1,
            }
        }
        
        fn publish_news(&self, headline: String, content: String) {
            println!("📰 发布新闻: {}", headline);
            self.notify("news_published", &format!("{}: {}", headline, content));
        }
    }
    
    impl Subject for NewsAgency {
        fn attach(&mut self, observer: Box<dyn Observer>) {
            let id = self.next_id;
            self.observers.insert(id, observer);
            self.next_id += 1;
            println!("✅ 新的观察者已注册，ID: {}", id);
        }
        
        fn detach(&mut self, observer_id: u32) {
            if self.observers.remove(&observer_id).is_some() {
                println!("❌ 观察者 {} 已注销", observer_id);
            }
        }
        
        fn notify(&self, event: &str, data: &str) {
            for (id, observer) in &self.observers {
                observer.update(event, data);
                println!("📡 通知观察者 {} 已更新", id);
            }
        }
    }
    
    // 客户端代码
    let mut news_agency = NewsAgency::new();
    
    let observer1 = Box::new(ConcreteObserver { id: 1, name: "新闻网站".to_string() });
    let observer2 = Box::new(ConcreteObserver { id: 2, name: "手机APP".to_string() });
    let observer3 = Box::new(ConcreteObserver { id: 3, name: "邮件服务".to_string() });
    
    news_agency.attach(observer1);
    news_agency.attach(observer2);
    news_agency.attach(observer3);
    
    news_agency.publish_news("突发新闻".to_string(), "Rust 2.0发布了！".to_string());
    
    news_agency.detach(2);
    
    news_agency.publish_news("技术新闻".to_string(), "WebAssembly获得新特性".to_string());
}

/// 演示建造者模式
pub fn builder_pattern() {
    println!("🔧 建造者模式：");
    
    // 产品
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Email {
        to: String,
        from: String,
        subject: String,
        body: String,
        attachments: Vec<String>,
        priority: String,
    }
    
    // 建造者特征
    trait EmailBuilder {
        fn new() -> Self;
        fn to(&mut self, email: &str) -> &mut Self;
        fn from(&mut self, email: &str) -> &mut Self;
        fn subject(&mut self, subject: &str) -> &mut Self;
        fn body(&mut self, body: &str) -> &mut Self;
        fn attachment(&mut self, file: &str) -> &mut Self;
        fn priority(&mut self, priority: &str) -> &mut Self;
        fn build(&mut self) -> Email;
    }
    
    // 具体建造者
    struct ConcreteEmailBuilder {
        to: Option<String>,
        from: Option<String>,
        subject: Option<String>,
        body: Option<String>,
        attachments: Vec<String>,
        priority: Option<String>,
    }
    
    impl EmailBuilder for ConcreteEmailBuilder {
        fn new() -> Self {
            Self {
                to: None,
                from: None,
                subject: None,
                body: None,
                attachments: Vec::new(),
                priority: None,
            }
        }
        
        fn to(&mut self, email: &str) -> &mut Self {
            self.to = Some(email.to_string());
            self
        }
        
        fn from(&mut self, email: &str) -> &mut Self {
            self.from = Some(email.to_string());
            self
        }
        
        fn subject(&mut self, subject: &str) -> &mut Self {
            self.subject = Some(subject.to_string());
            self
        }
        
        fn body(&mut self, body: &str) -> &mut Self {
            self.body = Some(body.to_string());
            self
        }
        
        fn attachment(&mut self, file: &str) -> &mut Self {
            self.attachments.push(file.to_string());
            self
        }
        
        fn priority(&mut self, priority: &str) -> &mut Self {
            self.priority = Some(priority.to_string());
            self
        }
        
        fn build(&mut self) -> Email {
            Email {
                to: self.to.clone().unwrap_or_else(|| "unknown@domain.com".to_string()),
                from: self.from.clone().unwrap_or_else(|| "noreply@domain.com".to_string()),
                subject: self.subject.clone().unwrap_or_else(|| "无主题".to_string()),
                body: self.body.clone().unwrap_or_else(|| "无内容".to_string()),
                attachments: self.attachments.clone(),
                priority: self.priority.clone().unwrap_or_else(|| "普通".to_string()),
            }
        }
    }
    
    // 客户端代码
    let mut email_builder = ConcreteEmailBuilder::new();
    
    let email = email_builder
        .to("user@example.com")
        .from("support@company.com")
        .subject("重要通知")
        .body("这是一个重要的系统通知，请查收附件。")
        .attachment("report.pdf")
        .attachment("data.xlsx")
        .priority("高")
        .build();
    
    println!("📧 生成的邮件: {:?}", email);
    
    // 另一个例子
    let mut simple_email = ConcreteEmailBuilder::new();
    let simple_mail = simple_email
        .to("friend@example.com")
        .subject("问候")
        .body("你好，希望你一切都好！")
        .build();
    
    println!("📧 简单邮件: {:?}", simple_mail);
}

/// 运行高级类型和生命周期示例
pub fn run_advanced_types_examples() {
    println!("🎯 === 现代化高级类型和生命周期示例 ===");
    println!();
    
    modern_associated_types();
    println!();
    
    modern_generic_parameters();
    println!();
    
    modern_lifetimes();
    println!();
    
    modern_trait_system();
    println!();
    
    modern_generic_constraints();
    println!();
    
    modern_newtype_pattern();
    
    println!("\n✅ 所有高级类型和生命周期示例运行完成！");
}

/// 运行设计模式示例
pub fn run_design_pattern_examples() {
    println!("🎯 === 设计模式示例 ===");
    println!();
    
    println!("=== 工厂模式 ===");
    factory_pattern();
    println!();
    
    println!("=== 策略模式 ===");
    strategy_pattern();
    println!();
    
    println!("=== 装饰器模式 ===");
    decorator_pattern();
    println!();
    
    println!("=== 观察者模式 ===");
    observer_pattern();
    println!();
    
    println!("=== 建造者模式 ===");
    builder_pattern();
    
    println!("\n✅ 所有设计模式示例运行完成！");
}