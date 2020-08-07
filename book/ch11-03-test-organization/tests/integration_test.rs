/**
 * tests 目录是写死的, Rust 假定这个目录是集成测试, 目录里的每个文件都是单独的 crate.
 */

use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    assert_eq!(4, adder::add_two(2));
}
