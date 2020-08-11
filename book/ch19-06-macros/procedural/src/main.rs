//! 自定义 derive 宏

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// 下面一行会展开代码, 功能相当于为 `Pancakes` 实现 `HelloMacro` 特征.
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
