use gui::{Button, Draw, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            // 定义 `Screen` 时我们并不知道别人会新增 `SelectBox` 类型, 我们只关心它实现了 `Draw` 特征, 这有点类似动态语言里的鸭子类型, 是否实现了 `Draw` 特征是在运行时判断的, 有个指针搜索过程, 并妨碍方法内联等优化.
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/// 选择框
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}
