//! # Rust常见陷阱模块
//!
//! 这个模块演示了Rust编程中常见的陷阱和错误，包括借用检查器错误、
//! 生命周期陷阱、性能陷阱、内存泄漏案例等常见问题。
//! 帮助开发者识别和避免这些常见错误。

#![allow(dead_code)]
#![allow(clippy::all)]

use std::rc::Weak;

/// 演示借用检查器错误
pub fn borrowing_checker_pitfalls() {
    println!("🔍 借用检查器陷阱：");

    // 陷阱1：同时存在可变和不可变借用
    println!("\n1️⃣ 同时存在可变和不可变借用:");

    let mut data = vec![1, 2, 3, 4, 5];

    // ❌ 错误示例：同时存在不可变和可变借用
    // let first = &data[0];
    // data.push(6); // 编译错误：同时存在借用

    // ✅ 正确解决方案
    let first = data[0]; // 先使用值
    data.push(6); // 然后可以修改
    println!("第一个元素: {}, 修改后: {:?}", first, data);

    // 陷阱2：借用生命周期问题
    println!("\n2️⃣ 借用生命周期问题:");

    fn return_first_element() -> i32 {
        let v = vec![1, 2, 3];
        // ❌ 错误：返回对局部变量的引用
        // &v[0] // 编译错误：返回对临时变量的引用

        // ✅ 正确做法：返回整个向量拥有的数据，而不是局部引用
        v[0]
    }

    println!("返回的第一个元素: {}", return_first_element());

    // 陷阱3：切片越界
    println!("\n3️⃣ 切片越界风险:");

    let numbers = vec![1, 2, 3, 4, 5];

    // ❌ 危险：直接索引访问可能导致panic
    // let value = numbers[10]; // 会panic

    // ✅ 安全：使用get方法
    match numbers.get(10) {
        Some(value) => println!("安全访问: {}", value),
        None => println!("索引超出范围，但程序继续执行"),
    }

    // 陷阱4：闭包捕获借用问题
    println!("\n4️⃣ 闭包捕获借用问题:");

    let mut counter = 0;
    let mut increment = || {
        counter += 1; // 闭包捕获了counter的可变引用
    };

    increment();
    drop(increment);
    println!("计数器: {}", counter);

    // 陷阱5：Vec重新分配导致引用失效
    println!("\n5️⃣ Vec重新分配导致引用失效:");

    let mut v = vec![1, 2, 3];
    let first = &v[0]; // 获取第一个元素的引用

    println!("第一个元素: {}", first);

    // ❌ 错误：此时如果继续保留first，再push可能因重新分配导致悬垂引用，因此编译器会阻止
    // v.push(4);
    // println!("第一个元素: {}", first);

    // ✅ 正确：在可能重新分配之前结束借用，再进行修改
    let first_value = v[0];
    v.push(4);
    println!("复制出的第一个元素值: {}, Vec内容: {:?}", first_value, v);
}

/// 生命周期陷阱
pub fn lifetime_pitfalls() {
    println!("⏰ 生命周期陷阱：");

    // 陷阱1：函数参数生命周期不匹配
    println!("\n1️⃣ 函数参数生命周期问题:");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("short");
    let string2 = "very long string";

    // 正确：生命周期正确匹配
    let result = longest(&string1, string2);
    println!("较长的字符串: {}", result);

    // 陷阱2：结构体中的生命周期
    println!("\n2️⃣ 结构体中的生命周期:");

    struct StringHolder<'a> {
        text: &'a str,
    }

    impl<'a> StringHolder<'a> {
        fn new(text: &'a str) -> Self {
            Self { text }
        }

        fn get_text(&self) -> &str {
            self.text
        }
    }

    let text = "保存的文本";
    let holder = StringHolder::new(text);
    println!("保存的文本: {}", holder.get_text());

    // 陷阱3：静态生命周期误用
    println!("\n3️⃣ 静态生命周期陷阱:");

    static GLOBAL_DATA: &[&str] = &["全局", "数据", "数组"];

    fn use_global_data() -> &'static str {
        GLOBAL_DATA[0] // 正确的静态生命周期使用
    }

    println!("全局数据: {}", use_global_data());

    // ❌ 陷阱：试图返回局部引用的静态引用
    // fn bad_function() -> &'static str {
    //     let local = "局部数据";
    //     &local // 编译错误：局部数据不能是静态的
    // }
}

/// 性能陷阱
pub fn performance_pitfalls() {
    println!("⚡ 性能陷阱：");

    // 陷阱1：不必要的克隆
    println!("\n1️⃣ 不必要的克隆:");

    let data = vec![1, 2, 3, 4, 5];

    // ✅ 高效：直接借用原始数据进行迭代，data 之后仍然可用
    let sum1: i32 = data.iter().sum();
    println!("借用迭代后原数据仍可使用: {:?}", data);

    let data2 = vec![1, 2, 3, 4, 5];
    // ❌ 低效：clone 会复制整个 Vec，但这里只是为了求和，并不需要所有权副本
    let sum2: i32 = data2.clone().iter().sum();
    println!("克隆后原数据同样仍可使用: {:?}", data2);

    println!("高效求和: {}", sum1);
    println!("低效求和(结果相同，但多了一次分配/复制): {}", sum2);

    // 陷阱2：频繁的内存分配
    println!("\n2️⃣ 频繁内存分配:");

    use std::time::Instant;

    let start = Instant::now();

    // ❌ 低效：在循环中频繁分配
    let mut result = String::new();
    for i in 0..1000 {
        result.push_str(&format!("{} ", i));
    }

    let duration = start.elapsed();
    println!("低效字符串构建耗时: {:?}", duration);

    // ✅ 高效：预分配容量
    let start = Instant::now();

    let mut result2 = String::with_capacity(5000);
    for i in 0..1000 {
        result2.push_str(&format!("{} ", i));
    }

    let duration2 = start.elapsed();
    println!("高效字符串构建耗时: {:?}", duration2);

    // 陷阱3：Box< dyn Trait> 的动态分发开销
    println!("\n3️⃣ 动态分发开销:");

    trait SimpleTrait {
        fn process(&self) -> i32;
    }

    struct A;
    struct B;

    impl SimpleTrait for A {
        fn process(&self) -> i32 {
            42
        }
    }

    impl SimpleTrait for B {
        fn process(&self) -> i32 {
            24
        }
    }

    let objects: Vec<Box<dyn SimpleTrait>> = vec![Box::new(A), Box::new(B), Box::new(A)];

    // 动态分发有运行时开销
    let sum: i32 = objects.iter().map(|obj| obj.process()).sum();
    println!("动态分发总和: {}", sum);

    // ✅ 更好的选择：如果类型已知，使用枚举
    enum KnownType {
        TypeA,
        TypeB,
    }

    impl KnownType {
        fn process(&self) -> i32 {
            match self {
                KnownType::TypeA => 42,
                KnownType::TypeB => 24,
            }
        }
    }

    let objects2 = vec![KnownType::TypeA, KnownType::TypeB, KnownType::TypeA];

    let sum2: i32 = objects2.iter().map(|obj| obj.process()).sum();
    println!("静态分发总和: {}", sum2);
}

/// 内存泄漏陷阱
pub fn memory_leak_pitfalls() {
    println!("🧠 内存泄漏陷阱：");

    // 陷阱1：Rc引用计数循环
    println!("\n1️⃣ Rc引用计数循环:");

    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<RefCell<Node>>>,
        prev: Option<Rc<RefCell<Node>>>,
    }

    // ❌ 这会创建循环引用，导致内存泄漏
    // 注释掉以避免警告
    /*
    let node1 = Rc::new(RefCell::new(Node {
        value: 1,
        next: None,
        prev: None,
    }));

    let node2 = Rc::new(RefCell::new(Node {
        value: 2,
        next: Some(Rc::clone(&node1)),
        prev: Some(Rc::clone(&node1)),
    }));

    // 建立了循环引用
    node1.borrow_mut().next = Some(Rc::clone(&node2));
    */

    // ✅ 正确做法：使用Weak打破循环
    println!("正确使用Weak引用避免循环:");

    #[derive(Debug)]
    struct SafeNode {
        value: i32,
        next: Option<Rc<RefCell<SafeNode>>>,
        parent: Option<Weak<RefCell<SafeNode>>>,
    }

    // 陷阱2：忘记drop大型结构
    println!("\n2️⃣ 忘记处理大型数据:");

    struct LargeData {
        data: Vec<u8>,
        metadata: Vec<String>,
    }

    // ❌ 陷阱：在作用域中保留大型数据
    {
        let large = LargeData {
            data: vec![0; 10_000_000], // 10MB
            metadata: vec!["metadata".to_string(); 1_000],
        };

        // 处理一些逻辑...
        println!("处理大数据...");
        drop(large); // 明确释放
    }

    // 作用域结束后自动释放
    println!("大数据已释放");
}

/// 错误处理陷阱
pub fn error_handling_pitfalls() {
    println!("🚨 错误处理陷阱：");

    // 陷阱1：忽略错误
    println!("\n1️⃣ 忽略错误:");

    // ❌ 错误示例：忽略错误
    // let _ = std::fs::read_to_string("可能不存在的文件.txt");

    // ✅ 正确处理错误
    match std::fs::read_to_string("可能不存在的文件.txt") {
        Ok(content) => println!("文件内容: {}", content),
        Err(e) => println!("读取文件失败: {}", e),
    }

    // 陷阱2：不当的panic使用
    println!("\n2️⃣ 不当的panic使用:");

    fn risky_function(should_panic: bool) -> Result<i32, String> {
        if should_panic {
            // ❌ 在库代码中不应该panic
            // panic!("不应该在这里panic");

            // ✅ 正确做法：返回错误
            return Err("操作失败".to_string());
        }
        Ok(42)
    }

    match risky_function(false) {
        Ok(value) => println!("操作成功: {}", value),
        Err(e) => println!("操作失败: {}", e),
    }

    // 陷阱3：使用unwrap导致崩溃
    println!("\n3️⃣ unwrap导致的崩溃:");

    let optional_value: Option<i32> = None;

    // ❌ 危险：unwrap在生产代码中可能导致崩溃
    // let value = optional_value.unwrap(); // 会panic

    // ✅ 安全：使用unwrap_or或match
    let value = optional_value.unwrap_or(0);
    println!("安全解包值: {}", value);

    let value2 = match optional_value {
        Some(v) => v,
        None => {
            println!("值为None，使用默认值");
            0
        }
    };
    println!("match解包值: {}", value2);
}

/// 并发陷阱
pub fn concurrency_pitfalls() {
    println!("🔄 并发陷阱：");

    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    fn join_with_log(handle: thread::JoinHandle<()>, label: &str) {
        if handle.join().is_err() {
            eprintln!("❌ {} 线程发生 panic", label);
        }
    }

    // 陷阱1：数据竞争
    println!("\n1️⃣ 数据竞争:");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            match counter.lock() {
                Ok(mut num) => {
                    *num += 1;
                    println!("线程 {} 增加计数", i);
                }
                Err(poisoned) => {
                    let mut num = poisoned.into_inner();
                    *num += 1;
                    eprintln!("⚠️ 线程 {} 遇到中毒锁，但示例仍完成计数", i);
                }
            }
        });
        handles.push(handle);
    }

    for (index, handle) in handles.into_iter().enumerate() {
        join_with_log(handle, &format!("计数线程 {}", index));
    }

    let final_count = match counter.lock() {
        Ok(num) => *num,
        Err(poisoned) => {
            eprintln!("⚠️ 最终读取时检测到中毒锁，继续输出已恢复的计数");
            *poisoned.into_inner()
        }
    };
    println!("最终计数: {}", final_count);

    // 陷阱2：死锁
    println!("\n2️⃣ 死锁风险:");

    let mutex1 = Arc::new(Mutex::new(1));
    let mutex2 = Arc::new(Mutex::new(2));

    // 使用通道控制顺序，演示“相反的加锁顺序会形成循环等待”，但不让示例真实卡死
    let (start_tx, start_rx) = mpsc::channel();
    let (first_lock_tx, first_lock_rx) = mpsc::channel();
    let shared_start_rx = Arc::new(Mutex::new(start_rx));
    let shared_first_lock_rx = Arc::new(Mutex::new(first_lock_rx));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);
    let start_rx_thread1 = Arc::clone(&shared_start_rx);
    let first_lock_rx_thread1 = Arc::clone(&shared_first_lock_rx);

    let handle1 = thread::spawn(move || {
        let received = match start_rx_thread1.lock() {
            Ok(rx) => rx.recv(),
            Err(poisoned) => poisoned.into_inner().recv(),
        };

        if received.is_err() {
            eprintln!("⚠️ 线程1 未收到开始信号");
            return;
        }

        let _guard1 = match mutex1_clone.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        println!("线程1先拿到 mutex1");

        if first_lock_tx.send("线程1已持有mutex1").is_err() {
            eprintln!("⚠️ 线程1 无法发送阶段信号");
            return;
        }

        let peer_ready = match first_lock_rx_thread1.lock() {
            Ok(rx) => rx.recv(),
            Err(poisoned) => poisoned.into_inner().recv(),
        };

        if peer_ready.is_err() {
            eprintln!("⚠️ 线程1 未收到对端已持锁的信号");
            return;
        }

        println!("线程1准备继续请求 mutex2：如果线程2此时也持有 mutex2，就会进入死锁");
        if mutex2_clone.try_lock().is_err() {
            println!("线程1检测到 mutex2 当前不可用，因此停止在示意阶段，不让示例真的卡住");
        }
    });

    let mutex1_clone2 = Arc::clone(&mutex1);
    let mutex2_clone2 = Arc::clone(&mutex2);
    let start_rx_thread2 = Arc::clone(&shared_start_rx);
    let first_lock_rx_thread2 = Arc::clone(&shared_first_lock_rx);

    let handle2 = thread::spawn(move || {
        let received = match start_rx_thread2.lock() {
            Ok(rx) => rx.recv(),
            Err(poisoned) => poisoned.into_inner().recv(),
        };

        if received.is_err() {
            eprintln!("⚠️ 线程2 未收到开始信号");
            return;
        }

        let _guard2 = match mutex2_clone2.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        println!("线程2先拿到 mutex2");

        let peer_ready = match first_lock_rx_thread2.lock() {
            Ok(rx) => rx.recv(),
            Err(poisoned) => poisoned.into_inner().recv(),
        };

        if let Ok(message) = peer_ready {
            println!("线程2观察到: {}", message);
        } else {
            eprintln!("⚠️ 线程2 未收到对端阶段信号");
            return;
        }

        println!("线程2准备继续请求 mutex1：如果线程1此时也持有 mutex1，就会进入死锁");
        if mutex1_clone2.try_lock().is_err() {
            println!("线程2检测到 mutex1 当前不可用，因此改为退出并释放资源");
        }
    });

    if start_tx.send(()).is_err() {
        eprintln!("⚠️ 无法启动线程1 的死锁演示");
    }
    if start_tx.send(()).is_err() {
        eprintln!("⚠️ 无法启动线程2 的死锁演示");
    }
    drop(start_tx);

    join_with_log(handle1, "死锁演示线程1");
    join_with_log(handle2, "死锁演示线程2");

    println!("避免死锁：统一加锁顺序，或使用 try_lock/超时/更细粒度设计让线程可以收尾");

    // 陷阱3：线程间传递数据
    println!("\n3️⃣ 线程间数据传递:");

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let data = "来自线程的数据";
        match tx.send(data) {
            Ok(()) => println!("数据已发送"),
            Err(e) => eprintln!("❌ 发送失败: {}", e),
        }
    });

    match rx.recv_timeout(Duration::from_secs(1)) {
        Ok(received) => println!("收到数据: {}", received),
        Err(e) => eprintln!("❌ 接收失败: {}", e),
    }

    join_with_log(handle, "消息发送线程");
}

/// 运行陷阱示例
pub fn run_pitfalls_examples() {
    println!("🎯 === Rust常见陷阱示例 ===");
    println!();

    borrowing_checker_pitfalls();
    println!();

    lifetime_pitfalls();
    println!();

    performance_pitfalls();
    println!();

    memory_leak_pitfalls();
    println!();

    error_handling_pitfalls();
    println!();

    concurrency_pitfalls();

    println!("\n✅ 所有陷阱示例运行完成！");
    println!("💡 了解这些陷阱能帮助你写出更安全、更高效的Rust代码！");
}
