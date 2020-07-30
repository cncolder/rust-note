// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// 不能使用未初始化的变量

fn main() {
    // let x;
    let x = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
