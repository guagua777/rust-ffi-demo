// build.rs
use build_rs::{input, output};

fn main() {
    // 打印构建日志
    output::warning("开始配置 zlib 系统库链接");

    // 获取项目根目录
    let _root = input::manifest_dir();

    // 1. 添加系统库搜索路径 (x86_64‑linux‑gnu 默认库目录)
    output::rustc_link_search_native("/usr/lib/x86_64-linux-gnu");

    // 2. 链接 libz，静态链接；如需动态链接调用 rustc_link_lib_dynamic
    output::rustc_link_lib_static("z");

    // 3. 向下游依赖暴露 zlib 路径环境变量 DEP_Z_*
    output::metadata("lib_path", "/usr/lib/x86_64-linux-gnu/libz.a");
    output::metadata("include_path", "/usr/include");

    // 当头文件改动时重新执行 build.rs
    output::rerun_if_changed("/usr/include/zlib.h");
}