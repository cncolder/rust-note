//! 类型

fn main() {
    type_synonym_alias();

    never_type();

    dynamically_sized_types();
}

fn type_synonym_alias() {
    /// Kilometers 是 i32 的同义词.
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
}

fn never_type() {
    /// `!` 代表 never 类型.
    fn never() -> ! {
        panic!()
    }

    loop {
        let guess = "3";
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            // `continue` 的类型是 `!`, `!` 不会有值, Rust 推测出返回类型是 `u32`.
            // 类似的还有 `panic!` 宏.
            Err(_) => continue,
        };
        println!("{}", guess);
    }
}

/// `DST` Dynamically Sized Types.
fn dynamically_sized_types() {
    // `str` (非 `&str`) 是动态尺寸类型. 直到运行时才能知道它的大小.
    // 所以不能创建一个 `str` 类型的变量.
    let _s1: &str = "Hello there!";

    // 对于所有泛型函数来说
    fn generic<T>(t: T) {}
    // 泛型都有一个 `Sized` 特征边界
    fn generic2<T: Sized>(t: T) {}
    // 除非指定 `?Sized`, 这是 `Sized` 的对立面, 意思是 `T` 可能是也可能不是 `Sized`
    // 问号用法只能用在 `Sized` 上, 其他 trait 都不能用.
    fn generic3<T: ?Sized>(t: &T) {}
}
