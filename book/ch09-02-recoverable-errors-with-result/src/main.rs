use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };

    println!("{:?}", f);

    // Result 枚举有很多方便的方法, 可以避免使用 match.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败: {:?}", error);
            })
        } else {
            panic!("打开文件失败: {:?}", error)
        }
    });

    println!("{:?}", f);

    File::open("nothing").expect("打开文件 nothing 失败");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 书里有错, 新版 rust 文件句柄已经没有 read_to_string 方法了.
    // match fs::read_to_string("hello.txt") {
    //     Ok(s) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // 有趣的问号, Ok 从表达式返回, Err 直接 return 终止函数, 并且还会自动转换错误类型. 问号是可以连续使用的.
    let s = fs::read_to_string("hello.txt")?;
    Ok(s)
}
