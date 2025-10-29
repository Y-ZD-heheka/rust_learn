//! # 并发编程模块
//!
//! 这个模块演示了Rust的并发编程特性，包括线程、消息传递和共享状态。

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

/// 演示创建线程
pub fn threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 等待子线程完成
}

/// 演示消息传递
pub fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 这会编译错误，因为val的所有权已转移
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 演示共享状态
pub fn shared_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/// 演示条件变量
pub fn condition_variables() {
    use std::sync::Condvar;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
        println!("Child thread: started = {}", *started);
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("Main thread: started = {}", *started);
}

/// 演示通道的多发送者
pub fn multiple_senders() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/// 运行并发编程示例
pub fn run_concurrency_examples() {
    println!("=== 并发编程示例 ===");
    threads();
    message_passing();
    shared_state();
    condition_variables();
    multiple_senders();
}