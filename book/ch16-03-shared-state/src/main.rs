//! Shared-State 共享状态
//!
//! Mutex 互斥锁, 显示调用 lock(), drop 时自动 unlock.
//!
//! Arc 原子引用计数, 线程安全的 Rc.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    mutex_auto_unlock();

    share_mutex();
}

fn mutex_auto_unlock() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn share_mutex() {
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

    println!("结果: {}", *counter.lock().unwrap());
}
