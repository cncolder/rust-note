//! Unsafe Rust

use std::slice;

fn main() {
    raw_pointers();

    raw_pointer_to_address();

    unsafe_fn();

    wrap_unsafe_code();

    take_another_memory();

    ffi_example();

    add_to_static_count(3);

    an_union();
}

/// 可以直接创建原始指针, 解引用必须在 `unsafe` 代码块内.
fn raw_pointers() {
    let mut num = 5;

    // 不可变原始指针
    let r1 = &num as *const i32;
    // 可变原始指针
    let r2 = &mut num as *mut i32;

    println!("*count r1 {:?}, *mut r2 {:?}", r1, r2);

    // unsafe 允许可变指针和不可变指针同时指向一个地址.
    unsafe {
        println!("r1 {}", *r1);
        println!("r2 {}", *r2);
    }
}

fn raw_pointer_to_address() {
    let address = 0x012345_usize;
    // 指定任意内存地址的指针.
    let r = address as *const i32;

    println!("address r {:?}", r);
}

fn unsafe_fn() {
    // unsafe 函数体也是处在 unsafe 代码块之中的, 不必再写 unsafe.
    unsafe fn dangerous() {
        println!("unsafe function");
    }

    // unsafe 函数必须在 unsafe 代码块内调用.
    unsafe {
        dangerous();
    }
}

fn wrap_unsafe_code() {
    // slice 原地分割成两半. 虽然函数内部用到了 unsafe, 但函数本身是安全的.
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        // slice 可变指针.
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    println!("split {:?}, {:?}", a, b);
}

fn take_another_memory() {
    let address = 0x01234_usize;
    let r = address as *mut i32;

    // 从内存中随便取10个数字长度的数据, 不知道会取到什么. 别用这个值, 如果 Rust 发现不是 i32 会异常退出.
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10) };

    // println!("从内存中随便取10个数字 {:?}", slice);
}

/// _Foreign Function Interface (FFI)_ 编程语言定义函数的一种方法, 允许其他语言调用.
fn ffi_example() {
    // 使用 C 语言 abs 函数.
    // 如果用到的函数在其他库, 需要明确在运行时链接, 如果链接失败会 panic.
    // #[link(name = "my_c_library")]
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("C 语言 -3 的绝对值是: {}", abs(-3));
    }
}

/// 使用 extern 关键字导出函数供其他语言使用, 省略代码块, 直接写在 fn 前面.
/// no_mangle 防止 Rust 编译混淆函数名.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("从 C 语言调用 Rust 函数!");
}

/// Rust 是有全局变量的, 必须注明类型, 生命周期 `&'static` 可以省略.
/// 静态常量在内存中的地址是固定的, 无论在哪儿使用, 总是获得相同数据.
/// 习惯上使用大写加下划线.
static HELLO_WORLD: &str = "Hello, world!";

/// 静态常量允许修改, 修改操作是不安全的, 在多线程操作会引发数据竞争.
fn add_to_static_count(inc: u32) {
    static mut COUNTER: u32 = 0;

    unsafe {
        COUNTER += inc;
        println!("static COUNTER {}", COUNTER);
    }
}

/// 不安全特征
/// 如果特征里的某个方法是不安全的, 那么整个特征和实现都是不安全的.
unsafe trait Foo {}
unsafe impl Foo for i32 {}

/// Union 跟 Struct 长的很像, 主要是用来跟 C 语言打交道的, Union 里所有字段共享一块内存, 也就是说 Union 同一时刻只能呈现一个字段的值, 指定其他字段值时会顶替当前字段, 所以访问字段是不安全的.
fn an_union() {
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    let u = MyUnion { f1: 1 };

    unsafe {
        println!("union f1 {:?}", u.f1);
    }
}
