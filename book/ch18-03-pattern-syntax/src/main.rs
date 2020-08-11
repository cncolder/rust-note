//! Pattern Syntax 样式语法
//!
//! 想起了 F#, 有内味了.

fn main() {
    let x = 1;
    match x {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("万物"),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("五十"),
        // 这里的 `y` 是遮蔽变量, 而不是匹配成 `Some(10)`.
        Some(y) => println!("匹配 y = {:?}", y),
        _ => println!("默认 x = {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("一或二"),
        3 => println!("三"),
        _ => println!("万物"),
    }

    let x = 5;
    match x {
        // 范围样式只允许数字或字符
        1..=5 => println!("一到五"),
        _ => println!("万物"),
    }

    let x = 'c';
    match x {
        // 等号有点丑, 慢慢习惯吧.
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("{:?}, x {}, y {}", p, a, b);

    // 以下是一种简写, 完整写法是 `let Point { x: x, y: y } = p;`, 因为我们不是也没必要匹配字段名, 只是想匹配字符值, 所以 Rust 约定下面的写法等同于完整写法. 记住匹配不是解构.
    let Point { x, y } = p;
    println!("{:?}, x {}, y {}", p, x, y);

    // 明确指定部分字段值进行匹配.
    match p {
        Point { x, y: 0 } => println!("x 坐标轴上 {}", x),
        Point { x: 0, y } => println!("y 坐标轴上 {}", y),
        Point { x, y } => println!("不在坐标轴上: ({}, {})", x, y),
    }

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        // 嵌套样式
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // `_` 代表忽略样式, 从而达到忽略这个参数的目的.
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        // 其中之一是 None 时赋值.
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("一些数字: {}, {}, {}", first, third, fifth),
    }

    let s = Some(String::from("Hello!"));
    // `_` 是真正忽略赋值过程, 所以 s 没有被 move.
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };
    match origin {
        // Point3D { x, y: _, z: _ } => println!("x {}", x),
        Point3D { x, .. } => println!("x {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("一些数字: {}, {}", first, last),
        // `..` 必须保证编译器能懂.
        // (.., second, ..) => println!("Some numbers: {}", second),
    }

    let num = Some(8);
    let lucky_num = 8;
    match num {
        // _match guard_ 相当于给分支设置额外条件, 注意这里的 `x` 就是样式里的 `x`.
        Some(x) if x < 5 => println!("小于五: {}", x),
        Some(x) if x == lucky_num => println!("幸运数: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = false;
    match x {
        // 这个 `if` 条件对 4 5 6 同时生效.
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };
    match msg {
        Message3::Hello {
            // 即要匹配样式, 又要得到变量.
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
