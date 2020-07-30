fn main() {
    let some_u8_value = Some(0_u8);

    // 为什么不是 if match
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // 愚蠢的设计
    else {
        println!("no three");
    }
}
