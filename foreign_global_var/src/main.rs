
use libc;

// 不需要指定搜索路径，因为 Rust 会自动搜索系统库
#[link(name = "readline")]
unsafe extern "C" {
    static rl_readline_version: libc::c_int;
}

fn main() {
    println!("You have readline version {} installed.",
             unsafe { rl_readline_version as i32 });
}


// 让我先查看项目结构和代码，了解具体情况。
// 用户遇到了一个 Rust FFI 链接错误，提示找不到 `-lreadline` 库。让我先看看项目结构和代码，了解具体情况。

// 问题很清楚：链接器找不到 `readline` 系统库。在 Linux 系统上需要安装 `libreadline-dev` 开发包。

// ## 解决方案

// 你的代码通过 `#[link(name = "readline")]` 声明了要链接 `readline` 库，但系统中没有安装该库的开发文件。运行以下命令安装：

// ```bash
// # Debian/Ubuntu 系列
// sudo apt-get install libreadline-dev

// # Fedora/RHEL 系列
// sudo dnf install readline-devel

// # Arch Linux
// sudo pacman -S readline
// ```

// 安装完成后，再次运行 `cargo run` 即可。

// ---

// ## 原理说明

// 代码中第 4 行的 `#[link(name = "readline")]` 属性告诉 Rust 链接器：
// - 查找系统库 `libreadline.so`（Linux）或 `libreadline.dylib`（macOS）
// - 解析第 6 行声明的外部符号 `rl_readline_version`

// `libreadline-dev` 包包含：
// - **头文件** (`readline.h` 等) — 虽然 Rust FFI 不需要头文件来声明符号，但确认符号存在
// - **链接脚本/符号链接** (`libreadline.so` → `libreadline.so.8`) — 让 `-lreadline` 能找到正确的动态库
// - **pkg-config 配置** — 供构建系统使用

// 这是 Rust FFI 开发中非常常见的坑：**代码中声明了 `#[link]`，但系统缺少对应的 `-dev`/`-devel` 包**。