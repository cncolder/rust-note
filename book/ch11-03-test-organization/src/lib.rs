/**
 * 单元测试直接写在 lib 文件里的好处是可以测试私有函数. 测试代码只有 cargo test 时才会运行.
 */

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
