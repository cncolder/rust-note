//! 单线程 Web 服务器

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    server();
}

// 启动服务器
fn server() {
    // 绑定端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("服务器已启动 {:?} ...", listener.local_addr().unwrap());
    println!();

    // incoming 是个流 iter
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

/// 处理连接
/// 看起来读取流不需要 mut, 实际上 TcpStream 在内部记录哪些数据已经返回给用户.
fn handle_connection(mut stream: TcpStream) {
    // 缓冲区 1024
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("请求:");
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    // 二进制字符串
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "../index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "../404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
