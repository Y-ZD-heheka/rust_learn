//! # 所有权系统模块
//!
//! 这个模块演示了Rust的所有权系统，包括所有权、借用和生命周期的概念。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::fmt;

/// 演示现代所有权基本概念
pub fn ownership_basics() {
    println!("🏠 所有权基础概念：");
    
    // 字符串字面量是不可变的，存储在栈上
    let s1: &str = "hello";
    println!("字符串切片 s1: {}", s1);

    // String 类型存储在堆上，可变
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("可变字符串 s2: {}", s2);

    // 所有权转移
    let s3 = s2; // s2 的所有权转移给 s3，s2 不再有效
    // println!("s2: {}", s2); // 这行会编译错误
    println!("接收所有权 s3: {}", s3);

    // 现代移动语义示例
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1; // 所有权移动
    // println!("vec1: {:?}", vec1); // 这会编译错误
    println!("vec2: {:?}", vec2);
}

/// 演示现代化借用和引用
pub fn borrowing() {
    println!("🔗 现代化借用和引用：");
    
    let s1 = String::from("hello rust");

    // 不可变借用 - 现代化函数签名
    let len = calculate_length(&s1);
    println!("字符串'{}'的长度: {}", s1, len);

    // 可变借用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("修改后的字符串: {}", s2);

    // 现代借用规则演示 - 多个不可变引用
    let r1 = &s2;
    let r2 = &s2;
    let r3 = &s2; // Rust 2021允许更多不可变引用
    println!("多个不可变引用: {}, {}, {}", r1, r2, r3);

    // 借用检查的作用域演示
    {
        let r4 = &s2; // 新的不可变引用作用域
        println!("作用域内的引用: {}", r4);
    } // r4 在这里被丢弃

    // 现在可以创建可变引用了
    let mut s3 = String::from("mutable");
    append_text(&mut s3);
    println!("可变借用后: {}", s3);
}

/// 使用现代化生命周期语法计算字符串长度
fn calculate_length(s: &str) -> usize {
    s.len()
}

/// 现代化可变借用函数
fn change(s: &mut String) {
    s.push_str(", world");
}

/// 追加文本的现代函数
fn append_text(s: &mut String) {
    s.push_str(" - modified");
}

/// 演示现代化切片操作
pub fn slices() {
    println!("✂️ 现代化切片操作：");
    
    let s = String::from("hello world rust programming");
    
    // 字符串切片的现代方法
    let words: Vec<&str> = s.split_whitespace().collect();
    let first_word = words.get(0).copied().unwrap_or("");
    let last_word = words.last().copied().unwrap_or("");
    
    println!("原字符串: {}", s);
    println!("第一个词: '{}', 最后一个词: '{}'", first_word, last_word);
    
    // 使用切片索引
    let hello = &s[..5];
    let world = &s[6..11];
    let rust = &s[12..];
    
    println!("切片演示: '{}', '{}', '{}'", hello, world, rust);
    
    // 数组切片的现代化操作
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let middle_slice = &arr[3..7];
    let even_numbers: Vec<_> = arr
        .iter()
        .enumerate()
        .filter(|(_, x)| *x % 2 == 0)
        .map(|(i, x)| (i, *x))
        .collect();
    
    println!("中间切片: {:?}", middle_slice);
    println!("偶数及其索引: {:?}", even_numbers);
}

/// 演示高级生命周期和泛型
pub fn advanced_lifetimes() {
    println!("⏰ 高级生命周期和泛型：");
    
    // 现代化泛型函数，使用trait bound
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        announcement: T,
    ) -> &'a str
    where
        T: fmt::Display,
    {
        println!("公告: {}", announcement);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        &string1,
        string2,
        "最长字符串是..."
    );
    
    println!("结果是: {}", result);
}

/// 演示现代Box和智能指针
pub fn modern_pointers() {
    println!("🎯 现代智能指针：");
    
    // 使用Box进行堆分配
    let boxed_value = Box::new(42);
    println!("Box中的值: {}", boxed_value);
    
    // Box的所有权转移
    let transferred_box = boxed_value; // 移动所有权
    // println!("boxed_value: {}", boxed_value); // 编译错误
    println!("转移后的Box值: {}", transferred_box);
    
    // 现代递归结构
    #[derive(Debug)]
    #[allow(dead_code)]
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    
    let list = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil)))
        ))
    );
    
    println!("递归列表: {:?}", list);
}

/// 演示复杂的所有权管理场景
pub fn complex_ownership_scenarios() {
    println!("🔄 复杂所有权管理场景：");
    
    // 场景1：文件句柄和资源管理
    #[derive(Debug)]
    pub struct FileHandle {
        pub filename: String,
        content: Option<String>,
    }
    
    impl FileHandle {
        pub fn new(filename: &str) -> Self {
            println!("📁 打开文件: {}", filename);
            Self {
                filename: filename.to_string(),
                content: None,
            }
        }
        
        pub fn read_content(&mut self) -> Result<&str, String> {
            if let Some(ref content) = self.content {
                Ok(content)
            } else {
                // 模拟文件读取
                let content = format!("这是文件 {} 的内容", self.filename);
                self.content = Some(content);
                Ok(self.content.as_ref().unwrap())
            }
        }
        
        pub fn write_content(&mut self, content: &str) -> Result<(), String> {
            self.content = Some(content.to_string());
            println!("✏️ 文件 {} 内容已更新", self.filename);
            Ok(())
        }
    }
    
    // 演示文件句柄的所有权转移
    fn process_file(mut file: FileHandle) -> Result<String, String> {
        file.write_content("处理后的内容")?;
        file.read_content().map(|s| s.to_string())
    }
    
    let file_handle = FileHandle::new("test.txt");
    match process_file(file_handle) {
        Ok(content) => println!("处理结果: {}", content),
        Err(e) => println!("处理失败: {}", e),
    }
    
    // 场景2：图书管理系统
    #[derive(Debug, Clone)]
    pub struct Book {
        pub title: String,
        pub author: String,
        pub isbn: String,
    }
    
    impl Book {
        pub fn new(title: &str, author: &str, isbn: &str) -> Self {
            Self {
                title: title.to_string(),
                author: author.to_string(),
                isbn: isbn.to_string(),
            }
        }
    }
    
    #[derive(Debug)]
    pub struct Library {
        books: Vec<Book>,
    }
    
    impl Library {
        pub fn new() -> Self {
            Self { books: Vec::new() }
        }
        
        pub fn add_book(&mut self, book: Book) {
            self.books.push(book);
            println!("📚 添加图书: {}", self.books.last().unwrap().title);
        }
        
        pub fn find_book_by_isbn(&self, isbn: &str) -> Option<&Book> {
            self.books.iter().find(|book| book.isbn == isbn)
        }
        
        pub fn find_book_by_author(&self, author: &str) -> Vec<&Book> {
            self.books.iter().filter(|book| book.author.contains(author)).collect()
        }
    }
    
    let mut library = Library::new();
    
    // 创建图书并移动所有权给图书馆
    let book1 = Book::new("Rust编程之道", "张三", "978-7-115-12345");
    let book2 = Book::new("Rust权威指南", "李四", "978-7-115-12346");
    
    library.add_book(book1); // 所有权转移
    library.add_book(book2); // 所有权转移
    
    // 查找图书（返回引用，不转移所有权）
    if let Some(book) = library.find_book_by_isbn("978-7-115-12345") {
        println!("找到图书: {} by {}", book.title, book.author);
    }
    
    // 批量查找
    let rust_books = library.find_book_by_author("张三");
    println!("张三的图书数量: {}", rust_books.len());
}

/// 演示智能指针的复杂使用
pub fn advanced_smart_pointers() {
    println!("🎯 高级智能指针：");
    
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::Mutex;
    
    // 场景1：Rc 用于单线程共享（简化版本）
    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
    }
    
    impl Node {
        pub fn new(value: i32) -> Self {
            Self { value }
        }
    }
    
    let root = Node::new(1);
    
    println!("根节点值: {}", root.value);
    
    // 使用Rc创建共享引用示例
    let shared_node = Rc::new(Node::new(10));
    let _node_clone1 = Rc::clone(&shared_node);
    let _node_clone2 = Rc::clone(&shared_node);
    
    println!("共享节点值: {}, 引用计数: {}", shared_node.value, Rc::strong_count(&shared_node));
    println!("引用计数验证: {}", Rc::strong_count(&_node_clone1));
    
    // 场景2：Arc + Mutex 用于多线程共享
    #[derive(Debug, Clone)]
    pub struct SharedCounter {
        pub count: Arc<Mutex<i32>>,
    }
    
    impl SharedCounter {
        pub fn new() -> Self {
            Self {
                count: Arc::new(Mutex::new(0)),
            }
        }
        
        pub fn increment(&self) {
            let mut count = self.count.lock().unwrap();
            *count += 1;
            println!("计数器增加到: {}", *count);
        }
        
        pub fn get_count(&self) -> i32 {
            let count = self.count.lock().unwrap();
            *count
        }
    }
    
    let counter = SharedCounter::new();
    counter.increment();
    counter.increment();
    println!("当前计数: {}", counter.get_count());
    
    // 场景3：自定义智能指针
    #[derive(Debug)]
    pub struct CustomBox<T> {
        data: Box<T>,
    }
    
    impl<T> CustomBox<T> {
        pub fn new(data: T) -> Self {
            Self {
                data: Box::new(data),
            }
        }
        
        pub fn get_ref(&self) -> &T {
            &self.data
        }
        
        pub fn get_mut(&mut self) -> &mut T {
            &mut self.data
        }
    }
    
    let mut custom_box = CustomBox::new(42);
    *custom_box.get_mut() *= 2;
    println!("自定义Box值: {}", custom_box.get_ref());
}

/// 演示复杂的生命周期场景
pub fn complex_lifetime_scenarios() {
    println!("⏰ 复杂生命周期场景：");
    
    // 场景1：生命周期参数在不同位置的使用
    struct Parser<'a> {
        text: &'a str,
        position: usize,
    }
    
    impl<'a> Parser<'a> {
        fn new(text: &'a str) -> Self {
            Self { text, position: 0 }
        }
        
        fn parse_word(&mut self) -> Option<&'a str> {
            let start = self.position;
            while let Some(&ch) = self.text.as_bytes().get(self.position) {
                if ch.is_ascii_whitespace() {
                    break;
                }
                self.position += 1;
            }
            
            if start == self.position {
                None
            } else {
                Some(&self.text[start..self.position])
            }
        }
    }
    
    let text = "Hello Rust World";
    let mut parser = Parser::new(text);
    
    while let Some(word) = parser.parse_word() {
        println!("解析到词: {}", word);
    }
    
    // 场景2：静态生命周期常量
    static ERROR_MESSAGES: &[&'static str] = &[
        "连接失败",
        "认证错误",
        "网络超时",
    ];
    
    fn get_error_message(code: usize) -> Option<&'static str> {
        ERROR_MESSAGES.get(code).copied()
    }
    
    if let Some(msg) = get_error_message(0) {
        println!("错误消息: {}", msg);
    }
    
    // 场景3：生命周期参数与泛型的交互
    #[derive(Debug)]
    struct Container<'a, T> {
        data: &'a T,
    }
    
    impl<'a, T> Container<'a, T> {
        fn new(data: &'a T) -> Self {
            Self { data }
        }
        
        fn get_data(&self) -> &'a T {
            self.data
        }
    }
    
    let value = 42;
    let container = Container::new(&value);
    println!("容器中的值: {}", container.get_data());
    
    // 场景4：借用检查器的高级场景
    let mut numbers = vec![1, 2, 3, 4, 5];
    
    // 不可变借用
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("第一个偶数: {:?}", first_even);
    
    // 可变借用（在不可变借用之后）
    if let Some(index) = numbers.iter().position(|&x| x == 4) {
        numbers[index] = 40; // 可以修改，因为第一阶段借用已结束
        println!("修改后的数组: {:?}", numbers);
    }
}

/// 演示借用检查器的实际应用
pub fn borrowing_checker_applications() {
    println!("🔍 借用检查器应用：");
    
    // 场景1：常见的借用错误及解决方案
    let mut data = vec![1, 2, 3, 4, 5];
    
    // ❌ 错误示例：同时存在可变和不可变借用
    // let first = &data[0];
    // data.push(6); // 编译错误：同时存在借用
    
    // ✅ 正确解决方案
    let first = data[0]; // 先使用值
    data.push(6); // 然后可以修改
    println!("第一个元素: {}, 修改后: {:?}", first, data);
    
    // 场景2：借用生命周期的实际控制
    #[derive(Debug)]
    struct TempData<'a> {
        data: &'a mut Vec<i32>,
    }
    
    impl<'a> TempData<'a> {
        fn process(&mut self) {
            self.data.push(99);
            println!("处理后数据: {:?}", self.data);
        }
    }
    
    let mut vec = vec![1, 2, 3];
    let mut temp_data = TempData { data: &mut vec };
    temp_data.process(); // 借用生命周期结束
    
    // 这里可以再次创建新的借用
    vec.clear();
    vec.push(7);
    println!("清理后: {:?}", vec);
    
    // 场景3：切片和借用
    let matrix = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let row_len = 3;
    
    // 创建多个不重叠的借用
    let row1 = &matrix[0..row_len];
    let row2 = &matrix[row_len..row_len*2];
    let row3 = &matrix[row_len*2..row_len*3];
    
    println!("第1行: {:?}", row1);
    println!("第2行: {:?}", row2);
    println!("第3行: {:?}", row3);
    
    // 计算矩阵的转置（演示复杂的借用关系）
    let mut transposed = vec![0; 9];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j * 3 + i] = matrix[i * 3 + j];
        }
    }
    println!("转置矩阵: {:?}", transposed);
}

/// 运行所有权系统示例
pub fn run_ownership_examples() {
    println!("🎯 === 现代化所有权系统示例 ===");
    println!();
    
    ownership_basics();
    println!();
    
    borrowing();
    println!();
    
    slices();
    println!();
    
    advanced_lifetimes();
    println!();
    
    modern_pointers();
    println!();
    
    complex_ownership_scenarios();
    println!();
    
    advanced_smart_pointers();
    println!();
    
    complex_lifetime_scenarios();
    println!();
    
    borrowing_checker_applications();
    
    println!("\n✅ 所有所有权示例运行完成！");
}

/// 演示复杂图结构的所有权模式
pub fn complex_graph_ownership() {
    println!("📊 复杂图结构的所有权模式：");
    
    use std::collections::HashMap;
    use std::rc::{Rc, Weak};
    
    // 图节点结构体
    #[derive(Debug)]
    #[allow(dead_code)]
    struct GraphNode {
        #[allow(dead_code)]
        id: u32,
        #[allow(dead_code)]
        value: String,
        #[allow(dead_code)]
        neighbors: Vec<Weak<Node>>,
    }
    
    type Node = Rc<GraphNode>;
    
    impl GraphNode {
        fn new(id: u32, value: String) -> Self {
            Self {
                id,
                value,
                neighbors: Vec::new(),
            }
        }
        
        #[allow(dead_code)]
        fn add_neighbor(&self, _neighbor: &Node) {
            // 这里需要不同的方法，因为不能修改 Rc 的内容
            println!("⚠️ 不能直接修改Rc内容，演示循环引用问题");
        }
    }
    
    // 图结构管理器
    struct Graph {
        nodes: HashMap<u32, Node>,
    }
    
    impl Graph {
        fn new() -> Self {
            Self {
                nodes: HashMap::new(),
            }
        }
        
        fn add_node(&mut self, id: u32, value: String) -> Node {
            let node = Rc::new(GraphNode::new(id, value));
            self.nodes.insert(id, Rc::clone(&node));
            node
        }
        
        fn add_edge(&mut self, from_id: u32, to_id: u32) -> Result<(), String> {
            let _from_node = self.nodes.get(&from_id)
                .ok_or_else(|| format!("节点 {} 不存在", from_id))?;
            let _to_node = self.nodes.get(&to_id)
                .ok_or_else(|| format!("节点 {} 不存在", to_id))?;
            
            // 模拟添加边（实际中需要不同的设计）
            println!("📎 在节点 {} 和 {} 之间添加边", from_id, to_id);
            Ok(())
        }
        
        fn get_node(&self, id: u32) -> Option<&Node> {
            self.nodes.get(&id)
        }
    }
    
    // 演示图结构
    let mut graph = Graph::new();
    
    let _node1 = graph.add_node(1, "节点1".to_string());
    let _node2 = graph.add_node(2, "节点2".to_string());
    let _node3 = graph.add_node(3, "节点3".to_string());
    
    println!("📝 创建了 {} 个节点", graph.nodes.len());
    
    // 添加边
    let _ = graph.add_edge(1, 2);
    let _ = graph.add_edge(2, 3);
    let _ = graph.add_edge(3, 1); // 形成环
    
    // 访问节点
    if let Some(node) = graph.get_node(1) {
        println!("🔍 节点1信息: {:?}", node);
    }
    
    println!("📊 复杂图结构演示完成");
}

/// 演示高级借用检查场景
pub fn advanced_borrowing_scenarios() {
    println!("🔗 高级借用检查场景：");
    
    // 场景1: 多个不可变借用
    let mut data = vec![1, 2, 3, 4, 5];
    
    // 创建一个切片引用
    let slice1 = &data;
    let slice2 = &data; // 多个不可变引用是允许的
    let slice3 = &data;
    
    println!("🔍 多个不可变引用:");
    println!("  slice1: {:?}", slice1);
    println!("  slice2: {:?}", slice2);
    println!("  slice3: {:?}", slice3);
    
    // 场景2: 借用检查器的作用域
    {
        let _immutable_borrow = &data;
        // 这里不能进行可变借用
        // data.push(6); // 这会编译错误
        
        println!("📍 在不可变借用作用域内");
    } // _immutable_borrow 在这里被释放
    
    // 现在可以进行可变借用了
    data.push(6);
    println!("✅ 添加元素后: {:?}", data);
    
    // 场景3: 借用检查和函数调用
    fn process_slice(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }
    
    fn modify_vector(vec: &mut [i32]) {
        for item in vec.iter_mut() {
            *item *= 2;
        }
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 先进行不可变借用
    let sum = process_slice(&numbers);
    println!("🔢 数字之和: {}", sum);
    
    // 再进行可变借用
    let mut numbers_mut = numbers.clone(); // 克隆以避免借用冲突
    modify_vector(&mut numbers_mut);
    println!("🔄 修改后的数字: {:?}", numbers_mut);
    
    println!("📊 高级借用检查场景演示完成");
}

/// 演示内存管理和性能优化
pub fn memory_management_optimization() {
    println!("⚡ 内存管理和性能优化：");
    
    use std::mem::{size_of, align_of};
    
    // 场景1: Box vs 堆分配
    fn stack_vs_heap() {
        // 栈分配 (快速)
        let small_data = [1u8; 64];
        println!("📊 栈数据大小: {} 字节", size_of_val(&small_data));
        
        // 堆分配 (较慢但适合大数据)
        let large_data = Box::new([0u8; 10000]);
        println!("📊 堆数据大小: {} 字节", size_of_val(&large_data));
    }
    
    stack_vs_heap();
    
    // 场景2: 引用计数 vs 克隆
    use std::rc::Rc;
    
    let data1 = Rc::new(vec![1, 2, 3, 4, 5]);
    let _data2 = Rc::clone(&data1); // 增加引用计数
    
    println!("🔢 引用计数: {}", Rc::strong_count(&data1));
    
    {
        let _data3 = Rc::clone(&data1);
        println!("🔢 引用计数（内部）: {}", Rc::strong_count(&data1));
    } // _data3 离开作用域，引用计数减少
    
    println!("🔢 引用计数（外部）: {}", Rc::strong_count(&data1));
    
    // 场景3: 内存布局优化
    #[derive(Debug)]
    #[repr(C)] // 使用C布局以获得更好的内存对齐
    struct OptimizedData {
        a: u8,      // 1字节
        b: u64,     // 8字节 (会在64位边界对齐)
        c: u32,     // 4字节
        d: u16,     // 2字节
        e: u8,      // 1字节
    }
    
    let _opt_data = OptimizedData {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };
    
    println!("📊 优化数据大小: {} 字节", size_of::<OptimizedData>());
    println!("📊 对齐要求: {} 字节", align_of::<OptimizedData>());
    
    // 场景4: 零成本抽象
    fn process_iter(items: &[i32]) -> i32 {
        items
            .iter()
            .filter(|&&x| x % 2 == 0) // 过滤偶数
            .map(|&x| x * x)         // 平方
            .sum()                   // 求和
    }
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = process_iter(&numbers);
    println!("🔢 零成本抽象处理结果: {}", result);
    
    println!("📊 内存管理和性能优化演示完成");
}

/// 运行复杂所有权场景示例
pub fn run_complex_ownership_examples() {
    println!("🎯 === 复杂所有权场景示例 ===");
    println!();
    
    println!("=== 基础所有权示例 ===");
    run_ownership_examples();
    println!();
    
    println!("=== 复杂图结构所有权 ===");
    complex_graph_ownership();
    println!();
    
    println!("=== 高级借用检查场景 ===");
    advanced_borrowing_scenarios();
    println!();
    
    println!("=== 内存管理和性能优化 ===");
    memory_management_optimization();
    
    println!("\n✅ 所有复杂所有权场景示例运行完成！");
}