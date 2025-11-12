//! # 并发编程模块
//!
//! 这个模块演示了Rust的并发编程特性，包括线程、消息传递和共享状态。
//! 采用了现代化的Rust 2021/2024最佳实践。

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, RwLock};

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

    // 使用现代化错误处理
    let _result = handle.join().map(|msg| {
        println!("✅ {}", msg);
        msg
    }).unwrap_or_else(|_e| {
        eprintln!("❌ 线程执行失败");
        "默认值".to_string()
    });
    
    println!("主线程继续执行");
}

/// 现代化消息传递
pub fn modern_message_passing() {
    println!("📨 现代化消息传递：");
    
    let (tx, rx) = mpsc::channel();

    // 生产者线程
    thread::spawn(move || {
        let messages = vec![
            "消息1".to_string(),
            "消息2".to_string(),
            "消息3".to_string(),
        ];

        for msg in messages {
            println!("📤 发送: {}", msg);
            if tx.send(msg).is_err() {
                break;
            }
            thread::sleep(Duration::from_millis(200));
        }
        
        println!("📊 生产者完成");
    });

    // 消费者线程
    thread::spawn(move || {
        for received in rx {
            println!("📥 接收: {}", received);
        }
        println!("📊 消费者完成");
    });
    
    thread::sleep(Duration::from_millis(1000)); // 等待线程完成
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
            let data = data.read().unwrap();
            println!("🔍 读取者{}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 写入者
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.write().unwrap();
            data.push(6);
            println!("✏️ 写入者添加了新数据: {:?}", *data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// 现代化条件变量和同步
pub fn modern_synchronization() {
    println!("⚡ 现代化同步机制：");
    
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

    for handle in handles {
        handle.join().unwrap();
    }
    
    // 使用Once进行一次性初始化
    static mut INITIALIZED: bool = false;
    static INIT_ONCE: Once = Once::new();

    INIT_ONCE.call_once(|| {
        unsafe {
            INITIALIZED = true;
        }
        println!("🚀 一次性初始化完成");
    });
}

/// 现代化生产者-消费者模式
pub fn modern_producer_consumer() {
    println!("🏭 现代化生产者-消费者：");
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = Arc::clone(&buffer);
    
    // 生产者
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            {
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.push(i);
                println!("📦 生产: {}", i);
            }
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 消费者
    let consumer = {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            for _ in 1..=5 {
                {
                    let mut buffer = buffer_clone.lock().unwrap();
                    if let Some(item) = buffer.pop() {
                        println!("📦 消费: {}", item);
                    }
                }
                thread::sleep(Duration::from_millis(300));
            }
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}

/// 现代化工作池模式
pub fn modern_work_pool() {
    println!("🏊 现代化工作池：");
    
    use std::sync::{Arc, Mutex, mpsc};
    use std::thread;
    
    // 创建一个共享的工作队列
    let work_queue = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6]));
    let (result_sender, result_receiver) = mpsc::channel();
    
    // 创建工作线程池
    let mut workers = Vec::new();
    
    for id in 0..3 {
        let work_queue = Arc::clone(&work_queue);
        let result_sender = result_sender.clone();
        
        let worker = thread::spawn(move || {
            loop {
                let work = {
                    let mut queue = work_queue.lock().unwrap();
                    queue.pop()
                };
                
                match work {
                    Some(work_item) => {
                        println!("👷 工作者 {} 处理任务: {}", id, work_item);
                        thread::sleep(Duration::from_millis(100));
                        
                        let result = format!("工作者 {} 完成任务: {}", id, work_item);
                        let _ = result_sender.send(result);
                    }
                    None => {
                        println!("👷 工作者 {} 退出，队列为空", id);
                        break;
                    }
                }
            }
        });
        workers.push(worker);
    }
    
    // 等待所有工作完成
    for _ in 0..6 {
        if let Ok(result) = result_receiver.recv() {
            println!("📊 {}", result);
        }
    }
    
    // 等待所有工作线程完成
    for worker in workers {
        worker.join().unwrap();
    }
}

/// 演示真实Web服务器并发处理
pub fn web_server_concurrent_handling() {
    println!("🌐 Web服务器并发请求处理：");
    
    use std::sync::{Arc, Mutex};
    
    // 模拟请求统计
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
    }
    
    // 模拟HTTP请求处理
    fn handle_request(id: u32, stats: &RequestStats) {
        *stats.total.lock().unwrap() += 1;
        
        let processing_time = match id % 4 {
            0 => 50,   // 快速请求
            1 => 200,  // 中等请求  
            2 => 500,  // 慢请求
            _ => 100,  // 错误请求
        };
        
        thread::sleep(Duration::from_millis(processing_time));
        
        if id % 4 == 3 {
            *stats.failed.lock().unwrap() += 1;
            println!("❌ 请求 {} 失败", id);
        } else {
            *stats.completed.lock().unwrap() += 1;
            println!("✅ 请求 {} 成功", id);
        }
    }
    
    let stats = RequestStats::new();
    let mut handles = vec![];
    
    // 模拟并发请求
    for i in 0..10 {
        let stats_clone = RequestStats {
            total: Arc::clone(&stats.total),
            completed: Arc::clone(&stats.completed),
            failed: Arc::clone(&stats.failed),
        };
        
        let handle = thread::spawn(move || {
            handle_request(i, &stats_clone);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("📊 请求统计:");
    println!("   总请求: {}", *stats.total.lock().unwrap());
    println!("   成功: {}", *stats.completed.lock().unwrap());
    println!("   失败: {}", *stats.failed.lock().unwrap());
}

/// 演示数据库连接池
pub fn database_connection_pool() {
    println!("🗄️ 数据库连接池：");
    
    use std::sync::{Arc, Mutex};
    use std::collections::VecDeque;
    
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
    
    // 连接池
    struct ConnectionPool {
        connections: Arc<Mutex<VecDeque<DbConnection>>>,
        max_size: usize,
    }
    
    impl ConnectionPool {
        fn new(size: usize) -> Self {
            let mut connections = VecDeque::new();
            for i in 0..size {
                connections.push_back(DbConnection::new(i as u32));
            }
            
            Self {
                connections: Arc::new(Mutex::new(connections)),
                max_size: size,
            }
        }
        
        fn get_connection(&self) -> Option<DbConnection> {
            let mut pool = self.connections.lock().unwrap();
            pool.pop_front()
        }
        
        fn return_connection(&self, conn: DbConnection) {
            if !conn.busy {
                let mut pool = self.connections.lock().unwrap();
                if pool.len() < self.max_size {
                    pool.push_back(conn);
                }
            }
        }
    }
    
    let pool = ConnectionPool::new(3);
    let mut handles = vec![];
    
    // 模拟并发查询
    let queries = vec![
        "SELECT * FROM users",
        "INSERT INTO logs VALUES (1)",
        "UPDATE products SET price = 99",
        "SELECT * FROM orders",
    ];
    
    for (i, query) in queries.iter().enumerate() {
        let pool_clone = ConnectionPool {
            connections: Arc::clone(&pool.connections),
            max_size: pool.max_size,
        };
        
        let query = query.to_string();
        
        let handle = thread::spawn(move || {
            if let Some(mut conn) = pool_clone.get_connection() {
                let result = conn.execute_query(&query);
                println!("{}", result);
                pool_clone.return_connection(conn);
            } else {
                println!("线程 {} 等待连接", i);
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("📊 连接池查询完成");
}

/// 演示文件处理队列
pub fn file_processing_queue() {
    println!("📁 文件处理队列：");
    
    use std::sync::{Arc, Mutex};
    
    // 文件任务
    struct FileTask {
        name: String,
        size: usize,
        processed: bool,
    }
    
    impl FileTask {
        fn new(name: &str, size: usize) -> Self {
            Self {
                name: name.to_string(),
                size,
                processed: false,
            }
        }
        
        fn process(&mut self) -> Result<String, String> {
            thread::sleep(Duration::from_millis(self.size as u64 / 10));
            
            if self.size == 0 {
                return Err("空文件".to_string());
            }
            
            self.processed = true;
            Ok(format!("处理文件 {}: {} 字节", self.name, self.size))
        }
    }
    
    // 处理器
    struct FileProcessor {
        queue: Arc<Mutex<Vec<FileTask>>>,
        processed: Arc<Mutex<u32>>,
    }
    
    impl FileProcessor {
        fn new() -> Self {
            Self {
                queue: Arc::new(Mutex::new(Vec::new())),
                processed: Arc::new(Mutex::new(0)),
            }
        }
        
        fn add_task(&self, task: FileTask) {
            self.queue.lock().unwrap().push(task);
        }
        
        fn process_next(&self) {
            let mut task = {
                let mut queue = self.queue.lock().unwrap();
                if let Some(task) = queue.pop() {
                    task
                } else {
                    return;
                }
            };
            
            match task.process() {
                Ok(result) => {
                    *self.processed.lock().unwrap() += 1;
                    println!("✅ {}", result);
                }
                Err(error) => {
                    println!("❌ 处理失败: {}", error);
                }
            }
        }
        
        fn process_all(&self, workers: usize) {
            let mut handles = vec![];
            
            for _ in 0..workers {
                let processor_clone = FileProcessor {
                    queue: Arc::clone(&self.queue),
                    processed: Arc::clone(&self.processed),
                };
                
                let handle = thread::spawn(move || {
                    for _ in 0..5 { // 每个工作线程处理5个任务
                        processor_clone.process_next();
                        thread::sleep(Duration::from_millis(10));
                    }
                });
                
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
        }
        
        fn get_stats(&self) -> (usize, u32) {
            let queue_len = self.queue.lock().unwrap().len();
            let processed_count = *self.processed.lock().unwrap();
            (queue_len, processed_count)
        }
    }
    
    let processor = FileProcessor::new();
    
    // 添加文件任务
    let files = vec![
        ("doc1.txt", 1000),
        ("doc2.txt", 0),  // 空文件
        ("data.csv", 2500),
        ("script.py", 800),
        ("error.log", 500),
    ];
    
    for (name, size) in files.iter() {
        let task = FileTask::new(name, *size);
        processor.add_task(task);
    }
    
    println!("📋 处理 {} 个文件", files.len());
    processor.process_all(3);
    
    let (remaining, processed) = processor.get_stats();
    println!("📊 处理统计:");
    println!("   已处理: {} 个文件", processed);
    println!("   剩余: {} 个文件", remaining);
}

/// 演示任务调度系统
pub fn task_scheduler_demo() {
    println!("⏰ 任务调度系统：");
    
    use std::sync::{Arc, Mutex};
    
    // 任务类型
    #[derive(Debug, Clone)]
    enum Task {
        Compute { id: u32, data: Vec<i32> },
        Process { id: u32, text: String },
        Download { id: u32, url: String },
    }
    
    impl Task {
        fn duration(&self) -> Duration {
            match self {
                Task::Compute { data, .. } => Duration::from_millis(data.len() as u64 * 10),
                Task::Process { text, .. } => Duration::from_millis(text.len() as u64 * 5),
                Task::Download { .. } => Duration::from_millis(200),
            }
        }
    }
    
    // 调度器
    struct Scheduler {
        tasks: Arc<Mutex<Vec<Task>>>,
        workers: Arc<Mutex<Vec<(u32, bool)>>>, // (worker_id, busy)
    }
    
    impl Scheduler {
        fn new() -> Self {
            let workers = (0..4).map(|i| (i, false)).collect();
            
            Self {
                tasks: Arc::new(Mutex::new(Vec::new())),
                workers: Arc::new(Mutex::new(workers)),
            }
        }
        
        fn add_task(&self, task: Task) {
            self.tasks.lock().unwrap().push(task);
        }
        
        fn schedule(&self) {
            let tasks_clone = Arc::clone(&self.tasks);
            let workers_clone = Arc::clone(&self.workers);
            
            // 启动调度线程
            let _handle = thread::spawn(move || {
                loop {
                    let task_opt = {
                        let mut tasks = tasks_clone.lock().unwrap();
                        tasks.pop()
                    };
                    
                    if let Some(task) = task_opt {
                        // 寻找空闲的工作线程
                        let worker_id = {
                            let mut workers = workers_clone.lock().unwrap();
                            if let Some((id, busy_flag)) = workers.iter_mut().find(|(_, busy)| !*busy) {
                                *busy_flag = true;
                                *id
                            } else {
                                continue; // 没有空闲线程
                            }
                        };
                        
                        // 执行任务
                        let workers_clone_for_task = Arc::clone(&workers_clone);
                        thread::spawn(move || {
                            let duration = task.duration();
                            thread::sleep(duration);
                            
                            let result = match task {
                                Task::Compute { id, data } => {
                                    let sum: i32 = data.iter().sum();
                                    format!("计算任务 {} 完成，结果: {}", id, sum)
                                }
                                Task::Process { id, text } => {
                                    format!("处理任务 {} 完成，文本: {}", id, text.to_uppercase())
                                }
                                Task::Download { id, url } => {
                                    format!("下载任务 {} 完成，URL: {}", id, url)
                                }
                            };
                            
                            println!("✅ {}", result);
                            
                            // 释放工作线程
                            let mut workers = workers_clone_for_task.lock().unwrap();
                            if let Some((_, busy)) = workers.iter_mut().find(|(wid, _)| *wid == worker_id) {
                                *busy = false;
                            }
                        });
                    } else {
                        thread::sleep(Duration::from_millis(100)); // 等待新任务
                    }
                }
            });
            
            // 等待一段时间后停止调度器
            let _stop_handle = thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                println!("⏰ 任务调度完成");
                // 注意：在真实应用中应该优雅地停止调度器
            });
        }
        
        fn get_stats(&self) -> (usize, usize, usize) {
            let task_count = self.tasks.lock().unwrap().len();
            let workers = self.workers.lock().unwrap();
            let busy_workers = workers.iter().filter(|(_, busy)| *busy).count();
            (task_count, busy_workers, workers.len())
        }
    }
    
    let scheduler = Scheduler::new();
    
    // 添加任务
    let tasks = vec![
        Task::Compute { id: 1, data: vec![1, 2, 3, 4, 5] },
        Task::Process { id: 2, text: "hello world".to_string() },
        Task::Download { id: 3, url: "https://example.com".to_string() },
        Task::Compute { id: 4, data: vec![10, 20, 30] },
        Task::Process { id: 5, text: "rust programming".to_string() },
    ];
    
    for task in &tasks {
        scheduler.add_task(task.clone());
    }
    
    println!("📋 添加 {} 个任务", tasks.len());
    scheduler.schedule();
    
    // 等待任务执行
    thread::sleep(Duration::from_secs(3));
    
    let (remaining, busy, total) = scheduler.get_stats();
    println!("📊 调度统计:");
    println!("   剩余任务: {} 个", remaining);
    println!("   忙碌工作线程: {}/{} 个", busy, total);
}

/// 运行并发编程示例
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
    println!();
    
    file_processing_queue();
    println!();
    
    task_scheduler_demo();
    
    println!("\n✅ 所有并发编程示例运行完成！");
}