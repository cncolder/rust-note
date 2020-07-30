use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("谜底是: {}", secret_number);

    loop {
        println!("请输入你的答案.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的答案: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小!"),
            Ordering::Greater => println!("太大!"),
            Ordering::Equal => {
                println!("猜对啦!");
                break;
            }
        }
    }
}
