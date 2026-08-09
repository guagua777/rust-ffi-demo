use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // 手动编译为动态库
    Command::new("gcc")
        .args(&["-fPIC", "-shared", "-o"])
        // 库的名字要跟rust中指定的库名保持一致
        .arg(format!("{}/libextlib.so", out_dir))
        .arg("call_rust_struct.c")
        .status()
        .unwrap();
    
    // 告诉cargo搜索路径和库名
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=extlib");
}