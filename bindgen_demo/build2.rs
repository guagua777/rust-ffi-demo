use std::env;
use std::path::PathBuf;

// 包含内容
// 1. 编译c文件为静态库
// 2. 生成rust绑定代码
fn main() {
    // 指定c库的目录
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("hello")
        // 转变为绝对路径，确保路径是绝对的
        // 给rustc-link-search传递
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

    // 指定头文件
    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("hello.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // 中间文件路径
    // This is the path to the intermediate object file for our library.
    let obj_path = libdir_path.join("hello.o");
    // 生成的静态库的路径
    // This is the path to the static library file.
    let lib_path = libdir_path.join("libhello.a");

    // 传递给Cargo的指令
    // 告诉cargo从哪里查找静态库
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // 告诉cargo链接具体哪个库
    // Tell cargo to tell rustc to link our `hello` library. Cargo will
    // automatically know it must look for a `libhello.a` file.
    println!("cargo:rustc-link-lib=hello");

    // Run `clang` to compile the `hello.c` file into a `hello.o` object file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("hello.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile object file");
    }

    // Run `ar` to generate the `libhello.a` file from the `hello.o` file.
    // Unwrap if it is not possible to spawn the process.
    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        // 指定头文件
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // 写入目标文件
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
