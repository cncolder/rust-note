fn main() {
    another_function(5);

    let y = {
        let x = 3;
        // 结尾不加分号会赋值给 y
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("The value of five(): {}", five());

    println!("The value of plus_one(five()): {}", plus_one(five()));
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
