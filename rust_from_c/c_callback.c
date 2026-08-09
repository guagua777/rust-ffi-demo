#include <stdint.h>

typedef void (*rust_callback)(int32_t);
rust_callback cb;

int32_t register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
  cb(7); // Will call callback(7) in Rust.
}

/// https://doc.rust-lang.org/nomicon/ffi.html
/// 1. 将该c文件编译为动态库 指定输出文件名，必须以 lib 开头，Rust的链接器才能找到
/// gcc -fPIC -shared -o libextlib.so c_callback.c
/// gcc -fPIC -shared -o c_callback.so c_callback.c
/// 2. rust中调用该动态库，在调用的过程中，需要指定路径和库名