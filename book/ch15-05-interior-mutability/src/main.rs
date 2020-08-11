//! RefCell
//!
//! 打破借用规则, 将借用规则判断推迟到运行时, 允许可变拥有者和多个不可变拥有者.
//!
//! 引用类型选择:
//! - Rc 允许多个拥有者; Box 和 RefCell 只能有一个拥有者.
//! - Box 在编译时校验不可变借用和可变借用; Rc 在编译时只校验不可变借用; RefCell 只在运行时校验不可变和可变借用.
//! - 因为 RefCell 是在运行时校验可变借用的, 可以修改不可变 RefCell 中的值.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    mutable_borrow_immutable_value();

    multiple_owners();
}

fn mutable_borrow_immutable_value() {
    let x = 5;
    // 不可变值不能以可变方式借用
    // let y = &mut x;
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn multiple_owners() {
    let value = Rc::new(RefCell::new(5));

    // (5)
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // (6 . 5)
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // (10 . 5)
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    println!("value rc = {:?}", Rc::strong_count(&value));
}
