//! # 并发编程模块
//!
//! 这个模块演示了Rust的并发编程特性，包括线程、消息传递和共享状态。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::sync::mpsc;
use std::sync::{Arc, LockResult, RwLock};
use std::thread;
use std::time::Duration;

fn recover_lock<T>(result: LockResult<T>) -> T {
    match result {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("⚠️ 检测到锁中毒，示例继续使用受保护数据以保持演示完整性");
            poisoned.into_inner()
        }
    }
}

fn join_and_report<T>(handle: thread::JoinHandle<T>, label: &str) -> Option<T> {
    match handle.join() {
        Ok(value) => Some(value),
        Err(_) => {
            eprintln!("❌ {} 线程发生 panic", label);
            None
        }
    }
}

/// 现代化同步线程示例
pub fn modern_sync_threads() {
    println!("🧵 现代化同步线程：");

    let handle = thread::spawn(move || {
        for i in 1..5 {
            println!("线程 {}: 处理任务", i);
            thread::sleep(Duration::from_millis(100));
        }
        "线程完成".to_string()
    });

    if let Some(msg) = join_and_report(handle, "同步线程示例") {
        println!("✅ {}", msg);
    }

    println!("主线程继续执行");
}

/// 现代化消息传递
pub fn modern_message_passing() {
    println!("📨 现代化消息传递：");

    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        let messages = vec![
            "消息1".to_string(),
            "消息2".to_string(),
            "消息3".to_string(),
        ];

        for msg in messages {
            println!("📤 发送: {}", msg);
            if tx.send(msg).is_err() {
                eprintln!("⚠️ 接收端已关闭，生产者提前结束");
                return;
            }
            thread::sleep(Duration::from_millis(200));
        }

        println!("📊 生产者完成，并通过关闭通道通知消费者结束");
    });

    let consumer = thread::spawn(move || {
        for received in rx {
            println!("📥 接收: {}", received);
        }
        println!("📊 消费者检测到通道关闭后完成");
    });

    let _ = join_and_report(producer, "消息生产者");
    let _ = join_and_report(consumer, "消息消费者");
}

/// 现代化共享状态管理
pub fn modern_shared_state() {
    println!("🔒 现代化共享状态：");

    // 使用Arc<RwLock>进行读多写少的场景优化
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // 多个读取者
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let data = recover_lock(data.read());
            println!("🔍 读取者{}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 写入者
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = recover_lock(data.write());
            data.push(6);
            println!("✏️ 写入者添加了新数据: {:?}", *data);
        });
        handles.push(handle);
    }

    for (index, handle) in handles.into_iter().enumerate() {
        let _ = join_and_report(handle, &format!("共享状态线程 {}", index));
    }
}

/// 现代化条件变量和同步
pub fn modern_synchronization() {
    println!("⚡ 现代化同步机制：");

    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Barrier, Once};

    // 使用Barrier进行多线程同步
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("线程 {} 开始准备", i);
            thread::sleep(Duration::from_millis(100));

            println!("线程 {} 等待同步点", i);
            barrier.wait();

            println!("线程 {} 继续执行", i);
        });
        handles.push(handle);
    }

    for (index, handle) in handles.into_iter().enumerate() {
        let _ = join_and_report(handle, &format!("同步机制线程 {}", index));
    }

    // 使用Once配合原子类型表达一次性初始化，避免static mut的误导
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    static INIT_ONCE: Once = Once::new();

    INIT_ONCE.call_once(|| {
        INITIALIZED.store(true, Ordering::SeqCst);
        println!("🚀 一次性初始化完成");
    });

    println!(
        "📌 初始化状态: {}（教学提示：共享可变全局状态应避免使用 static mut）",
        INITIALIZED.load(Ordering::SeqCst)
    );
}

/// 现代化生产者-消费者模式
pub fn modern_producer_consumer() {
    println!("🏭 现代化生产者-消费者：");

    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};

    struct SharedBuffer {
        items: VecDeque<i32>,
        production_done: bool,
    }

    let shared = Arc::new((
        Mutex::new(SharedBuffer {
            items: VecDeque::new(),
            production_done: false,
        }),
        Condvar::new(),
    ));

    let producer_shared = Arc::clone(&shared);
    let producer = thread::spawn(move || {
        let (lock, available) = &*producer_shared;

        for i in 1..=5 {
            let mut state = recover_lock(lock.lock());
            state.items.push_back(i);
            println!("📦 生产: {}", i);
            available.notify_one();
        }

        let mut state = recover_lock(lock.lock());
        state.production_done = true;
        println!("📊 生产者已发出完成信号");
        available.notify_all();
    });

    let consumer = {
        let consumer_shared = Arc::clone(&shared);
        thread::spawn(move || {
            let (lock, available) = &*consumer_shared;

            loop {
                let mut state = recover_lock(lock.lock());
                while state.items.is_empty() && !state.production_done {
                    state = recover_lock(available.wait(state));
                }

                if let Some(item) = state.items.pop_front() {
                    println!("📦 消费: {}", item);
                    continue;
                }

                if state.production_done {
                    println!("📊 消费者确认所有数据都已处理");
                    break;
                }
            }
        })
    };

    let _ = join_and_report(producer, "生产者");
    let _ = join_and_report(consumer, "消费者");
}

/// 现代化工作池模式
pub fn modern_work_pool() {
    println!("🏊 现代化工作池：");

    use std::sync::{Arc, Mutex, mpsc};

    // 创建一个共享的工作队列
    let work_queue = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6]));
    let (result_sender, result_receiver) = mpsc::channel();

    // 创建工作线程池
    let mut workers = Vec::new();

    for id in 0..3 {
        let work_queue = Arc::clone(&work_queue);
        let result_sender = result_sender.clone();

        let worker = thread::spawn(move || loop {
            let work = {
                let mut queue = recover_lock(work_queue.lock());
                queue.pop()
            };

            match work {
                Some(work_item) => {
                    println!("👷 工作者 {} 处理任务: {}", id, work_item);
                    thread::sleep(Duration::from_millis(100));

                    let result = format!("工作者 {} 完成任务: {}", id, work_item);
                    if result_sender.send(result).is_err() {
                        eprintln!("⚠️ 结果接收端已关闭，工作者 {} 提前结束", id);
                        break;
                    }
                }
                None => {
                    println!("👷 工作者 {} 退出，队列为空", id);
                    break;
                }
            }
        });
        workers.push(worker);
    }

    drop(result_sender);

    while let Ok(result) = result_receiver.recv() {
        println!("📊 {}", result);
    }

    for (index, worker) in workers.into_iter().enumerate() {
        let _ = join_and_report(worker, &format!("工作池线程 {}", index));
    }
}

/// 演示真实Web服务器并发处理
pub fn web_server_concurrent_handling() {
    println!("🌐 Web服务器并发请求处理：");

    use std::sync::{Arc, Mutex};

    // 模拟请求统计
    #[derive(Clone)]
    struct RequestStats {
        total: Arc<Mutex<u32>>,
        completed: Arc<Mutex<u32>>,
        failed: Arc<Mutex<u32>>,
    }

    impl RequestStats {
        fn new() -> Self {
            Self {
                total: Arc::new(Mutex::new(0)),
                completed: Arc::new(Mutex::new(0)),
                failed: Arc::new(Mutex::new(0)),
            }
        }

        fn increment(counter: &Arc<Mutex<u32>>) {
            let mut value = recover_lock(counter.lock());
            *value += 1;
        }

        fn total(&self) -> u32 {
            *recover_lock(self.total.lock())
        }

        fn completed(&self) -> u32 {
            *recover_lock(self.completed.lock())
        }

        fn failed(&self) -> u32 {
            *recover_lock(self.failed.lock())
        }
    }

    // 模拟HTTP请求处理
    fn handle_request(id: u32, stats: &RequestStats) {
        RequestStats::increment(&stats.total);

        let processing_time = match id % 4 {
            0 => 50,  // 快速请求
            1 => 200, // 中等请求
            2 => 500, // 慢请求
            _ => 100, // 错误请求
        };

        thread::sleep(Duration::from_millis(processing_time));

        if id % 4 == 3 {
            RequestStats::increment(&stats.failed);
            println!("❌ 请求 {} 失败", id);
        } else {
            RequestStats::increment(&stats.completed);
            println!("✅ 请求 {} 成功", id);
        }
    }

    let stats = RequestStats::new();
    let mut handles = vec![];

    // 模拟并发请求
    for i in 0..10 {
        let stats_clone = stats.clone();

        let handle = thread::spawn(move || {
            handle_request(i, &stats_clone);
        });

        handles.push(handle);
    }

    for (index, handle) in handles.into_iter().enumerate() {
        let _ = join_and_report(handle, &format!("Web请求线程 {}", index));
    }

    println!("📊 请求统计:");
    println!("   总请求: {}", stats.total());
    println!("   成功: {}", stats.completed());
    println!("   失败: {}", stats.failed());
}

/// 演示数据库连接池
pub fn database_connection_pool() {
    println!("🗄️ 数据库连接池：");

    use std::collections::VecDeque;
    use std::sync::{Arc, Condvar, Mutex};

    // 模拟数据库连接
    struct DbConnection {
        id: u32,
        busy: bool,
    }

    impl DbConnection {
        fn new(id: u32) -> Self {
            Self { id, busy: false }
        }

        fn execute_query(&mut self, query: &str) -> String {
            self.busy = true;
            thread::sleep(Duration::from_millis(50));

            let result = if query.contains("error") {
                format!("连接 {} 查询失败", self.id)
            } else {
                format!("连接 {} 查询成功: {} 行", self.id, 100 + (self.id % 50))
            };

            self.busy = false;
            result
        }
    }

    #[derive(Clone)]
    struct ConnectionPool {
        state: Arc<(Mutex<VecDeque<DbConnection>>, Condvar)>,
        max_size: usize,
    }

    impl ConnectionPool {
        fn new(size: usize) -> Self {
            let mut connections = VecDeque::new();
            for i in 0..size {
                connections.push_back(DbConnection::new(i as u32));
            }

            Self {
                state: Arc::new((Mutex::new(connections), Condvar::new())),
                max_size: size,
            }
        }

        fn get_connection(&self, requester_id: usize) -> DbConnection {
            let (lock, available) = &*self.state;
            let mut pool = recover_lock(lock.lock());

            loop {
                if let Some(conn) = pool.pop_front() {
                    return conn;
                }

                println!("线程 {} 等待可用连接", requester_id);
                pool = recover_lock(available.wait(pool));
            }
        }

        fn return_connection(&self, conn: DbConnection) {
            if conn.busy {
                eprintln!("⚠️ 连接 {} 仍处于忙碌状态，未归还到连接池", conn.id);
                return;
            }

            let (lock, available) = &*self.state;
            let mut pool = recover_lock(lock.lock());

            if pool.len() < self.max_size {
                let conn_id = conn.id;
                pool.push_back(conn);
                println!("🔁 连接 {} 已归还连接池", conn_id);
                available.notify_one();
            } else {
                eprintln!("⚠️ 连接池已满，丢弃连接 {}", conn.id);
            }
        }
    }

    let pool = ConnectionPool::new(3);
    let mut handles = vec![];

    // 模拟并发查询
    let queries = vec![
        "SELECT * FROM users".to_string(),
        "INSERT INTO logs VALUES (1)".to_string(),
        "UPDATE products SET price = 99".to_string(),
        "SELECT * FROM orders".to_string(),
    ];

    for (i, query) in queries.into_iter().enumerate() {
        let pool_clone = pool.clone();

        let handle = thread::spawn(move || {
            let mut conn = pool_clone.get_connection(i);
            println!("线程 {} 获取连接 {} 并执行查询: {}", i, conn.id, query);
            let result = conn.execute_query(&query);
            println!("{}", result);
            pool_clone.return_connection(conn);
        });

        handles.push(handle);
    }

    for (index, handle) in handles.into_iter().enumerate() {
        let _ = join_and_report(handle, &format!("连接池线程 {}", index));
    }

    println!("📊 连接池查询完成，所有查询线程都已收尾");
}

/// 运行所有并发编程示例
pub fn run_concurrency_examples() {
    println!("🎯 === 现代化并发编程示例 ===");
    println!();

    modern_sync_threads();
    println!();

    modern_message_passing();
    println!();

    modern_shared_state();
    println!();

    modern_synchronization();
    println!();

    modern_producer_consumer();
    println!();

    modern_work_pool();
    println!();

    web_server_concurrent_handling();
    println!();

    database_connection_pool();

    println!("\n✅ 所有并发编程示例运行完成！");
}
