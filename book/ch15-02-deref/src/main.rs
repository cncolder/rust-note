//! & ref 引用 指针
//! * deref 解引用 取值
//!
//! 调用函数或方法时实现 `Deref trait` 的类型会发生 _Deref 胁迫_.
//! 在编译时尝试不断调用 `deref()` 直到满足参数类型.
//!
//! 流程如下:
//! - `m` 类型是 `MyBox<String>`, 因为实现了 `Deref`, `&MyBox<String>` 调用 `deref()` 会转化成 `&String`.
//! - `String` 也实现了 `Deref`, `&String` 调用 `deref()` 后会返回字符串切片 `&str`.
//!
//! Deref 胁迫条件:
//! - 从 `&T` 到 `&U`, `T: Deref<Target=U>`
//! - 从 `&mut T` 到 `&mut U`, `T: DerefMut<Target=U>`
//! - 从 `&mut T` 到 `&U`, `T: Deref<Target=U>`
//!
//! 不可变引用无法转变成可变引用, 因为借用规则.

use std::ops::Deref;

fn main() {
    ref_and_deref();

    box_as_ref();

    my_box_as_ref();

    hello_rust();
}

fn ref_and_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/// Box<T> 和引用类似
fn box_as_ref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

/// 类似 Box 的 tuple 结构.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 MyBox Deref
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn my_box_as_ref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

/// Deref 胁迫, `&MyBox<String>` -> `&String` -> `&str`
fn hello_rust() {
    let m = MyBox::new(String::from("Rust"));

    // 等效写法 hello(&(*m)[..]);
    hello(&m);
}
