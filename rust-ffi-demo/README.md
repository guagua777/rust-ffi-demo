# rust-ffi-demo


1. https://doc.rust-lang.org/nomicon/ffi.html
1. https://jakegoulding.com/rust-ffi-omnibus/
2. https://github.com/shepmaster/rust-ffi-omnibus
3. https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#using-extern-functions-to-call-external-code
4. https://doc.rust-lang.org/rust-by-example/std_misc/ffi.html
5. https://doc.rust-lang.org/cargo/reference/build-scripts.html
6. https://doc.rust-lang.org/nomicon/ffi.html
7. https://github.com/google/snappy/blob/master/snappy-c.h



在 macOS 和 Linux 的大多数 shell 中，可以通过在命令前添加 LD_LIBRARY_PATH=target/debug 来实现这一点
在 Windows 系统上，最简单的做法是在运行示例之前，将编译好的动态库复制到当前工作目录。您只需要 .dll 文件。
On Windows, the simplest course of action is to copy the compiled dynamic library into the current working directory before running the examples. 

https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field