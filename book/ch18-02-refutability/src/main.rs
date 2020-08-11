//! 样式分 refutable(存疑) 和 irrefutable(确定) 两种.

fn main() {
    let some_option_value: Option<u8> = Some(1);
    // `let` `for` `fn` 需要确定样式, 因为没有也无法覆盖所有情形.
    // let Some(x) = some_option_value;

    // 条件样式匹配则不存在这个问题
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // 匹配确定样式时编译器会抱怨
    if let x = 5 {
        println!("{}", x);
    }

    println!("Hello, world!");
}
