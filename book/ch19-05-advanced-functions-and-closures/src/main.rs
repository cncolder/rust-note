fn main() {
    function_pointer();

    function_and_closure();

    returning_closure();
}

/// 函数指针
fn function_pointer() {
    /// 所有函数变量都是函数指针, 并且同时实现了三个闭包特征 `Fn` `FnMut` `FnOnce`.
    /// 所以凡是可以传递闭包的地方, 都可以传递函数.
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    /// `f` 参数后面的 `fn` 是类型, 不是 trait, 如果是闭包, 需要指定 trait `Fn`.
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("答案是 {}", answer);
}

/// 函数与闭包
fn function_and_closure() {
    let numbers = vec![1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    // 所有实现 `Display` 的类型都同时实现了 `ToString`.
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();

    println!("strings {:?}", strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    // 有没有觉得 tuple 结构的初始化方式 `Status::Value(1)` 很像是在调用函数?
    // 其实就是!
    // 所以 `Status::Value` 其实是个函数指针.
    // 有人喜欢这种写法, 有人喜欢闭包写法, 编译后的代码是一样的.
    let statuses: Vec<Status> = (0_u32..20).map(Status::Value).collect();
    let statuses: Vec<Status> = (0_u32..20).map(|n| Status::Value(n)).collect();

    println!("statuses {:?}", statuses);
}

/// 返回闭包
/// 闭包由 trait 呈现, 一般来说如果想返回 trait, 需要先实现它, 之后返回具体的类型.
/// 对闭包来说不可行.
fn returning_closure() {
    // 直接返回闭包是不行的
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // 需要返回闭包 trait object.
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
