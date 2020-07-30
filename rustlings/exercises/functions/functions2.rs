// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// 参数类型必须明确指定, 无法推导.

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
