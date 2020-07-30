fn main() {
    use std::collections::HashMap;

    // HashMap 的 key value 必须是相同类型
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 传入 HashMap 的值如果不能 Copy, 会剥夺所有权. teams 在存入 hash 后无效.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // zip 类似 lodash, collect 类似 Object.fromEntries
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);

    let name = String::from("Blue");
    // 取值时比较 key 是否相等.
    println!("{:?}, {:?}", scores.get("Blue"), scores.get(&name));

    // for 操作得到的是 Tuple?
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 插入相同的 key 会替换值.
    scores.insert(String::from("Blue"), 20);
    println!("after insert {:?}", scores);

    // 如果没有 key 则赋值.
    scores.entry(String::from("Blue")).or_insert(30);
    println!("after entry or_insert {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // count 是引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
