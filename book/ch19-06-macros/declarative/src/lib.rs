//! # Macros 宏
//!
//! 希望我能看懂这个章节. 🙏
//!
//! ## 宏分三种:
//!
//! - `#[derive]` 宏用于结构和枚举
//! - Attribute-like 宏给任意目标添加自定义属性
//! - Function-like 宏看起来像函数调用, 实际是在操作参数名.
//!
//! ## 宏与函数的区别:
//!
//! 宏是一种用代码来写其他代码的途径, 即 _元编程_. 例如 `derive` `println!` `vec!`, 它们会展开并产生很多代码.
//!
//! 元编程可以节省很多代码, 函数也有这个作用, 但宏能做的更多.
//!
//! 函数签名必须声明参数和数量和类型, 宏可以接受任意数量的参数, 例如 `println("")` 和 `println("{}", 1)`. 宏是在编译时展开的, 例如宏可以为给定类型实现 trait, 函数则不行, 因为函数是运行时调用的, 实现 trait 只能在编译时.
//!
//! 宏比函数难写, 难理解.
//!
//! 宏必须先定义再调用, 函数没有这个限制.

/// 导出宏, 整个 crate 都可用.
#[macro_export]
/// 声明宏
/// 定义宏时, 名称不带叹号.
/// 花括号内是宏的内容.
/// 2020-08-11 目前为止宏定义代码即没有提示也不支持格式化.
/// `macro_rules!` 在奇特情况下存在问题. 未来会有其他替代方案. 尽可能先不写宏.
///
/// # Examples
///
/// `vec![1, 2, 3];`
/// ```
/// {
///     let mut temp_vec = Vec::new();
///     temp_vec.push(1);
///     temp_vec.push(2);
///     temp_vec.push(3);
///     temp_vec
/// }
/// ```
macro_rules! vec {
    // 类似 `match` 表达式的样式匹配, 匹配 `( $( $x:expr ),* )` 成功, 产出代码.
    // 复杂的宏会有多条分支, 匹配失败会产生错误. 样式语法用来匹配 Rust 代码.
    // 样式用括号包裹, `$()` 匹配任意 Rust 表达式并赋值给 `$x`.
    // `,*` 类似正则, 匹配0个以上的逗号.
    // 当调用 `vec![1, 2, 3];` 时, `$x` 匹配三次.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // 此处与参数匹配相对应, `$x` 替换后产出代码, 匹配次数和产出次数相同.
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
