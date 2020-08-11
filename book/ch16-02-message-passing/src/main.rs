//! Message Passing 消息传递

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    message_passing();

    multiple_messages_passing();

    multiple_producers();
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // unwrap 的作用是 Result 必须要用一下, 如果接收端把通道销毁了会导致 panic.
        tx.send(val).unwrap();
        // 此处 val 已经被 move 到主线程了.
    });

    // recv 会阻塞当前线程, try_recv 会立即返回 Result.
    let received = rx.recv().unwrap();

    println!("接收: {}", received);
}

fn multiple_messages_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // 此处 vals 也被 move 了.
    });

    // rx 可以当 iter 来用, 但它好像没有 .next() 方法.
    for received in rx {
        // 虽然这里没有 sleep, 仍然是1秒打印1行.
        println!("连续接收: {}", received);
    }
}

fn multiple_producers() {
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
            thread::sleep(Duration::from_secs(1));
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
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("汇总接收: {}", received);
    }
}
