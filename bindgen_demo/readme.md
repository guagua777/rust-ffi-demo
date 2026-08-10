1. https://www.bilibili.com/video/BV1whfRY7Ehn
2. https://crates.io/crates/bindgen
3. https://rust-lang.github.io/rust-bindgen/
4. https://docs.rs/bindgen/latest/bindgen/
5. https://github.com/KyleMayes/clang-sys#environment-variables
6. https://www.bilibili.com/video/BV1zH21YEEhW
7. bindgen 为将c头文件转为rust代码的工具
8. https://github.com/mozilla/cbindgen 为将rust代码转为c头文件的工具
9. 


1. 生成rust代码，只需要头文件，不需要库
2. cargo build的时候，需要库，不需要头文件，因为会涉及到链接编辑，link edit，及ld,
    以及需要指定 LD_LIBRARY_PATH 环境变量
3. 生成rust代码的时候，从哪里找哪个头文件，从哪里找，给clang指定头文件的路径，通过-I参数,
4. build的时候，从哪里找哪个库文件，此时就不是clang了，而是rustc了，给rustc传递参数，使用cargo命令，
    cargo:rustc-link-search以及cargo:rustc-link-lib
5. 一个是给clang传递参数，指定头文件的路径，一个是给rustc传递参数，指定库文件的路径
6. 参数：
--allowlist-function/type：仅生成匹配模式的接口，减少冗余代码。
--clang-args：传递包含路径或宏定义给 Clang（如 -I..., -DDEBUG）。
--no-derive-debug / --with-derive-default：控制生成的 Trait 派生 
7. 参数：终极指南：rust-bindgen 命令行工具的20个实用参数与示例解析
https://blog.csdn.net/gitblog_01131/article/details/154710348
8. bindgen --help 中常用参数
9. ‌找不到 libclang‌：设置环境变量 export LIBCLANG_PATH=/path/to/llvm/lib 或使用 find 定位后配置。
10. ‌头文件解析失败‌：检查 --clang-args 是否正确添加了 -I 包含路径或 -D 宏定义，确保 Clang 能预处理头文件。
‌11. 生成代码报错‌：尝试添加 --no-derive-debug 或调整 --rust-target 版本兼容性；对于复杂类型，可手动在代码中修正或使用 blocklist-type 排除 
12. rust-bindgen环境配置全攻略：从零搭建开发环境
    https://blog.csdn.net/gitblog_00889/article/details/154685122
13. bindgen 参数 bindgen --clang-args