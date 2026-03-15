//! # 类型系统模块
//!
//! 这个模块演示了Rust的类型系统，包括结构体、枚举、特征等。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

/// 现代化结构体演示
pub fn structs() {
    println!("🏗️ 现代化结构体：");

    // 使用现代化的结构体定义
    #[derive(Debug, Clone, PartialEq)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 现代化结构体实现
    impl User {
        fn new(username: String, email: String) -> Self {
            Self {
                username,
                email,
                sign_in_count: 1,
                active: true,
            }
        }

        #[allow(dead_code)]
        fn activate(&mut self) {
            self.active = true;
        }

        #[allow(dead_code)]
        fn deactivate(&mut self) {
            self.active = false;
        }

        fn get_display_name(&self) -> &str {
            if self.active {
                &self.username
            } else {
                "[停用]"
            }
        }
    }

    // 创建结构体实例
    let user1 = User::new(
        String::from("rust_learner"),
        String::from("rust@example.com"),
    );

    println!("用户信息: {}", user1.get_display_name());
    println!("详情: {:?}", user1);

    // 使用更新语法
    let user2 = User {
        username: String::from("another_learner"),
        email: String::from("learn@example.com"),
        ..user1 // 使用其他字段的默认值
    };

    println!("用户2: {:?}", user2);

    // 元组结构体的现代化用法
    #[derive(Debug, Clone, Copy)]
    struct Color(u8, u8, u8);

    impl Color {
        fn new(r: u8, g: u8, b: u8) -> Self {
            Self(r, g, b)
        }

        fn to_hex_string(&self) -> String {
            format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
        }
    }

    let black = Color::new(0, 0, 0);
    let white = Color::new(255, 255, 255);
    println!("黑色: {:?} -> {}", black, black.to_hex_string());
    println!("白色: {:?} -> {}", white, white.to_hex_string());

    // 单元结构体用于特征实现
    #[derive(Debug, Clone)]
    struct Empty;

    impl Display for Empty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Empty struct")
        }
    }

    let unit = Empty;
    println!("单元结构体: {}", unit);
}

/// 现代化枚举演示
pub fn enums() {
    println!("🎯 现代化枚举：");

    // 现代化IP地址枚举
    #[derive(Debug, Clone)]
    enum IpAddr {
        V4 { addr: [u8; 4] },
        V6(String),
    }

    impl IpAddr {
        fn get_description(&self) -> String {
            match self {
                Self::V4 { addr } => {
                    format!("IPv4: {}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3])
                }
                Self::V6(addr) => format!("IPv6: {}", addr),
            }
        }

        fn is_localhost(&self) -> bool {
            match self {
                Self::V4 { addr } => *addr == [127, 0, 0, 1],
                Self::V6(addr) => addr == "::1",
            }
        }
    }

    let home = IpAddr::V4 {
        addr: [192, 168, 1, 1],
    };
    let loopback = IpAddr::V6(String::from("::1"));

    println!("本地地址: {}", home.get_description());
    println!("环回地址: {}", loopback.get_description());
    println!(
        "localhost检查: {} | {}",
        home.is_localhost(),
        loopback.is_localhost()
    );

    // 现代化Option枚举使用
    let some_number = Some(42);
    let _some_string = Some("Hello Rust".to_string());
    let _absent_number: Option<i32> = None;

    // 使用模式匹配进行安全解包
    match some_number {
        Some(n) if n > 40 => println!("大数: {}", n),
        Some(n) => println!("小数: {}", n),
        None => println!("没有值"),
    }

    // 现代化Result类型使用
    #[derive(Debug)]
    #[allow(dead_code)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
    }

    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("除法结果: {}", result),
        Err(MathError::DivisionByZero) => println!("除零错误"),
        Err(MathError::NegativeSquareRoot) => println!("负数平方根错误"),
    }
}

/// 现代化特征系统
pub fn traits() {
    println!("🎨 现代化特征系统：");

    // 定义高级特征
    trait Summary {
        fn summarize(&self) -> String {
            "No summary available".to_string()
        }

        fn summarize_author(&self) -> String {
            "Unknown author".to_string()
        }

        fn detailed_summary(&self) -> String {
            format!("{} - by {}", self.summarize(), self.summarize_author())
        }
    }

    trait Display: Summary {
        fn display_format(&self) -> String;
    }

    // 实现特征
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!(
                "头条: {}, 地点: {}, 作者: {}",
                self.headline, self.location, self.author
            )
        }

        fn summarize_author(&self) -> String {
            self.author.clone()
        }
    }

    impl Display for NewsArticle {
        fn display_format(&self) -> String {
            format!("📰 {}", self.detailed_summary())
        }
    }

    // 现代特征对象
    trait Drawable {
        fn draw(&self) -> String;
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Drawable for Circle {
        fn draw(&self) -> String {
            format!("🔵 圆形(半径: {:.1})", self.radius)
        }

        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl Drawable for Rectangle {
        fn draw(&self) -> String {
            format!("🟦 矩形({:.1} x {:.1})", self.width, self.height)
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let article = NewsArticle {
        headline: "Rust 2024新特性发布".to_string(),
        location: "中国".to_string(),
        author: "Rust团队".to_string(),
        content: "Rust编程语言发布了2024版本，包含了众多新特性...".to_string(),
    };

    let shapes: Vec<&dyn Drawable> = vec![
        &Circle { radius: 5.0 },
        &Rectangle {
            width: 4.0,
            height: 6.0,
        },
    ];

    println!("文章摘要: {}", article.display_format());
    println!("绘图示例:");

    for shape in shapes {
        println!("  {} - 面积: {:.2}", shape.draw(), shape.area());
    }
}

/// 现代化泛型系统
pub fn generics() {
    println!("🔧 现代化泛型系统：");

    // 现代化泛型函数
    #[allow(dead_code)]
    fn largest<T: PartialOrd + Clone + fmt::Display>(list: &[T]) -> T {
        let mut largest = list[0].clone();

        for item in list {
            if item > &largest {
                largest = item.clone();
            }
        }

        largest
    }

    // 现代化泛型结构体
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: fmt::Display + Copy + PartialOrd> Point<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }

        #[allow(dead_code)]
        fn distance_from_origin(&self) -> T {
            // 对于数值类型，这可能会很有用
            if self.x > self.y { self.x } else { self.y }
        }
    }

    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 3.2);

    println!("整数点: {:?}", int_point);
    println!("浮点数点: {:?}", float_point);

    // 现代化泛型枚举
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 泛型特征约束示例
    trait Maximum {
        type Item;

        fn get_max(&self) -> Option<&Self::Item>;
    }

    impl<T: PartialOrd> Maximum for Vec<T> {
        type Item = T;

        fn get_max(&self) -> Option<&Self::Item> {
            self.iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        }
    }

    let numbers = vec![34, 50, 25, 100, 65];
    if let Some(max) = numbers.get_max() {
        println!("最大值: {}", max);
    }
}

/// 演示高级特征对象和动态分发
pub fn advanced_trait_objects() {
    println!("🎨 高级特征对象：");

    // 简化形状示例
    trait SimpleShape {
        fn area(&self) -> f64;
        fn name(&self) -> &str;
    }

    #[derive(Debug, Clone)]
    struct SimpleCircle {
        radius: f64,
    }

    impl SimpleShape for SimpleCircle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        fn name(&self) -> &str {
            "圆形"
        }
    }

    #[derive(Debug, Clone)]
    struct SimpleRectangle {
        width: f64,
        height: f64,
    }

    impl SimpleShape for SimpleRectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn name(&self) -> &str {
            "矩形"
        }
    }

    let shapes: Vec<Box<dyn SimpleShape>> = vec![
        Box::new(SimpleCircle { radius: 5.0 }),
        Box::new(SimpleRectangle {
            width: 4.0,
            height: 6.0,
        }),
        Box::new(SimpleCircle { radius: 3.0 }),
    ];

    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("总形状面积: {:.2}", total_area);

    for (i, shape) in shapes.iter().enumerate() {
        println!(
            "形状{}: {} (面积: {:.2})",
            i + 1,
            shape.name(),
            shape.area()
        );
    }
}

/// 演示关联类型和泛型关联类型
pub fn associated_types() {
    println!("🔗 关联类型和泛型关联类型：");

    // 标准库中的关联类型示例
    use std::ops::Add;

    // 使用泛型的加法操作
    fn add_vectors<T: Add<Output = T> + Clone>(a: &[T], b: &[T]) -> Vec<T> {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| x.clone() + y.clone())
            .collect()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let result = add_vectors(&v1, &v2);
    println!("向量加法结果: {:?}", result);

    // 自定义关联类型
    trait Graph {
        type Node;
        type Edge;

        fn add_node(&mut self, node: Self::Node);
        fn add_edge(
            &mut self,
            from: Self::Node,
            to: Self::Node,
            edge: Self::Edge,
        ) -> Result<(), String>;
        fn get_neighbors(&self, node: &Self::Node) -> Option<Vec<&Self::Node>>;
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct City {
        name: String,
        population: u64,
    }

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Route {
        #[allow(dead_code)]
        distance: f64,
        #[allow(dead_code)]
        travel_time: f64,
    }

    struct SimpleGraph {
        nodes: Vec<City>,
        edges: Vec<(usize, usize, Route)>,
    }

    impl SimpleGraph {
        fn find_city_index(&self, city: &City) -> Option<usize> {
            self.nodes.iter().position(|node| node == city)
        }
    }

    impl Graph for SimpleGraph {
        type Node = City;
        type Edge = Route;

        fn add_node(&mut self, node: Self::Node) {
            self.nodes.push(node);
        }

        fn add_edge(
            &mut self,
            from: Self::Node,
            to: Self::Node,
            edge: Self::Edge,
        ) -> Result<(), String> {
            let from_idx = self
                .find_city_index(&from)
                .ok_or_else(|| format!("起点城市不存在: {}", from.name))?;
            let to_idx = self
                .find_city_index(&to)
                .ok_or_else(|| format!("终点城市不存在: {}", to.name))?;
            self.edges.push((from_idx, to_idx, edge));
            Ok(())
        }

        fn get_neighbors(&self, node: &Self::Node) -> Option<Vec<&Self::Node>> {
            let node_idx = self.find_city_index(node)?;
            Some(
                self.edges
                    .iter()
                    .filter(|(from, _, _)| *from == node_idx)
                    .map(|(_, to_idx, _)| &self.nodes[*to_idx])
                    .collect(),
            )
        }
    }

    let mut graph = SimpleGraph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let beijing = City {
        name: "北京".to_string(),
        population: 21540000,
    };
    let shanghai = City {
        name: "上海".to_string(),
        population: 24870000,
    };
    let guangzhou = City {
        name: "广州".to_string(),
        population: 15300000,
    };
    let shenzhen = City {
        name: "深圳".to_string(),
        population: 17600000,
    };

    graph.add_node(beijing.clone());
    graph.add_node(shanghai.clone());
    graph.add_node(guangzhou.clone());

    let beijing_to_shanghai = Route {
        distance: 1200.0,
        travel_time: 8.5,
    };
    match graph.add_edge(beijing.clone(), shanghai.clone(), beijing_to_shanghai) {
        Ok(()) => println!("✅ 已添加 北京 -> 上海 路线"),
        Err(error) => println!("❌ 添加路线失败: {}", error),
    }

    match graph.add_edge(guangzhou.clone(), shenzhen.clone(), Route {
        distance: 140.0,
        travel_time: 1.5,
    }) {
        Ok(()) => println!("✅ 已添加 广州 -> 深圳 路线"),
        Err(error) => println!("⚠️ 受控失败路径: {}", error),
    }

    match graph.get_neighbors(&beijing) {
        Some(neighbors) => println!("北京的邻居城市: {:?}", neighbors),
        None => println!("未找到城市: {}", beijing.name),
    }

    match graph.get_neighbors(&shenzhen) {
        Some(neighbors) => println!("深圳的邻居城市: {:?}", neighbors),
        None => println!("未找到城市: {}", shenzhen.name),
    }
}

/// 演示NewType模式和类型安全
pub fn newtype_pattern() {
    println!("📦 NewType模式：");

    // 类型安全的用户ID
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserId(u64);

    // 类型安全的会话ID
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct SessionId(String);

    // 类型安全的年龄
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Age(u8);

    impl UserId {
        pub fn new(id: u64) -> Self {
            Self(id)
        }

        pub fn as_u64(&self) -> u64 {
            self.0
        }
    }

    impl SessionId {
        pub fn new(id: &str) -> Self {
            Self(id.to_string())
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    impl Age {
        pub fn new(age: u8) -> Result<Self, &'static str> {
            if age >= 13 && age <= 120 {
                Ok(Self(age))
            } else {
                Err("年龄必须在13-120之间")
            }
        }

        pub fn as_u8(&self) -> u8 {
            self.0
        }
    }

    // 类型安全的使用示例
    let user_id = UserId::new(12345);
    let session_id = SessionId::new("abc-def-123");
    let age = Age::new(25).unwrap();

    // 这些不能混淆
    fn process_user(user_id: UserId, age: Age) {
        println!("处理用户ID: {}, 年龄: {}", user_id.as_u64(), age.as_u8());
    }

    let user_id_clone = user_id.clone();
    let age_clone = age.clone();
    process_user(user_id_clone, age_clone);

    // 编译时防止错误
    // process_user(session_id.clone(), age.clone()); // 这会编译错误

    println!("用户ID: {}", user_id.as_u64());
    println!("会话ID: {}", session_id.as_str());
}

/// 演示高级特征边界和约束
pub fn advanced_trait_bounds() {
    println!("🔒 高级特征边界：");

    // 多重特征约束
    fn process_item<T>(item: &T) -> String
    where
        T: Clone + std::fmt::Display + PartialEq,
    {
        format!("项目: {} (克隆相等: {})", item, item.clone() == *item)
    }

    let text = "Hello Rust";
    println!("{}", process_item(&text));

    // 特征对象的高级使用
    #[allow(dead_code)]
    trait Drawable {
        type Color;

        fn draw(&self) -> String;
        fn get_color(&self) -> &Self::Color;
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Red;
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Blue;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct RedCircle {
        #[allow(dead_code)]
        radius: f64,
        #[allow(dead_code)]
        color: Red,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct BlueRectangle {
        #[allow(dead_code)]
        width: f64,
        #[allow(dead_code)]
        height: f64,
        #[allow(dead_code)]
        color: Blue,
    }

    impl Drawable for RedCircle {
        type Color = Red;

        fn draw(&self) -> String {
            format!("绘制红色圆圈，半径: {:.1}", self.radius)
        }

        fn get_color(&self) -> &Self::Color {
            &self.color
        }
    }

    impl Drawable for BlueRectangle {
        type Color = Blue;

        fn draw(&self) -> String {
            format!("绘制蓝色矩形，{:.1} x {:.1}", self.width, self.height)
        }

        fn get_color(&self) -> &Self::Color {
            &self.color
        }
    }

    // 简化特征对象使用
    trait SimpleDrawable {
        fn draw(&self) -> String;
    }

    #[derive(Debug)]
    struct SimpleCircle {
        radius: f64,
    }

    impl SimpleDrawable for SimpleCircle {
        fn draw(&self) -> String {
            format!("绘制圆圈，半径: {:.1}", self.radius)
        }
    }

    #[derive(Debug)]
    struct SimpleRectangle {
        width: f64,
        height: f64,
    }

    impl SimpleDrawable for SimpleRectangle {
        fn draw(&self) -> String {
            format!("绘制矩形，{:.1} x {:.1}", self.width, self.height)
        }
    }

    let shapes: Vec<Box<dyn SimpleDrawable>> = vec![
        Box::new(SimpleCircle { radius: 5.0 }),
        Box::new(SimpleRectangle {
            width: 4.0,
            height: 6.0,
        }),
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("{}: {}", i + 1, shape.draw());
    }

    // 简化的特征约束示例
    fn process_item_simple<T>(item: &T) -> String
    where
        T: Clone + std::fmt::Display + PartialEq,
    {
        format!("项目: {} (相等: {})", item, item.clone() == *item)
    }

    let text = "Hello Rust";
    println!("{}", process_item_simple(&text));
}

/// 演示Rust的类型级编程
pub fn type_level_programming() {
    println!("⚙️ 类型级编程：");

    // 标记类型
    struct Enabled;
    struct Disabled;

    // 类型状态机
    struct LightSwitch<State> {
        is_on: bool,
        _state: std::marker::PhantomData<State>,
    }

    impl LightSwitch<Disabled> {
        fn new() -> Self {
            Self {
                is_on: false,
                _state: std::marker::PhantomData,
            }
        }

        fn enable(self) -> LightSwitch<Enabled> {
            LightSwitch {
                is_on: true,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl LightSwitch<Enabled> {
        fn disable(self) -> LightSwitch<Disabled> {
            LightSwitch {
                is_on: false,
                _state: std::marker::PhantomData,
            }
        }

        fn is_enabled(&self) -> bool {
            self.is_on
        }
    }

    let switch = LightSwitch::<Disabled>::new();
    let enabled_switch = switch.enable();

    if enabled_switch.is_enabled() {
        println!("💡 灯是开启的");
    }

    // 不可变/可变引用类型标记
    struct ReadOnly;
    struct ReadWrite;

    struct Permission<Access> {
        data: Vec<i32>,
        _access: std::marker::PhantomData<Access>,
    }

    impl Permission<ReadOnly> {
        fn new() -> Self {
            Self {
                data: vec![1, 2, 3, 4, 5],
                _access: std::marker::PhantomData,
            }
        }

        fn read(&self) -> &[i32] {
            &self.data
        }
    }

    impl Permission<ReadWrite> {
        fn new() -> Self {
            Self {
                data: vec![1, 2, 3, 4, 5],
                _access: std::marker::PhantomData,
            }
        }

        fn read(&self) -> &[i32] {
            &self.data
        }

        fn write(&mut self, index: usize, value: i32) {
            if index < self.data.len() {
                self.data[index] = value;
            }
        }
    }

    let readonly_perm = Permission::<ReadOnly>::new();
    println!("只读权限数据: {:?}", readonly_perm.read());

    let mut readwrite_perm = Permission::<ReadWrite>::new();
    readwrite_perm.write(0, 10);
    println!("读写权限数据: {:?}", readwrite_perm.read());
}

/// 演示组合模式和协议设计
pub fn composition_patterns() {
    println!("🔧 组合模式：");

    // 组件特征
    trait Component {
        fn operation(&self) -> String;
    }

    // 叶节点
    struct Leaf {
        name: String,
    }

    impl Component for Leaf {
        fn operation(&self) -> String {
            format!("叶子: {}", self.name)
        }
    }

    // 组合节点
    struct Composite {
        name: String,
        children: Vec<Box<dyn Component>>,
    }

    impl Composite {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                children: Vec::new(),
            }
        }

        fn add(&mut self, component: Box<dyn Component>) {
            self.children.push(component);
        }

        fn remove(&mut self, index: usize) {
            if index < self.children.len() {
                self.children.remove(index);
            }
        }
    }

    impl Component for Composite {
        fn operation(&self) -> String {
            let mut result = format!("组合: {}", self.name);
            for child in &self.children {
                result.push_str(&format!("({})", child.operation()));
            }
            result
        }
    }

    // 构建组合结构
    let mut root = Composite::new("根");
    let mut branch1 = Composite::new("分支1");
    let mut branch2 = Composite::new("分支2");

    branch1.add(Box::new(Leaf {
        name: "叶子1".to_string(),
    }));
    branch1.add(Box::new(Leaf {
        name: "叶子2".to_string(),
    }));

    branch2.add(Box::new(Leaf {
        name: "叶子3".to_string(),
    }));

    root.add(Box::new(branch1));
    root.add(Box::new(branch2));

    println!("组合结构: {}", root.operation());

    // 装饰器模式
    trait DataSource: Component {
        fn get_data(&self) -> &str;
    }

    struct SimpleDataSource {
        data: String,
    }

    impl DataSource for SimpleDataSource {
        fn get_data(&self) -> &str {
            &self.data
        }
    }

    impl Component for SimpleDataSource {
        fn operation(&self) -> String {
            self.get_data().to_string()
        }
    }

    struct DataSourceDecorator {
        source: Box<dyn DataSource>,
    }

    impl DataSourceDecorator {
        fn new(source: Box<dyn DataSource>) -> Self {
            Self { source }
        }
    }

    impl DataSource for DataSourceDecorator {
        fn get_data(&self) -> &str {
            // 可以在这里添加额外的逻辑，比如加密、缓存等
            self.source.get_data()
        }
    }

    impl Component for DataSourceDecorator {
        fn operation(&self) -> String {
            format!("装饰器包装: {}", self.get_data())
        }
    }

    let simple = SimpleDataSource {
        data: "原始数据".to_string(),
    };
    let decorated = DataSourceDecorator::new(Box::new(simple));

    println!("装饰器结果: {}", decorated.operation());
}

/// 运行类型系统示例
pub fn run_types_examples() {
    println!("🎯 === 现代化类型系统示例 ===");
    println!();

    structs();
    println!();

    enums();
    println!();

    traits();
    println!();

    generics();
    println!();

    advanced_trait_objects();
    println!();

    associated_types();
    println!();

    newtype_pattern();
    println!();

    advanced_trait_bounds();
    println!();

    type_level_programming();
    println!();

    composition_patterns();

    println!("\n✅ 所有类型系统示例运行完成！");
}

/// 演示高级类型模式：类型级编程
pub fn advanced_type_patterns() {
    println!("🚀 高级类型模式：");

    // 场景1：Phantom类型参数用于标记类型
    use std::marker::PhantomData;

    #[derive(Debug)]
    struct Container<T> {
        data: Vec<T>,
        _marker: PhantomData<T>,
    }

    impl<T> Container<T> {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                _marker: PhantomData,
            }
        }

        fn push(&mut self, item: T) {
            self.data.push(item);
        }

        fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    // 类型标记示例
    #[derive(Debug)]
    struct Empty;
    #[derive(Debug)]
    struct Populated;

    struct StateContainer<T, State> {
        data: Vec<T>,
        _state: PhantomData<State>,
    }

    impl<T, State> StateContainer<T, State> {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                _state: PhantomData,
            }
        }
    }

    impl<T> StateContainer<T, Populated> {
        fn has_data(&self) -> bool {
            !self.data.is_empty()
        }
    }

    let empty_container: StateContainer<String, Empty> = StateContainer::new();
    let mut full_container: StateContainer<i32, Populated> = StateContainer::new();

    full_container.data.push(42);
    full_container.data.push(24);

    println!("📦 空容器长度: {}", empty_container.data.len());
    println!(
        "📦 满容器长度: {}, 有数据: {}",
        full_container.data.len(),
        full_container.has_data()
    );

    // 场景2：类型级别的数值计算
    trait TypeAdd<Rhs = Self> {
        type Output;
        fn type_add(self, rhs: Rhs) -> Self::Output;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct One;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Two;
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Three;

    impl TypeAdd for One {
        type Output = Two;
        fn type_add(self, _rhs: One) -> Self::Output {
            Two
        }
    }

    impl TypeAdd for Two {
        type Output = Three;
        fn type_add(self, _rhs: Two) -> Self::Output {
            Three
        }
    }

    let one = One;
    let result = one.type_add(one);
    println!("🔢 1 + 1 = {:?}", result);

    println!("📊 高级类型模式演示完成");
}

/// 演示类型安全的API设计模式
pub fn type_safe_api_patterns() {
    println!("🛡️ 类型安全的API设计模式：");

    // 场景1：防止状态混淆的类型标记（简化版）
    #[derive(Debug, Clone)]
    struct DraftState;
    #[derive(Debug, Clone)]
    struct PublishedState;

    struct SimpleDocument<State> {
        content: String,
        _state: std::marker::PhantomData<State>,
    }

    impl SimpleDocument<PublishedState> {
        fn get_content(&self) -> &str {
            &self.content
        }
    }

    impl<State> SimpleDocument<State> {
        fn new() -> Self {
            Self {
                content: String::new(),
                _state: std::marker::PhantomData,
            }
        }

        fn add_content(&mut self, text: &str) {
            self.content.push_str(text);
            println!("📝 添加内容: {}", text);
        }
    }

    let mut document = SimpleDocument::<DraftState>::new();
    document.add_content("这是一个示例文档。");

    println!("📄 草稿内容: {}", document.content);

    // 场景2：简化的类型级长度验证
    trait Length {
        const LEN: usize;
    }

    struct Len5;
    struct Len10;

    impl Length for Len5 {
        const LEN: usize = 5;
    }

    impl Length for Len10 {
        const LEN: usize = 10;
    }

    // 简化的定长数组容器
    struct FixedLengthContainer<L: Length> {
        _marker: std::marker::PhantomData<L>,
    }

    impl<L: Length> FixedLengthContainer<L> {
        fn new() -> Self {
            Self {
                _marker: std::marker::PhantomData,
            }
        }

        fn len(&self) -> usize {
            L::LEN
        }

        fn validate_size(&self, data: &[i32]) -> Result<(), String> {
            if data.len() == L::LEN {
                Ok(())
            } else {
                Err(format!("期望长度 {}, 实际长度 {}", L::LEN, data.len()))
            }
        }
    }

    let container5 = FixedLengthContainer::<Len5>::new();
    let container10 = FixedLengthContainer::<Len10>::new();

    println!("📏 长度5容器容量: {}", container5.len());
    println!("📏 长度10容器容量: {}", container10.len());

    // 验证数组长度
    let data5 = vec![1, 2, 3, 4, 5];
    let data3 = vec![1, 2, 3];

    match container5.validate_size(&data5) {
        Ok(()) => println!("✅ 长度验证通过: {:?}", data5),
        Err(e) => println!("❌ 验证失败: {}", e),
    }

    match container5.validate_size(&data3) {
        Ok(()) => println!("✅ 长度验证通过"),
        Err(e) => println!("❌ 验证失败: {}", e),
    }

    // 场景3：单位类型防止误用
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Meters(f64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Kilometers(f64);

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Seconds(f64);

    impl Meters {
        fn to_kilometers(self) -> Kilometers {
            Kilometers(self.0 / 1000.0)
        }
    }

    impl Kilometers {
        fn to_meters(self) -> Meters {
            Meters(self.0 * 1000.0)
        }
    }

    let distance = Meters(5000.0);
    let distance_km = distance.to_kilometers();

    println!("📏 距离: {:.1} 米 = {:.1} 公里", distance.0, distance_km.0);

    // 防止单位混淆 - 这行会编译错误
    // let time = Seconds(100.0);
    // let _ = distance + time; // 编译错误！不能相加

    println!("📊 类型安全API设计模式演示完成");
}

/// 运行高级类型系统示例
pub fn run_advanced_types_examples() {
    println!("🎯 === 高级类型系统示例 ===");
    println!();

    println!("=== 基础类型系统示例 ===");
    run_types_examples();
    println!();

    println!("=== 高级类型模式 ===");
    advanced_type_patterns();
    println!();

    println!("=== 类型安全API设计模式 ===");
    type_safe_api_patterns();

    println!("\n✅ 所有高级类型系统示例运行完成！");
}
