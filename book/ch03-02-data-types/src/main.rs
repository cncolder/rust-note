fn main() {
    println!("{}", 0.1 + 0.2);
    println!("{}", 0.1_f32 + 0.2_f32);

    // tuple 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0 as f64 + tup.1 + tup.2 as f64);

    // array 数组
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 显示任意对象
    println!("{:?}", arr);
    // 显示格式化
    println!("{:#?}", arr);
    let _3x5 = [3; 5];
    println!("{:?}", _3x5);
}
