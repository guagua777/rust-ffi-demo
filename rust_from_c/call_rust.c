
/// 声明该方法，该方法在rust库中定义
extern void hello_from_rust();

int main(void) {
    hello_from_rust();
    return 0;
}


// 从哪里找哪个库
// gcc call_rust.c -o call_rust -lrust_from_c -L./target/debug


// 运行
// LD_LIBRARY_PATH=./target/debug ./call_rust