//! Box<T>
//!
//! 除了把值保存在堆而非栈上以外, 没有性能优势.
//!
//! 用途:
//! - 当一个值在编译值大小未知
//! - 大数据需要转移所有权又不想拷贝时
//! - 当你只需要值实现部分 trait 不关心具体类型时

use self::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list)
}

/// Lisp 经典的 Cons List 类型
///
/// # Examples
///
/// ```
/// let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
/// ```
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
