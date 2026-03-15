//! # 宏和元编程模块
//!
//! 这个模块演示了Rust的宏系统和元编程能力。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

/// 现代化声明宏示例
macro_rules! say_hello {
    () => {
        println!("🎯 你好！");
    };
    ($name:expr) => {
        println!("🎯 你好，{}！", $name);
    };
    ($($name:expr),+ $(,)?) => {
        $(
            println!("🎯 你好，{}！", $name);
        )+
    };
}

/// 现代化字符串处理宏
macro_rules! format_greeting {
    (name: $name:expr, age: $age:expr) => {
        format!("👤 姓名: {}, 年龄: {} 岁", $name, $age)
    };
    (name: $name:expr) => {
        format!("👤 姓名: {}", $name)
    };
    ($($key:ident: $value:expr),+ $(,)?) => {
        {
            let mut result = String::new();
            $(
                result.push_str(&format!("{}: {}, ", stringify!($key), $value));
            )+
            result
        }
    };
}

/// 现代化向量操作宏
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// 现代化数学操作宏
macro_rules! math_operations {
    ($($op:ident: $a:expr, $b:expr);+ $(;)?) => {
        {
            let mut results = Vec::new();
            $(
                let result = match stringify!($op) {
                    "add" => $a + $b,
                    "sub" => $a - $b,
                    "mul" => $a * $b,
                    "div" => $a / $b,
                    _ => 0,
                };
                results.push((stringify!($op), result));
            )+
            results
        }
    };
}

/// 现代化条件宏
macro_rules! debug_println {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            println!("🐛 [DEBUG] {}", format_args!($($arg)*));
        }
        #[cfg(not(debug_assertions))]
        {
        }
    }};
}

/// 现代化类型安全的宏
macro_rules! create_point {
    (x: $x:expr, y: $y:expr) => {
        Point { x: $x, y: $y }
    };
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

/// 现代化声明宏使用
pub fn modern_declarative_macros() {
    println!("🎯 现代化声明宏：");

    // 基本使用
    say_hello!();
    say_hello!("Rust 2024");
    say_hello!("Alice", "Bob", "Charlie");

    println!();

    // 字符串格式化
    let greeting1 = format_greeting!(name: "张三", age: 25);
    let greeting2 = format_greeting!(name: "李四");
    let greeting3 = format_greeting!(name: "王五", age: 30, city: "北京");

    println!("{}", greeting1);
    println!("{}", greeting2);
    println!("{}", greeting3);

    println!();

    // 调试输出
    debug_println!("当前变量值为: {}", 42);
    debug_println!("复杂计算: {} + {} = {}", 10, 20, 30);
}

/// 现代化自定义宏
pub fn modern_custom_macros() {
    println!("🔧 现代化自定义宏：");

    // 现代化向量操作
    let numbers = my_vec![1, 2, 3, 4, 5];
    let strings = my_vec!["hello", "rust", "world"];
    let floats = my_vec![1.0, 2.0, 3.0];

    println!("数字向量: {:?}", numbers);
    println!("字符串向量: {:?}", strings);
    println!("浮点数向量: {:?}", floats);

    // 数学操作宏
    let ops = math_operations!(
        add: 10, 5;
        sub: 10, 5;
        mul: 10, 5;
        div: 10, 5;
    );

    println!("数学运算结果:");
    for (op, result) in ops {
        println!("  {}: {}", op, result);
    }
}

/// 现代化派生宏示例
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 现代化过程宏演示
pub fn modern_procedural_macros_demo() {
    println!("⚙️ 现代化过程宏演示：");

    // 演示派生宏
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 4.0, y: 6.0 };

    println!("点1: {}", p1);
    println!("点2: {}", p2);
    println!("点1 Debug: {:?}", p1);
    println!("点2 Clone: {:?}", p2.clone());
    println!("相等检查: {}", p1 == p2);

    // 使用宏创建点
    let p3 = create_point!(x: 10.0, y: 20.0);
    let p4 = create_point!(5.0, 15.0);

    println!("点3: {}", p3);
    println!("点4: {}", p4);
    println!("点3到点4距离: {:.2}", p3.distance(&p4));
}

/// 现代化宏卫生性和作用域
macro_rules! scoped_function {
    ($func_name:ident, $message:expr) => {
        fn $func_name() {
            println!("🔧 {}: {}", stringify!($func_name), $message);
        }
    };
}

macro_rules! data_structure {
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)? }) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct $name {
            $(
                pub $field: $ty,
            )*
        }
    };
    (enum $name:ident { $( $variant:ident $(($($field:ty),*))? ),* $(,)? }) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub enum $name {
            $(
                $variant $(($( $field ),*))?,
            )*
        }
    };
}

/// 现代化宏卫生性演示
pub fn modern_macro_hygiene() {
    println!("🧹 现代化宏卫生性：");

    // 作用域内的函数创建
    scoped_function!(greet_user, "用户问候");
    scoped_function!(show_banner, "显示横幅");

    greet_user();
    show_banner();

    // 数据结构宏
    data_structure!(
        struct User {
            name: String,
            age: u32,
            email: String,
        }
    );

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    println!("用户信息: {:?}", user);

    // 枚举宏
    data_structure!(
        enum Status {
            Active,
            Inactive,
            Pending(String),
        }
    );

    let status = Status::Active;
    let pending_status = Status::Pending("等待审核".to_string());

    println!("状态: {:?}", status);
    println!("待处理: {:?}", pending_status);
}

/// 现代化重复模式宏
macro_rules! generate_variants {
    (enum $name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        pub enum $name {
            $(
                $variant,
            )*
        }

        #[allow(dead_code)]
        #[allow(non_snake_case)]
        impl $name {
            $(
                pub fn $variant() -> Self {
                    Self::$variant
                }
            )*

            pub fn all_variants() -> &'static [Self] {
                &[
                    $( Self::$variant, )*
                ]
            }
        }
    };
}

/// 现代化重复模式演示
pub fn modern_repetition_patterns() {
    println!("🔄 现代化重复模式：");

    generate_variants!(
        enum Color {
            Red,
            Green,
            Blue,
            Yellow,
        }
    );

    // 使用生成的变体
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    let yellow = Color::Yellow;

    println!("颜色变体: {:?}", [red, green, blue, yellow]);

    // 使用all_variants方法
    let all_colors = Color::all_variants();
    println!("所有颜色:");
    for (i, color) in all_colors.iter().enumerate() {
        println!("  {}: {:?}", i, color);
    }
}

/// 演示HTML构建DSL
pub fn html_builder_dsl() {
    println!("🌐 HTML构建DSL：");

    fn escape_html(raw: &str) -> String {
        raw.chars()
            .flat_map(|ch| match ch {
                '&' => "&amp;".chars().collect::<Vec<_>>(),
                '<' => "&lt;".chars().collect::<Vec<_>>(),
                '>' => "&gt;".chars().collect::<Vec<_>>(),
                '"' => "&quot;".chars().collect::<Vec<_>>(),
                '\'' => "&#39;".chars().collect::<Vec<_>>(),
                _ => vec![ch],
            })
            .collect()
    }

    // HTML构建器结构体
    #[derive(Debug)]
    pub struct HtmlElement {
        tag: String,
        content: Vec<HtmlContent>,
        attributes: std::collections::HashMap<String, String>,
    }

    #[derive(Debug)]
    pub enum HtmlContent {
        Text(String),
        Element(Box<HtmlElement>),
    }

    impl HtmlElement {
        pub fn new(tag: &str) -> Self {
            Self {
                tag: tag.to_string(),
                content: Vec::new(),
                attributes: std::collections::HashMap::new(),
            }
        }

        pub fn text(mut self, text: &str) -> Self {
            self.content.push(HtmlContent::Text(text.to_string()));
            self
        }

        pub fn child(mut self, element: HtmlElement) -> Self {
            self.content.push(HtmlContent::Element(Box::new(element)));
            self
        }

        pub fn attr(mut self, name: &str, value: &str) -> Self {
            self.attributes.insert(name.to_string(), value.to_string());
            self
        }

        pub fn render(&self) -> String {
            let mut html = String::new();

            // 教学示例仅对文本节点和属性值做转义。
            // 标签名与属性名仍视为示例作者提供的受信输入，不作为通用模板引擎承诺。
            html.push_str(&format!("<{}", self.tag));

            for (name, value) in &self.attributes {
                html.push_str(&format!(" {}=\"{}\"", name, escape_html(value)));
            }

            html.push('>');

            for content in &self.content {
                match content {
                    HtmlContent::Text(text) => html.push_str(&escape_html(text)),
                    HtmlContent::Element(element) => html.push_str(&element.render()),
                }
            }

            html.push_str(&format!("</{}>", self.tag));
            html
        }
    }

    // HTML DSL宏
    #[allow(unused_macros)]
    macro_rules! html_div {
        ($text:expr) => {
            HtmlElement::new("div").text($text)
        };
        (attr: $attr:expr, value: $val:expr, text: $text:expr) => {
            HtmlElement::new("div").attr($attr, $val).text($text)
        };
    }

    #[allow(dead_code)]
    macro_rules! html_p {
        ($text:expr) => {
            HtmlElement::new("p").text($text)
        };
    }

    #[allow(dead_code)]
    macro_rules! html_h1 {
        ($text:expr) => {
            HtmlElement::new("h1").text($text)
        };
    }

    // 使用HTML DSL
    println!("🎨 构建HTML页面:");

    let title = html_h1!("现代化Rust Web应用")
        .attr("class", "title")
        .attr("id", "main-title");

    let paragraph = html_p!("这是一个使用Rust宏构建的HTML页面示例。")
        .attr("class", "description")
        .attr("data-note", "<unsafe> \"quoted\" & raw text");

    let escaped_example = html_div!("用户输入: <script>alert('xss')</script> & \"quoted\"")
        .attr("class", "example")
        .attr("title", "5 > 3 && 2 < 4");

    let header = HtmlElement::new("header").child(title);

    let main = HtmlElement::new("main")
        .child(paragraph)
        .child(escaped_example);

    let page = HtmlElement::new("html")
        .attr("lang", "zh-CN")
        .child(header)
        .child(main);

    println!("✅ 生成的HTML（文本与属性值已转义）:");
    println!("{}", page.render());

    println!("📊 HTML DSL演示完成");
}

/// 演示配置管理DSL
pub fn configuration_dsl() {
    println!("⚙️ 配置管理DSL：");

    // 配置结构体
    #[derive(Debug, Clone)]
    pub struct Config {
        pub database: DatabaseConfig,
        pub server: ServerConfig,
    }

    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct DatabaseConfig {
        pub host: String,
        pub port: u16,
        #[allow(dead_code)]
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct ServerConfig {
        pub host: String,
        pub port: u16,
        pub workers: u32,
    }

    // 配置DSL宏
    macro_rules! create_config {
        (development) => {{
            Config {
                database: DatabaseConfig {
                    host: "localhost".to_string(),
                    port: 5432,
                    name: "myapp_dev".to_string(),
                },
                server: ServerConfig {
                    host: "127.0.0.1".to_string(),
                    port: 3000,
                    workers: 2,
                },
            }
        }};
        (production) => {{
            Config {
                database: DatabaseConfig {
                    host: "prod-db.example.com".to_string(),
                    port: 5432,
                    name: "myapp_prod".to_string(),
                },
                server: ServerConfig {
                    host: "0.0.0.0".to_string(),
                    port: 8080,
                    workers: 8,
                },
            }
        }};
    }

    // 使用配置DSL
    println!("🎨 构建配置:");

    // 开发环境配置
    let dev_config = create_config!(development);
    println!("✅ 开发环境配置:");
    println!(
        "   数据库: {}:{}",
        dev_config.database.host, dev_config.database.port
    );
    println!(
        "   服务器: {}:{} ({} 工作线程)",
        dev_config.server.host, dev_config.server.port, dev_config.server.workers
    );

    // 生产环境配置
    let prod_config = create_config!(production);
    println!("✅ 生产环境配置:");
    println!(
        "   数据库: {}:{}",
        prod_config.database.host, prod_config.database.port
    );
    println!(
        "   服务器: {}:{} ({} 工作线程)",
        prod_config.server.host, prod_config.server.port, prod_config.server.workers
    );

    println!("📊 配置管理DSL演示完成");
}

/// 演示API路由DSL
pub fn api_routing_dsl() {
    println!("🛣️ API路由DSL：");

    // HTTP方法枚举
    #[derive(Debug, Clone)]
    pub enum HttpMethod {
        GET,
        POST,
        PUT,
        DELETE,
    }

    // 路由结构体
    #[derive(Debug)]
    pub struct Route {
        method: HttpMethod,
        path: String,
        handler: String,
    }

    impl Route {
        pub fn new(method: HttpMethod, path: &str, handler: &str) -> Self {
            Self {
                method,
                path: path.to_string(),
                handler: handler.to_string(),
            }
        }

        pub fn render(&self) -> String {
            format!(
                "{} {} -> {}",
                match self.method {
                    HttpMethod::GET => "GET",
                    HttpMethod::POST => "POST",
                    HttpMethod::PUT => "PUT",
                    HttpMethod::DELETE => "DELETE",
                },
                self.path,
                self.handler
            )
        }
    }

    // API路由DSL宏
    macro_rules! get_route {
        ($path:expr, $handler:expr) => {
            Route::new(HttpMethod::GET, $path, $handler)
        };
    }

    macro_rules! post_route {
        ($path:expr, $handler:expr) => {
            Route::new(HttpMethod::POST, $path, $handler)
        };
    }

    macro_rules! put_route {
        ($path:expr, $handler:expr) => {
            Route::new(HttpMethod::PUT, $path, $handler)
        };
    }

    macro_rules! delete_route {
        ($path:expr, $handler:expr) => {
            Route::new(HttpMethod::DELETE, $path, $handler)
        };
    }

    macro_rules! routes {
        ($($route:expr),* $(,)?) => {{
            vec![$($route),*]
        }};
    }

    // 使用API路由DSL
    println!("🎨 构建API路由:");

    let routes = routes![
        get_route!("/api/users", "get_users"),
        get_route!("/api/users/:id", "get_user_by_id"),
        post_route!("/api/users", "create_user"),
        put_route!("/api/users/:id", "update_user"),
        delete_route!("/api/users/:id", "delete_user"),
        get_route!("/api/health", "health_check"),
    ];

    println!("✅ 生成的路由:");
    for (i, route) in routes.iter().enumerate() {
        println!("   {}: {}", i + 1, route.render());
    }

    println!("📊 API路由DSL演示完成");
}

/// 运行宏和元编程示例
pub fn run_macros_examples() {
    println!("🎯 === 现代化宏和元编程示例 ===");
    println!();

    modern_declarative_macros();
    println!();

    modern_custom_macros();
    println!();

    modern_procedural_macros_demo();
    println!();

    modern_macro_hygiene();
    println!();

    modern_repetition_patterns();

    println!("\n✅ 所有宏和元编程示例运行完成！");
}

/// 演示复杂宏模式匹配
pub fn complex_macro_patterns() {
    println!("🎯 复杂宏模式匹配：");

    // 复杂条件宏
    macro_rules! match_type {
        ($val:expr, pattern => $pattern:expr) => {{
            let value = $val;
            let pattern = $pattern;
            match pattern {
                "string" => {
                    if let Some(s) = value.downcast_ref::<String>() {
                        println!("匹配到字符串: {}", s);
                        true
                    } else {
                        false
                    }
                }
                "number" => {
                    if let Some(n) = value.downcast_ref::<i32>() {
                        println!("匹配到数字: {}", n);
                        true
                    } else {
                        false
                    }
                }
                _ => {
                    println!("未知的模式类型: {}", pattern);
                    false
                }
            }
        }};
    }

    // 使用类型匹配宏
    let string_value: Box<dyn std::any::Any> = Box::new("Hello".to_string());
    let number_value: Box<dyn std::any::Any> = Box::new(42);

    println!("字符串匹配:");
    let _ = match_type!(string_value, pattern => "string");
    println!("数字匹配:");
    let _ = match_type!(number_value, pattern => "number");

    // 简化的嵌套向量宏
    macro_rules! nested_vec {
        ($($val:expr),+ $(,)?) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($val);
                )+
                temp_vec
            }
        };
    }

    // 使用嵌套向量宏
    println!("\n嵌套向量示例:");
    let flat = nested_vec![1, 2, 3, 4];
    println!("平面向量: {:?}", flat);

    // 模式识别宏
    macro_rules! identify_pattern {
        (all_even: [$($num:expr),* $(,)?]) => {
            {
                let mut all_even = true;
                $(
                    if $num % 2 != 0 {
                        all_even = false;
                    }
                )*
                all_even
            }
        };
        (sum_greater_than: [$($num:expr),* $(,)?], $threshold:expr) => {
            {
                let sum: i32 = $($num +)* 0;
                sum > $threshold
            }
        };
        (contains: [$($num:expr),* $(,)?], $target:expr) => {
            {
                let mut found = false;
                $(
                    if $num == $target {
                        found = true;
                    }
                )*
                found
            }
        };
    }

    println!("\n模式识别示例:");
    println!("都是偶数: {}", identify_pattern!(all_even: [2, 4, 6]));
    println!(
        "和大于30: {}",
        identify_pattern!(sum_greater_than: [5, 10, 20], 30)
    );
    println!("包含6: {}", identify_pattern!(contains: [1, 2, 3, 6], 6));
}

/// 演示内联函数宏
pub fn inline_function_macros() {
    println!("⚡ 内联函数宏：");

    // 类似函数的内联宏
    macro_rules! safe_divide {
        ($a:expr, $b:expr) => {
            if $b == 0 {
                println!("⚠️ 警告：除零错误");
                0
            } else {
                $a / $b
            }
        };
        ($a:expr, $b:expr, fallback: $fallback:expr) => {
            if $b == 0 {
                println!("⚠️ 警告：除零错误，使用回退值");
                $fallback
            } else {
                $a / $b
            }
        };
    }

    println!("安全除法示例:");
    println!("10 / 2 = {}", safe_divide!(10, 2));
    println!("10 / 0 = {}", safe_divide!(10, 0));
    println!("10 / 0 (回退=99) = {}", safe_divide!(10, 0, fallback: 99));

    // 缓存宏
    macro_rules! memoized_fibonacci {
        ($n:expr) => {{
            fn fib(n: u32) -> u64 {
                if n <= 1 {
                    n as u64
                } else {
                    fib(n - 1) + fib(n - 2)
                }
            }
            fib($n)
        }};
    }

    println!("\n斐波那契计算:");
    println!("fib(10) = {}", memoized_fibonacci!(10));

    // 简化状态机宏
    #[derive(Debug)]
    enum SimpleState {
        Idle,
        Processing,
        Completed,
    }

    macro_rules! simple_state_machine {
        ($initial:ident) => {
            SimpleState::$initial
        };
    }

    // 使用简化状态机宏
    println!("\n状态机示例:");
    let current = simple_state_machine!(Idle);
    println!("初始状态: {:?}", current);
}

/// 演示宏调试技术
pub fn macro_debugging_techniques() {
    println!("🐛 宏调试技术：");

    // 使用 stringified! 进行调试
    macro_rules! debug_macro {
        ($($arg:tt)*) => {
            {
                println!("🐛 宏调用: {}", stringify!($($arg)*));
                println!("🐛 实际参数: {:?}", ($($arg)*));
                $($arg)*
            }
        };
    }

    println!("调试宏示例:");
    let result = debug_macro!(2 + 3 * 4);
    println!("计算结果: {}", result);

    // 调试信息宏
    macro_rules! log_operation {
        (operation: $op:expr, input: $input:expr, output: $output:expr) => {
            println!(
                "🔍 {} - 输入: {:?} -> 输出: {:?}",
                stringify!($op),
                $input,
                $output
            );
        };
    }

    let data = vec![1, 2, 3, 4, 5];
    let filtered: Vec<_> = data.iter().filter(|&&x| x % 2 == 0).collect();
    log_operation!(operation: filter, input: data, output: filtered);

    // 宏展开跟踪
    macro_rules! trace_expansion {
        ($expr:expr) => {{
            println!("📍 正在展开: {}", stringify!($expr));
            let result = $expr;
            println!("📍 展开结果: {:?}", result);
            result
        }};
    }

    println!("\n宏展开跟踪:");
    let doubled: Vec<_> = trace_expansion!(vec![1, 2, 3].iter().map(|x| x * 2).collect());
    println!("最终结果: {:?}", doubled);
}

/// 演示高级DSL构建器
pub fn advanced_dsl_builders() {
    println!("🏗️ 高级DSL构建器：");

    // 简化的SQL查询构建器DSL
    macro_rules! simple_sql_select {
        (SELECT $( $field:ident ),* FROM $table:ident) => {
            {
                format!("SELECT {} FROM {}", stringify!($( $field ),*), stringify!($table))
            }
        };
    }

    println!("SQL查询DSL示例:");
    let simple_query = simple_sql_select!(SELECT name, age FROM users);
    println!("简单查询: {}", simple_query);

    // 简化的配置构建示例
    println!("\n配置构建示例:");
    let mut config = std::collections::HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());
    config.insert("debug".to_string(), "true".to_string());

    println!("构建的配置:");
    for (key, value) in &config {
        println!("  {}: {}", key, value);
    }

    // 简化的流式API DSL
    macro_rules! simple_stream_api {
        ($source:expr, filter: $filter:expr) => {{ $source.filter($filter).collect::<Vec<_>>() }};
        ($source:expr, map: $map:expr) => {{ $source.map($map).collect::<Vec<_>>() }};
    }

    println!("\n流式APIDSL示例:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<_> = simple_stream_api!(numbers.into_iter(), filter: |&x| x % 2 == 0);
    println!("流式处理结果: {:?}", result);
}

/// 运行宏和DSL示例
pub fn run_macro_dsl_examples() {
    println!("🎯 === 现代化宏和DSL示例 ===");
    println!();

    println!("=== 基础宏示例 ===");
    run_macros_examples();
    println!();

    println!("=== HTML构建DSL ===");
    html_builder_dsl();
    println!();

    println!("=== 配置管理DSL ===");
    configuration_dsl();
    println!();

    println!("=== API路由DSL ===");
    api_routing_dsl();

    println!("\n✅ 所有宏和DSL示例运行完成！");
}

/// 运行高级宏示例
pub fn run_advanced_macro_examples() {
    println!("🎯 === 高级宏和元编程示例 ===");
    println!();

    println!("=== 复杂宏模式匹配 ===");
    complex_macro_patterns();
    println!();

    println!("=== 内联函数宏 ===");
    inline_function_macros();
    println!();

    println!("=== 宏调试技术 ===");
    macro_debugging_techniques();
    println!();

    println!("=== 高级DSL构建器 ===");
    advanced_dsl_builders();

    println!("\n✅ 所有高级宏示例运行完成！");
}
