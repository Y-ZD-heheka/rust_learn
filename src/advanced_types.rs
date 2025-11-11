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
        (self.max - self.current, Some(self.max))
    }
}

/// 现代化关联类型使用
pub fn modern_associated_types() {
    println!("🔄 现代化关联类型：");
    
    let mut counter = ModernCounter::new(5);
    while let Some(value) = counter.next() {
        println!("计数器: {}", value);
    }
    
    let hint = counter.size_hint();
    println!("大小提示: {:?}", hint);
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
        fn get_color(&self) -> Self::Color;
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
                item, item.clone(), item == item)
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