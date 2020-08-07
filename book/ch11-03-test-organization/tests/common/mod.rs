/**
 * 公共模块必须以 name/mod.rs 的形式提供, 否则会被当成集成测试运行.
 */

pub fn setup() {
    println!("公共模块");
}
