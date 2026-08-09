// build.rs

// build.rs

fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}


use std::process::Command;
use std::env;
use std::path::Path;

// 问题，不具备可移植性
// 所以使用crate 库来进行替换
fn main1() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // C 文件编译成目标文件（通过调用 gcc 
    // Note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/hello.o", out_dir))
                       .status().unwrap();

    // 将该目标文件转换为静态库（通过调用 ar ）                
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    // 最后一步是向 Cargo 反馈，告知其输出位于 out_dir 目录下，
    // 并指示编译器将该 crate 链接到 libhello.a 通过 -l static=hello 标志静态设置。
    println!("cargo::rustc-link-search=native={}", out_dir);
    println!("cargo::rustc-link-lib=static=hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}