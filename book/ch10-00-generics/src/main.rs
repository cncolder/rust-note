fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("最大数字是: {}", largest(&number_list));
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
