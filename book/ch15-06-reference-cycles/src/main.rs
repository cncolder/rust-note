//! 循环引用
//!
//! 创建循环引用并不容易, 但也不是不可能... RefCell<Rc> 算一种.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

mod lib;

fn main() {
    ref_cycle();

    lib::weak_ref();
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn ref_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a 初始引用计数 = {}", Rc::strong_count(&a));
    println!("a 下一项 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("b 创建后 a 引用计数 = {}", Rc::strong_count(&a));
    println!("b 初始引用计数 = {}", Rc::strong_count(&b));
    println!("b 下一项 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("替换后 b 引用计数 = {}", Rc::strong_count(&b));
    println!("替换后 a 引用计数 = {}", Rc::strong_count(&b));

    // 因为循环引用, 下一行会爆栈.
    // println!("a 下一项 = {:?}", a.tail());
}
