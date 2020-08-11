//! Patterns 样式
//!
//! 匹配样式需要一个 `match` 表达式, 一个值, 和若干分支. 分支必须详尽匹配所有可能性.
//!
//! 所有赋值和参数其实都是样式匹配过程.

fn main() {
    if_let_pattern();

    while_let_pattern();

    for_pattern();

    let_pattern();

    fn_pattern();

    closure_pattern();
}

/// `if let` 表达式满足复杂需求
///
/// 允许遮敝变量, 但仅限于花括号内部使用.
///
/// 与 `match` 相比, 不检查条件是否详尽匹配, 会有某些情况未覆盖到.
fn if_let_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("使用喜欢的颜色 {} 作为背景", color);
    } else if is_tuesday {
        println!("周二是绿色的一天!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色背景");
        } else {
            println!("使用橙色背景");
        }
    } else {
        println!("使用蓝色背景");
    }
}

/// `while let` 会一直循环, 直到匹配不到时停止.
fn while_let_pattern() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/// `for` 后面的也是样式? 一直以为是解构.
fn for_pattern() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} 索引是 {}", value, index);
    }
}

/// `let` 后面的也是样式!
///
/// `let PATTERN = EXPRESSION;`
fn let_pattern() {
    // 因为 `x` 样式匹配 `5`, 所以 x 是 5.
    let x = 5;

    // Tuple 样式匹配;
    let (x, y, z) = (1, 2, 3);
    let (x, y, _) = (1, 2, 3);
    let (x, ..) = (1, 2, 3);
}

/// `fn` 括号里的也是样式...
///
/// 都是类似解构的功能, 说这里是样式也就不奇怪了.
fn fn_pattern() {
    fn foo(x: i32) {}

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("当前坐标: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

// 闭包参数里的也是样式 0.0
fn closure_pattern() {
    let print_positions = |(x, y)| {
        println!("当前位置: ({}, {})", x, y);
    };

    print_positions((6, 10));
}
