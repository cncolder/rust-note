#[allow(unused)]
use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是 {}", result);

    // 所有 &str 都是保存在二进制程序里的, 它们的生命周期是 static, 代表不会释放, 在程序的整个生命周期中都有效, 类似全局变量. 有时候编译器会提示使用 `'static`, 不要盲目听信, 先思考一下是不是哪里写错了.
    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 传说中的野指针, 因为 x 比 r 的生命周期短, 打印的时候 r 指定的位置已经不知道是什么东西了.
/*
#[rustfmt::skip]
fn dangling() {
   {
       let r;                // ---------+-- 'a
                             //          |
       {                     //          |
           let x = 5;        // -+-- 'b  |
           r = &x;           //  |       |
       }                     // -+       |
                             //          |
       println!("r: {}", r); //          |
   }                         // ---------+
}
*/

#[rustfmt::skip]
fn no_dangling() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}

// 虽然指定了返回值的生命周期, 但不代表 result 就能获得这个生命周期. 生命周期是用于连接参数和返回值的.
// fn return_a_string_only<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 结构字段如果是引用类型, 也需要标注生命周期.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Rust 在不断优化编译器, 以减少生命周期的使用.

// 编译器会按三条规则添加生命周期

trait SmartLifetimes {
    // 假设有以下函数
    fn first_word(s: &str) -> &str;
    // 第一条规则: 每个引用类型的参数都将获得生命周期. 多个参数会获得多个不同的生命周期.
    fn first_word1<'a>(s: &'a str) -> &str;
    // 第二条规则: 只有一个输入生命周期时, 关联到输出生命周期.
    fn first_word2<'a>(s: &'a str) -> &'a str;
}

impl<'a> ImportantExcerpt<'a> {
    // 第三条规则: 假如多个输入中有一个是 `&self` 或 `&mut self`, 代表方法, `self` 的生命周期会关联到返回值上.
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 即有生命周期又有泛型的例子
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
