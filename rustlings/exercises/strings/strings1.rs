// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// 多种写法

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 从 str 到 String 的多种写法

    // 1. 初始化
    // String::from("blue")

    // 2. Display::to_string
    // "blue".to_string()

    // 3. std::borrow::ToOwned
    // "blue".to_owned()

    // 4. 根据类型自动判断, 此处是函数的返回值 String
    // "blue".into()

    // 5. 格式化
    // format!("blue")

    /*
     * 就目前官方文档和社区讨论来看:
     * 应该优先选择 1 和 2.
     * 事实上 2 等于 1 等于 3.
     * 4 取决于上下文
     * 5 是调用 2 的宏.
     * 综上所述, 考虑到简洁, 以及与其他类型转化成 String 的一致性, 首选 2.
     * 如果值用于不同函数, 参数类型不定, 4 会很方便.
     */
    "blue".to_string()
}
