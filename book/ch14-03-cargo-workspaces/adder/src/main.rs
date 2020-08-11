//! 工作空间
//! 所有 crate 共享根目录 Cargo.lock
//! 统一运行 build test
//! publish 需要切换到每个目录下单独运行

use add_one;

fn main() {
    let num = 10;

    println!("{} 加一等于 {}!", num, add_one::add_one(num));
}
