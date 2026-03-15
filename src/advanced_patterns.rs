//! # 进阶设计模式和架构模块
//!
//! 这个模块演示了Rust的进阶设计模式和架构概念。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

use std::fmt;

// ============== Builder 模式 ==============

/// 数据库连接配置
///
/// 这个结构体表示数据库连接的配置信息，包括主机、端口、数据库名等。
#[derive(Clone)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    pool_size: usize,
    timeout_secs: u64,
}

/// Builder用于构建DatabaseConfig
pub struct DatabaseConfigBuilder {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    pool_size: usize,
    timeout_secs: u64,
}

impl Default for DatabaseConfigBuilder {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            username: "<provide-username>".to_string(),
            password: "<provide-password>".to_string(),
            pool_size: 10,
            timeout_secs: 30,
        }
    }
}

impl DatabaseConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn pool_size(mut self, size: usize) -> Self {
        self.pool_size = size;
        self
    }

    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn build(self) -> Result<DatabaseConfig, String> {
        if self.host.is_empty() {
            return Err("Host cannot be empty".to_string());
        }
        if self.port == 0 {
            return Err("Port must be greater than 0".to_string());
        }

        Ok(DatabaseConfig {
            host: self.host,
            port: self.port,
            database: self.database,
            username: self.username,
            password: self.password,
            pool_size: self.pool_size,
            timeout_secs: self.timeout_secs,
        })
    }
}

impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("pool_size", &self.pool_size)
            .field("timeout_secs", &self.timeout_secs)
            .finish()
    }
}

// ============== Strategy 模式 ==============

/// 支付策略特征
pub trait PaymentStrategy: fmt::Debug {
    fn pay(&self, amount: f64) -> Result<String, String>;
    fn validate(&self) -> bool;
}

/// 信用卡支付策略
#[derive(Debug, Clone)]
pub struct CreditCardPayment {
    card_number: String,
    cvv: String,
}

impl CreditCardPayment {
    pub fn new(card_number: &str, cvv: &str) -> Self {
        Self {
            card_number: card_number.to_string(),
            cvv: cvv.to_string(),
        }
    }
}

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        if !self.validate() {
            return Err("Invalid credit card".to_string());
        }
        Ok(format!("💳 Credit card payment of ${:.2} processed", amount))
    }

    fn validate(&self) -> bool {
        self.card_number.len() >= 13 && self.cvv.len() == 3
    }
}

/// PayPal支付策略
#[derive(Debug, Clone)]
pub struct PayPalPayment {
    email: String,
}

impl PayPalPayment {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        if !self.validate() {
            return Err("Invalid PayPal account".to_string());
        }
        Ok(format!("🅿️ PayPal payment of ${:.2} to {} processed", amount, self.email))
    }

    fn validate(&self) -> bool {
        self.email.contains('@')
    }
}

/// 加密货币支付策略
#[derive(Debug, Clone)]
pub struct CryptoPayment {
    wallet_address: String,
    currency: String,
}

impl CryptoPayment {
    pub fn new(wallet_address: &str, currency: &str) -> Self {
        Self {
            wallet_address: wallet_address.to_string(),
            currency: currency.to_string(),
        }
    }
}

impl PaymentStrategy for CryptoPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        if !self.validate() {
            return Err("Invalid crypto wallet".to_string());
        }
        Ok(format!("🪙 {} payment of {:.2} {} processed", self.currency, amount, self.currency))
    }

    fn validate(&self) -> bool {
        self.wallet_address.len() >= 26
    }
}

// ============== Observer 模式 ==============

/// 观察者特征
pub trait Observer: fmt::Debug {
    fn update(&self, message: &str);
}

/// Email订阅者
#[derive(Debug, Clone)]
pub struct EmailSubscriber {
    email: String,
}

impl EmailSubscriber {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

impl Observer for EmailSubscriber {
    fn update(&self, message: &str) {
        println!("📧 Email to {}: {}", self.email, message);
    }
}

/// SMS订阅者
#[derive(Debug, Clone)]
pub struct SmsSubscriber {
    phone: String,
}

impl SmsSubscriber {
    pub fn new(phone: &str) -> Self {
        Self {
            phone: phone.to_string(),
        }
    }
}

impl Observer for SmsSubscriber {
    fn update(&self, message: &str) {
        println!("📱 SMS to {}: {}", self.phone, message);
    }
}

/// 事件发布者
#[derive(Debug)]
pub struct EventPublisher {
    observers: Vec<Box<dyn Observer>>,
}

impl EventPublisher {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }
}

// ============== State 模式 ==============

/// 订单状态特征
pub trait OrderState: fmt::Debug {
    fn next_state(&self) -> Box<dyn OrderState>;
    fn get_status(&self) -> String;
}

#[derive(Debug)]
pub struct PendingState;

impl OrderState for PendingState {
    fn next_state(&self) -> Box<dyn OrderState> {
        Box::new(ConfirmedState)
    }

    fn get_status(&self) -> String {
        "⏳ Pending".to_string()
    }
}

#[derive(Debug)]
pub struct ConfirmedState;

impl OrderState for ConfirmedState {
    fn next_state(&self) -> Box<dyn OrderState> {
        Box::new(ShippingState)
    }

    fn get_status(&self) -> String {
        "✅ Confirmed".to_string()
    }
}

#[derive(Debug)]
pub struct ShippingState;

impl OrderState for ShippingState {
    fn next_state(&self) -> Box<dyn OrderState> {
        Box::new(DeliveredState)
    }

    fn get_status(&self) -> String {
        "🚚 Shipping".to_string()
    }
}

#[derive(Debug)]
pub struct DeliveredState;

impl OrderState for DeliveredState {
    fn next_state(&self) -> Box<dyn OrderState> {
        Box::new(DeliveredState)
    }

    fn get_status(&self) -> String {
        "📦 Delivered".to_string()
    }
}

/// 订单
pub struct Order {
    id: String,
    state: Box<dyn OrderState>,
}

impl Order {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            state: Box::new(PendingState),
        }
    }

    pub fn get_status(&self) -> String {
        self.state.get_status()
    }

    pub fn advance(&mut self) {
        self.state = self.state.next_state();
    }
}

// ============== Factory 模式 ==============

/// UI元素特征
pub trait UIElement: fmt::Debug {
    fn render(&self);
}

#[derive(Debug)]
pub struct Button {
    label: String,
}

impl UIElement for Button {
    fn render(&self) {
        println!("🔘 Button: {}", self.label);
    }
}

#[derive(Debug)]
pub struct TextInput {
    placeholder: String,
}

impl UIElement for TextInput {
    fn render(&self) {
        println!("📝 TextInput: {}", self.placeholder);
    }
}

#[derive(Debug)]
pub struct Checkbox {
    label: String,
}

impl UIElement for Checkbox {
    fn render(&self) {
        println!("☑️ Checkbox: {}", self.label);
    }
}

/// UI元素工厂
pub struct UIFactory;

impl UIFactory {
    pub fn create_button(label: &str) -> Box<dyn UIElement> {
        Box::new(Button {
            label: label.to_string(),
        })
    }

    pub fn create_text_input(placeholder: &str) -> Box<dyn UIElement> {
        Box::new(TextInput {
            placeholder: placeholder.to_string(),
        })
    }

    pub fn create_checkbox(label: &str) -> Box<dyn UIElement> {
        Box::new(Checkbox {
            label: label.to_string(),
        })
    }
}

// ============== Decorator 模式 ==============

/// 咖啡特征
pub trait Coffee: fmt::Debug {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
}

/// 基础咖啡
#[derive(Debug)]
pub struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn description(&self) -> String {
        "Simple Coffee".to_string()
    }

    fn cost(&self) -> f64 {
        2.0
    }
}

/// 咖啡装饰器 - 牛奶
#[derive(Debug)]
pub struct CoffeeWithMilk {
    inner_coffee: Box<dyn Coffee>,
}

impl CoffeeWithMilk {
    pub fn new(base_coffee: Box<dyn Coffee>) -> Self {
        Self { inner_coffee: base_coffee }
    }
}

impl Coffee for CoffeeWithMilk {
    fn description(&self) -> String {
        format!("{} with Milk", self.inner_coffee.description())
    }

    fn cost(&self) -> f64 {
        self.inner_coffee.cost() + 0.5
    }
}

/// 咖啡装饰器 - 糖
#[derive(Debug)]
pub struct CoffeeWithSugar {
    inner_coffee: Box<dyn Coffee>,
}

impl CoffeeWithSugar {
    pub fn new(base_coffee: Box<dyn Coffee>) -> Self {
        Self { inner_coffee: base_coffee }
    }
}

impl Coffee for CoffeeWithSugar {
    fn description(&self) -> String {
        format!("{} with Sugar", self.inner_coffee.description())
    }

    fn cost(&self) -> f64 {
        self.inner_coffee.cost() + 0.3
    }
}

/// 咖啡构建器 - 提供流畅的API来创建咖啡
pub struct CoffeeBuilder {
    base_coffee: Box<dyn Coffee>,
}

impl CoffeeBuilder {
    pub fn new() -> Self {
        Self {
            base_coffee: Box::new(SimpleCoffee),
        }
    }

    pub fn with_milk(mut self) -> Self {
        self.base_coffee = Box::new(CoffeeWithMilk::new(self.base_coffee));
        self
    }

    pub fn with_sugar(mut self) -> Self {
        self.base_coffee = Box::new(CoffeeWithSugar::new(self.base_coffee));
        self
    }

    pub fn build(self) -> Box<dyn Coffee> {
        self.base_coffee
    }
}

// ============== 主函数 ==============

/// 演示Builder模式
fn demo_builder() {
    println!("\n📐 === Builder 模式演示 ===");
    let config = DatabaseConfigBuilder::new()
        .host("db.example.com")
        .port(5432)
        .database("myapp")
        .username("app_user")
        .password("<supplied-at-runtime>")
        .pool_size(20)
        .timeout_secs(60)
        .build();

    match config {
        Ok(cfg) => {
            println!("✅ Database config created: {:?}", cfg);
            println!("ℹ️ password 字段已脱敏；示例中的凭据应由调用者在运行时提供。")
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

/// 演示Strategy模式
fn demo_strategy() {
    println!("\n💳 === Strategy 模式演示 ===");
    let strategies: Vec<Box<dyn PaymentStrategy>> = vec![
        Box::new(CreditCardPayment::new("4532015112830366", "123")),
        Box::new(PayPalPayment::new("user@example.com")),
        Box::new(CryptoPayment::new("1A1z7agoat2WzWZQRNCZrRcB7SNWvqBBx5", "BTC")),
    ];

    let amount = 99.99;
    for strategy in strategies {
        match strategy.pay(amount) {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("❌ {}", e),
        }
    }
}

/// 演示Observer模式
fn demo_observer() {
    println!("\n👀 === Observer 模式演示 ===");
    let mut publisher = EventPublisher::new();
    
    publisher.subscribe(Box::new(EmailSubscriber::new("admin@example.com")));
    publisher.subscribe(Box::new(SmsSubscriber::new("+1-555-0123")));
    
    println!("📢 Publishing event...");
    publisher.notify("Important announcement: System maintenance scheduled");
}

/// 演示State模式
fn demo_state() {
    println!("\n🔄 === State 模式演示 ===");
    let mut order = Order::new("ORD-001");
    
    println!("订单 {}: {}", order.id, order.get_status());
    
    order.advance();
    println!("订单 {}: {}", order.id, order.get_status());
    
    order.advance();
    println!("订单 {}: {}", order.id, order.get_status());
    
    order.advance();
    println!("订单 {}: {}", order.id, order.get_status());
}

/// 演示Factory模式
fn demo_factory() {
    println!("\n🏭 === Factory 模式演示 ===");
    let elements: Vec<Box<dyn UIElement>> = vec![
        UIFactory::create_button("Click Me"),
        UIFactory::create_text_input("Enter your name"),
        UIFactory::create_checkbox("I agree"),
    ];

    for element in elements {
        element.render();
    }
}

/// 演示Decorator模式
fn demo_decorator() {
    println!("\n🎁 === Decorator 模式演示 ===");

    // 使用传统装饰器模式
    let simple: Box<dyn Coffee> = Box::new(SimpleCoffee);
    println!("Simple: {} - ${:.2}", simple.description(), simple.cost());

    let with_milk: Box<dyn Coffee> = Box::new(CoffeeWithMilk::new(Box::new(SimpleCoffee)));
    println!("With Milk: {} - ${:.2}", with_milk.description(), with_milk.cost());

    // 使用构建器模式创建复杂咖啡
    let fancy_coffee = CoffeeBuilder::new()
        .with_milk()
        .with_sugar()
        .build();

    println!("Fancy (Builder): {} - ${:.2}", fancy_coffee.description(), fancy_coffee.cost());

    // 更复杂的例子
    let complex_coffee = CoffeeBuilder::new()
        .with_sugar()
        .with_milk()
        .with_sugar() // 双份糖
        .build();

    println!("Complex: {} - ${:.2}", complex_coffee.description(), complex_coffee.cost());
}

/// 运行所有进阶设计模式示例
///
/// 这个函数演示了多种设计模式的实现，包括：
/// - Builder模式：流畅的对象构建API
/// - Strategy模式：可互换的算法家族
/// - Observer模式：发布-订阅机制
/// - State模式：对象状态转换
/// - Factory模式：对象创建工厂
/// - Decorator模式：动态添加行为
///
/// # 示例
/// ```
/// use rust_learn::advanced_patterns::run_all_patterns;
/// run_all_patterns();
/// ```
pub fn run_all_patterns() {
    println!("🎯 === 进阶设计模式和架构示例 ===");
    println!();
    
    demo_builder();
    demo_strategy();
    demo_observer();
    demo_state();
    demo_factory();
    demo_decorator();
    
    println!("\n✅ 所有进阶设计模式示例运行完成！");
}