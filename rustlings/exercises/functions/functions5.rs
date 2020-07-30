// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// Rust 可以用省略分号的方式省略 return, 不知道这种设计是好是坏.

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
