// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// 只有带 mut 的变量是可以修改的

fn main() {
    // let x = 3;
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
