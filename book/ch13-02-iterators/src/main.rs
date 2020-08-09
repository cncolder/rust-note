fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("val: {}", val);
    }

    iterator_map()
}

fn iterator_map() {
    let v1 = vec![1, 2, 3];

    // 迭代器是懒加载的, 未消费的迭代器其实什么也没发生.
    let v2 = v1.iter().map(|x| x + 1);
    println!("{:?}", v2);

    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v3);
}
