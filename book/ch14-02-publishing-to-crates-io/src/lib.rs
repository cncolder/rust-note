//! # My Crate
//!
//! `my_crate` 是学习使用 cargo 文档以及文档中的测试时写的一些例子.
//!
//! 双斜线加叹号开头的注释是 crate 文档.

/// 加一.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// 三斜线开头的是函数文档, 注意上面的代码块会在 cargo test 时运行测试.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::kinds::PrimaryColor;
pub use crate::kinds::SecondaryColor;
pub use crate::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
