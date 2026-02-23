# 并发编程模块 (concurrency)

## 📖 模块概述

Rust 的并发编程设计理念是"无畏并发"（Fearless Concurrency），通过所有权系统在编译时防止数据竞争。本模块讲解线程、消息传递、共享状态等并发编程概念。

## 🎯 学习目标

- 理解 Rust 的并发安全模型
- 掌握线程的创建和管理
- 学会使用消息传递进行线程通信
- 掌握共享状态的同步机制
- 理解常见的并发模式

## 📚 内容目录

### 1. 线程基础 (`modern_sync_threads`)

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {
    for i in 1..5 {
        println!("线程中: {}", i);
        thread::sleep(Duration::from_millis(100));
    }
});

// 等待线程完成
handle.join().unwrap();

// 带返回值的线程
let handle = thread::spawn(|| {
    "线程完成".to_string()
});
let result = handle.join().unwrap();
```

### 2. 消息传递 (`modern_message_passing`)

```rust
use std::sync::mpsc;
use std::thread;

// 创建通道
let (tx, rx) = mpsc::channel();

// 发送端
thread::spawn(move || {
    let messages = vec!["消息1", "消息2", "消息3"];
    for msg in messages {
        tx.send(msg).unwrap();
    }
});

// 接收端
for received in rx {
    println!("收到: {}", received);
}

// 多生产者
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone();
let tx2 = tx.clone();

thread::spawn(move || {
    tx1.send("来自线程1").unwrap();
});

thread::spawn(move || {
    tx2.send("来自线程2").unwrap();
});
```

### 3. 共享状态 (`modern_shared_state`)

```rust
use std::sync::{Arc, Mutex, RwLock};

// Mutex 互斥锁
let data = Arc::new(Mutex::new(0));
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    let mut num = data_clone.lock().unwrap();
    *num += 1;
});

// RwLock 读写锁
let data = Arc::new(RwLock::new(vec![1, 2, 3]));

// 多个读取者
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    let read = data_clone.read().unwrap();
    println!("读取: {:?}", *read);
});

// 单个写入者
let data_clone = Arc::clone(&data);
thread::spawn(move || {
    let mut write = data_clone.write().unwrap();
    write.push(4);
});
```

### 4. 同步机制 (`modern_synchronization`)

```rust
use std::sync::{Barrier, Once};

// Barrier 屏障同步
let barrier = Arc::new(Barrier::new(3));

for i in 0..3 {
    let barrier = Arc::clone(&barrier);
    thread::spawn(move || {
        println!("线程 {} 准备", i);
        barrier.wait();  // 等待所有线程到达
        println!("线程 {} 继续", i);
    });
}

// Once 一次性初始化
static INIT: Once = Once::new();

INIT.call_once(|| {
    // 只执行一次的初始化代码
    println!("初始化");
});
```

### 5. 生产者-消费者模式 (`modern_producer_consumer`)

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

struct BoundedQueue<T> {
    queue: Mutex<VecDeque<T>>,
    capacity: usize,
}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }
    
    fn push(&self, item: T) -> Result<(), T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() < self.capacity {
            queue.push_back(item);
            Ok(())
        } else {
            Err(item)
        }
    }
    
    fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}
```

### 6. 工作池模式 (`modern_work_pool`)

```rust
use std::sync::{Arc, Mutex, mpsc};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        Self { workers, sender }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
```

### 7. Web 服务器并发 (`web_server_concurrent_handling`)

```rust
use std::sync::{Arc, Mutex};

struct RequestStats {
    total: Arc<Mutex<u32>>,
    completed: Arc<Mutex<u32>>,
    failed: Arc<Mutex<u32>>,
}

fn handle_request(id: u32, stats: &RequestStats) {
    *stats.total.lock().unwrap() += 1;
    
    // 处理请求...
    let success = process_request();
    
    if success {
        *stats.completed.lock().unwrap() += 1;
    } else {
        *stats.failed.lock().unwrap() += 1;
    }
}

// 并发处理多个请求
let stats = RequestStats::new();
let mut handles = vec![];

for i in 0..10 {
    let stats_clone = stats.clone();
    handles.push(thread::spawn(move || {
        handle_request(i, &stats_clone);
    }));
}

for handle in handles {
    handle.join().unwrap();
}
```

### 8. 数据库连接池 (`database_connection_pool`)

```rust
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<DbConnection>>>,
    max_size: usize,
}

impl ConnectionPool {
    fn new(size: usize) -> Self {
        let mut connections = VecDeque::new();
        for i in 0..size {
            connections.push_back(DbConnection::new(i));
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
        let mut pool = self.connections.lock().unwrap();
        if pool.len() < self.max_size {
            pool.push_back(conn);
        }
    }
}
```

## 🚀 运行示例

```bash
# 运行并发模块
cargo run concurrency

# 运行测试
cargo test concurrency
```

## 📊 并发模型对比

```
┌─────────────────────────────────────────────────────────────┐
│                    Rust 并发模型                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   消息传递 (MPSC)                     │   │
│  │  ┌───────┐      ┌───────┐      ┌───────┐           │   │
│  │  │ 生产者 │─────>│ 通道  │─────>│ 消费者 │           │   │
│  │  └───────┘      └───────┘      └───────┘           │   │
│  │                                                     │   │
│  │  优点: 无数据竞争, 易于理解                          │   │
│  │  缺点: 有拷贝开销                                    │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                   共享状态 (Mutex/RwLock)            │   │
│  │  ┌───────┐      ┌───────┐      ┌───────┐           │   │
│  │  │ 线程1 │─────>│       │<─────│ 线程2 │           │   │
│  │  └───────┘      │ Mutex │      └───────┘           │   │
│  │                 │ 数据   │                          │   │
│  │  ┌───────┐      │       │      ┌───────┐           │   │
│  │  │ 线程3 │─────>│       │<─────│ 线程4 │           │   │
│  │  └───────┘      └───────┘      └───────┘           │   │
│  │                                                     │   │
│  │  优点: 共享内存, 无拷贝                              │   │
│  │  缺点: 需要小心管理锁                                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## 📝 练习题

### 初级
1. 创建一个线程，打印 1 到 10
2. 使用通道在两个线程间传递消息
3. 使用 `Arc<Mutex<T>>` 在线程间共享计数器

### 中级
1. 实现一个简单的线程池
2. 使用 `RwLock` 实现一个并发缓存
3. 实现生产者-消费者模式

### 高级
1. 实现一个异步任务调度器
2. 设计一个并发安全的数据结构
3. 实现一个简单的 actor 模型

## 🔗 相关资源

- [Rust 并发编程](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust 异步编程](https://rust-lang.github.io/async-book/)
- [Tokio 文档](https://docs.rs/tokio)

## ⚠️ 常见陷阱

### 1. 死锁
```rust
// ❌ 可能死锁
let data1 = Arc::new(Mutex::new(0));
let data2 = Arc::new(Mutex::new(0));

// 线程1: 先锁 data1，再锁 data2
// 线程2: 先锁 data2，再锁 data1
// 可能导致死锁！

// ✅ 解决方案：统一锁顺序或使用 try_lock
```

### 2. 数据竞争
```rust
// ❌ 数据竞争（Rust 会阻止编译）
let mut data = vec![1, 2, 3];
let ref1 = &data[0];
let ref2 = &mut data;  // 编译错误！

// ✅ 使用 Mutex 保护
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
```

### 3. 忘记 join
```rust
// ❌ 主线程可能在线程完成前退出
thread::spawn(|| {
    // 这可能不会执行完
});

// ✅ 等待线程完成
let handle = thread::spawn(|| {
    // ...
});
handle.join().unwrap();
```

## 📊 学习检查清单

- [ ] 理解线程的创建和管理
- [ ] 掌握消息传递模式
- [ ] 会使用 Mutex 和 RwLock
- [ ] 理解 Arc 的作用
- [ ] 掌握 Barrier 和 Once
- [ ] 会实现生产者-消费者模式
- [ ] 理解工作池模式
- [ ] 能够避免常见并发陷阱
