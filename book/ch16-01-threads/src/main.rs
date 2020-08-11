//! Thread 线程
//!
//! Rust 线程指的是操作系统线程, 1:1
//! Go 协程是一种绿色线程, M:N, 因为需要巨大的运行时支持, Rust 以及标准库未提供.

use std::thread;
use std::time::Duration;

fn main() {
    thread_order();

    thread_waiting();

    thread_move_value();
}

/// 线程不保证执行顺序, 主线程线束时子线程强制结束.
fn thread_order() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("子线程 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_waiting() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待子线程结束
    handle.join().unwrap();
}

/// 子线程闭包不能简单的使用外部值, 因为生命周期难以确定.
fn thread_move_value() {
    let v = vec![1, 2, 3];

    // 闭包使用 move 获得 v 所有权.
    let handle = thread::spawn(move || {
        println!("移动值 {:?}", v);
    });

    handle.join().unwrap();
}
