#[allow(unused)]
fn main() {
    let mut s = String::new();
    let s = "初始值".to_string();
    let s = String::from("初始值");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 不会剥夺 s2 所有权.
    s1.push_str(s2);
    println!("s1 is {}, s2 is {}.", s1, s2);

    let mut s = String::from("lo");
    // push 只接受单个字符.
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 注意在这行之后 s1 不再可用. 加号实际是调用 s1 的 add 方法, s1 本身被当成参数传递进去, 与引用值 &s2 相连, 之后返回赋值给 s3. 这个操作看似 copy, 实际不是.
    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! 和 println! 类似, 只不过它用来返回值而不是打印出来, 这种用法在多个字符串相连时比加号更直观.
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);
    // 与加号的区别是 s1 不会被修改.
    println!("s1 还在 {}", s1);

    let hello = String::from("你好");
    // Rust String 不能用下标访问. String 底层是 Vec<u8>, 取出来会是数字, 而且由于 String 是 UTF-8 格式的, 用下标取出的值可能是不完整的 Unicode.
    // let h = &hello[0];
    // 可以获取到切片, 返回值是 &str. 这个操作很危险, 如果分割位置不是字符边界, 程序会崩溃.
    println!("不崩溃会打印'你'字: {}", &hello[0..3]);

    for c in hello.chars() {
        println!("{}", c)
    }

    for b in hello.bytes() {
        println!("{}", b)
    }

    // 有些语言是带音调的, chars() 会把字母和音调拆开, 例如阿三语. Rust 标准库没有提供 letters() 方法.
}
