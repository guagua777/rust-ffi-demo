完全使用build.rs
编译c程序，创建c的静态库或者动态库
在build.rs中指定rust程序，链接库的地址，以及相应的参数
1. 主要为链接库的搜索目录
2. 静态库还是动态库，库的名字
总结：去哪里找哪个库

rust中
声明依赖的外部函数
1. 使用#[link]
2. 使用unsafe

声明外部依赖函数
unsafe extern { fn hello(); }
或者是
unsafe extern "C" {
    pub fn crc32(crc: c_ulong, buf: *const u8, len: c_uint) -> c_ulong;
}
一个是有C，一个是没有


rust中
调用外部函数


总结：
1. 编译c程序，生成库，供rust调用，该过程需要使用编译相关的工具和相应的参数
2. 指定rust搜索依赖库的路径和库的名称
3. rust中声明外部函数，使用外部函数

https://doc.rust-lang.org/cargo/reference/build-script-examples.html#using-another-sys-crate