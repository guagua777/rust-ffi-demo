
// 默认是静态库
fn main1() {
    cc::Build::new()
        .file("c_callback.c")
        .compile("extlib");
}

use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // 手动编译为动态库
    Command::new("gcc")
        .args(&["-fPIC", "-shared", "-o"])
        .arg(format!("{}/libextlib.so", out_dir))
        .arg("c_callback.c")
        .status()
        .unwrap();
    
    // 告诉cargo搜索路径和库名
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=extlib");
}