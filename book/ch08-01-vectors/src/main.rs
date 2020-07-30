#[allow(unused)]
fn main() {
    // 标准用法.
    let v1: Vec<i32> = Vec::new();

    // vec! 是宏, 泛型可以从初始值或后续用法引申出来.
    let v2 = vec![1, 2, 3];

    // 即使是操作子项, 也需要声明成 mut.
    let mut v = vec![];
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    let third = &v[2];

    println!("第三项是 {}", third);

    match v.get(9) {
        Some(n) => println!("第九项是 {}", n),
        None => println!("没有第九项"),
    }

    // 下面一行执行后, 上面的 third 就不能再使用了. 因为 third 是 immutable borrow, 而 push 操作是 mutable borrow, Vec 类型在内存中是连续的, 如果没有足够的连续内存用于新加项时, 整个 Vec 需要移动到新位置, 此时 third 引用的内存会失效.
    v.push(4);
    // println!("第三项是 {}", third);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    // Vec 元素类型必须统一, enum 可以让不同的类型统一.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
