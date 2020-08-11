//! Rc 引用计数
//!
//! 用于一个对象有多个拥有者.
//!
//! Rc::clone 用来增加引用计数.
//!
//! Rc 实现 `Drop trait` 用来减少引用计数.

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    cons_list_demo();
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn cons_list_demo() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    println!("a 创建后引用计数是 {}", Rc::strong_count(&a));
    // 使用 clone 增加引用计数, 不会真的拷贝值.
    // 两种方式作用相同, Rust 约定用 `Rc::clone`, 应该是因为偷懒起了 clone 这个容易混淆的名字.
    let b = Cons(3, Rc::clone(&a));

    println!("b 创建后引用计数是 {}", Rc::strong_count(&a));

    {
        let c = Cons(4, a.clone());
        println!("c 创建后引用计数是 {}", Rc::strong_count(&a));
    }

    println!("c 回收后引用计数是 {}", Rc::strong_count(&a));
}
