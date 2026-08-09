// src/main.rs

// 使用构建脚本生成的代码
// rustc 定义的宏
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", message());
}