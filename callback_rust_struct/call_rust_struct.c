typedef void (*rust_callback)(void*, int32_t);
void* cb_target;
rust_callback cb;

int32_t register_callback(void* callback_target, rust_callback callback) {
    cb_target = callback_target;
    cb = callback;
    return 1;
}

void trigger_callback() {
  cb(cb_target, 7); // Will call callback(&rustObject, 7) in Rust.
}


// rust调用c程序，c再回调rust
// 1. 写build.rs 或者直接使用指令来完成也可以
// 2. 编译该c文件
// 3. 给cargo传递指令，让rust从哪里找哪个库文件
// 4. 执行rust程序


// 此处的c代码没有头文件，只是一个完成的c文件，生成库文件
// 如果是有头文件的c代码，可能需要有改动的地方