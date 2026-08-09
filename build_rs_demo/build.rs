use build_rs::{input, output};

// output 为 build.rs 提供了一个类型安全的输出接口，避免了直接使用 println!() 的不安全性。
// input 为 build.rs 提供了一个类型安全的输入接口，避免了直接使用 env::var() 的不安全性。

fn main() {
    // 1.读取环境变量（类型安全）
    let target_dir = input::target_dir();
    let manifest_dir = input::manifest_dir();

    // 2.静态链接 libz，等价 println!("cargo::rustc-link-lib=z")
    output::rustc_link_lib_static("z");

    // 3.添加原生库搜索路径
    output::rustc_link_search_native("/usr/lib/x86_64-linux-gnu");

    // 4.条件编译标记
    output::rustc_cfg("use_zlib");

    // 5.文件改动时重新运行build.rs
    output::rerun_if_changed("native.c");

    // 6.向下游 crate 传递 metadata 环境变量 DEP_XXX_*
    output::metadata("zlib_path", "/usr/lib/libz.a");

    // 7.打印构建警告
    output::warning("正在编译 zlib FFI 代码");
}