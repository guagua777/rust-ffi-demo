让我先看看你的build.rs和Cargo.toml的内容，然后为你解答。

让我查看一下实际的输出目录结构来确认位置。
用户问 build.rs 输出的 extlib 库输出到哪里去了。这是关于 cc crate 和 cargo build 系统的问题。让我解释一下 cc crate 的输出目录。

cc crate 的 `.compile()` 方法会把编译好的静态库输出到 cargo 的 OUT_DIR 环境变量指定的目录中，通常是在 target/debug/build/<package-name>-<hash>/out/ 目录下。

同时，cc crate 会自动打印 cargo:rustc-link-search=native=... 和 cargo:rustc-link-lib=static=... 这样的指令，让 cargo 自动找到并链接这个库，用户不需要手动指定路径。

让我检查一下项目的 target 目录，看看能不能找到实际的输出位置。

找到了！cc crate 编译输出的静态库在 target/debug/build/rust_from_c-00d3709cb00038e8/out/libextlib.a。现在让我给用户一个清晰的解答。
找到了！`cc` crate 编译输出的库位置如下：

## 实际输出路径

```
target/debug/build/rust_from_c-00d3709cb00038e8/out/libextlib.a
```

你可以清楚看到 `libextlib.a` 就在那里。

---

## 详细说明

### 1. 输出目录规则

`cc` crate 的 `.compile("extlib")` 方法会将编译产物输出到 Cargo 指定的 **`OUT_DIR`** 环境变量目录中，完整路径结构为：

```
target/<profile>/build/<package-name>-<hash>/out/
```

| 部分 | 说明 |
|------|------|
| `<profile>` | `debug` 或 `release`，取决于构建模式 |
| `<package-name>` | 你的包名，这里是 `rust_from_c` |
| `<hash>` | 一个唯一哈希值，用于区分不同构建 |
| `out/` | build script 的标准输出目录 |

### 2. 输出了什么文件

在 `out/` 目录下有：
```
libextlib.a          ← 编译好的静态库（注意是 .a 静态库，不是 .so 动态库！）
db3b6bfb...c_callback.o  ← 编译过程中的目标文件
flag_check / flag_check.c  ← cc crate 内部用于编译器特性检测的临时文件
```

### 3. ⚠️ 注意：这是静态库！

`cc::Build::new().compile("extlib")` 默认编译输出的是 **静态库**（`libextlib.a`），而不是动态库（`libextlib.so`）。

Cargo 会**自动**处理这个静态库的链接，因为 `cc` crate 在背后会输出以下 cargo 指令：
```
cargo:rustc-link-search=native=/path/to/out
cargo:rustc-link-lib=static=extlib
```

所以你**不需要**手动告诉 Rust 去哪里找这个库，`#[link(name = "extlib")]` 配合 `cc` crate 会自动完成链接。

---

## 如果一定要输出动态库

如果需要生成动态库（.so），`cc` crate 默认不直接支持，可以改用 `build.rs` 手动调用 gcc：

```rust
use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // 手动编译为动态库
    Command::new("gcc")
        .args(&["-fPIC", "-shared", "-o"])
        .arg(format!("{}/libextlib.so", out_dir))
        .arg("c_callback.c")
        .status()
        .unwrap();
    
    // 告诉cargo搜索路径和库名
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=extlib");
}
```

但通常用 `cc` crate 输出的静态库就足够了，Cargo 会自动处理好一切。