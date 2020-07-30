// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// 函数调用时参数数量必须一致, 不支持可选参数或参数默认值.

fn main() {
    call_me(1);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
