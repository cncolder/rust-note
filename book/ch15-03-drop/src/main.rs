//! Drop trait
//!
//! 实现 `Drop` 的类型在脱离作用域后, Rust 自动调用 `drop` 方法用来清理资源.

fn main() {
    create_custom_smart_pointer();
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // `drop` 方法不能手动调用, 因为 Rust 总是会自动调用, 两次调用会产生错误.
    fn drop(&mut self) {
        println!("清理 CustomSmartPointer `{}`", self.data);
    }
}

fn create_custom_smart_pointer() {
    let c = CustomSmartPointer {
        data: String::from("一"),
    };

    let d = CustomSmartPointer {
        data: String::from("二"),
    };

    println!("CustomSmartPointer 已创建.");

    // 如果需要提前清理内存, 可以使用 std::mem::drop. 这个方法会调用对象的 `drop` 方法, 并记录 `c` 已经被 `drop`, 但你不能手动调用对象的 `drop` 方法.
    drop(c);

    // 代码运行到这里时会倒序清理 `d` 和 `c`.
}
