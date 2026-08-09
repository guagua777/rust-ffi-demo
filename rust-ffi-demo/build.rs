// build.rs
fn main() {
    // 构建脚本通过向标准输出 (stdout) 打印信息与 Cargo 通信。
    // Cargo 会将每一行以 cargo:: 开头的行解释为一条指令，该指令会影响软件包的编译。所有其他行都会被忽略。
    // 例如，以下指令会告诉 Cargo 编译时链接 stdc++ 库：
    // 动态链接stdc++库，某些环境下可能不需要这行
    println!("cargo:rustc-link-lib=dylib=stdc++"); // This line may be unnecessary for some environments.
    // 库的搜索路径
    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu");
}


use cc;


// 1.cc编译native.c，输出产物 libnative.a
// cc::Build::new().file("native.c").compile("native");

// 2.告知rustc静态链接native库，rustc生成 -lnative 给到ld
// println!("cargo::rustc-link-lib=static=native");

// // 1.cc编译native.c，输出产物 libnative.a
// cc::Build::new().file("native.c").compile("native");
// Example custom build script.
fn main2() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // cargo::前缀表示cargo的内置指令，rerun-if-changed表示如果src/hello.c文件发生变化，则重新运行build.rs
    println!("cargo::rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}



// fn main() {
//     // 编译C源码，输出 libnative.a
//     cc::Build::new()
//         .file("native.c")
//         .out_dir("./build_out")
//         .compile("native");
    
//     // 将产物目录加入原生库搜索路径
//     println!("cargo::rustc-link-search=native=./build_out");
//     // 静态链接 native，ld自动寻找 libnative.a
//     println!("cargo::rustc-link-lib=static=native");
    
//     // C文件改动就重新构建
//     println!("cargo::rerun-if-changed=native.c");
// }