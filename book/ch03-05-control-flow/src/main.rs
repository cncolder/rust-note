fn main() {
    if_demo();

    loop_demo();

    while_demo();

    for_demo();
}

fn if_demo() {
    let number = 6;

    if number % 4 == 0 {
        println!("[if] number is divisible by 4");
    } else if number % 3 == 0 {
        println!("[if] number is divisible by 3");
    } else if number % 2 == 0 {
        println!("[if] number is divisible by 2");
    } else {
        println!("[if] number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // 没有分号结尾的表达式会返回值
    let number = if condition { 5 } else { 6 };

    println!("[if] The value of number is: {}", number);
}

fn loop_demo() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("[loop] The result is {}", result);
}

fn while_demo() {
    let mut number = 3;

    while number != 0 {
        println!("[while] {}!", number);

        number -= 1;
    }

    println!("[while] LIFTOFF!!!");
}

fn for_demo() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("[for] the value is: {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("[for] {}!", number);
    }
    println!("[for] LIFTOFF!!!");
}
