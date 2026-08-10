use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. 获取交叉编译目标（cargo 自动设置）
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    println!("cargo:warning=Target: {}", target);
    println!("cargo:warning=Host: {}", host);

    if true {
        return;
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=SYSROOT");
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");

    // 2. 构建 clang 参数
    let mut clang_args: Vec<String> = Vec::new();

    // 关键：指定目标三元组，让 libclang 按目标架构解析
    clang_args.push(format!("--target={}", target));

    // 3. sysroot（交叉编译工具链的根目录）
    //    优先从环境变量读，没有就尝试自动推断
    let sysroot = env::var("SYSROOT").unwrap_or_else(|_| {
        // 常见交叉编译工具链路径，按需修改
        let candidates = [
            format!("/usr/{}", target),
            format!("/opt/{}", target),
            format!("/usr/lib/{}", target),
        ];
        candidates
            .iter()
            .find(|p| std::path::Path::new(p).exists())
            .cloned()
            .unwrap_or_default()
    });

    if !sysroot.is_empty() {
        clang_args.push(format!("--sysroot={}", sysroot));

        // 常见头文件路径，按需添加
        clang_args.push(format!("-I{}/usr/include", sysroot));
        clang_args.push(format!("-I{}/include", sysroot));
    }

    // 4. 额外的 clang 参数（从环境变量读，灵活扩展）
    if let Ok(extra) = env::var("BINDGEN_EXTRA_CLANG_ARGS") {
        clang_args.extend(extra.split_whitespace().map(String::from));
    }

    // 5. 打印调试信息（cargo build -vv 可以看到）
    println!("cargo:warning=Target: {}", target);
    println!("cargo:warning=Host: {}", host);
    println!("cargo:warning=Sysroot: {}", sysroot);
    println!("cargo:warning=Clang args: {:?}", clang_args);

    // 6. 生成绑定
    let mut builder = Builder::default()
        .header("wrapper.h")
        .clang_args(&clang_args)
        // 常用配置，按需调整
        .use_core()                  // 用 core 而不是 std（no_std 场景）
        .ctypes_prefix("::core::ffi") // C 类型前缀
        .derive_default(true)
        .derive_debug(true)
        .derive_copy(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .derive_eq(true)
        // 只生成你需要的，加快编译
        // .allowlist_function("foo_.*")
        // .allowlist_type("foo_.*")
        ;

    // 7. 如果是 nightly，启用文档注释规范化
    if let Ok(rustfmt) = std::process::Command::new("rustup")
        .args(["which", "rustfmt", "--toolchain", "nightly"])
        .output()
    {
        if rustfmt.status.success() {
            let path = String::from_utf8_lossy(&rustfmt.stdout).trim().to_string();
            if !path.is_empty() {
                builder = builder
                    .with_rustfmt(path)
                    .rustfmt_configuration_file(Some(PathBuf::from("rustfmt.toml")));
            }
        }
    }

    let bindings = builder.generate().expect("Failed to generate bindings");

    // 8. 写出到 OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");

    // 9. 告诉 cargo 链接库（按需修改）
    // println!("cargo:rustc-link-lib=static=foo");
    // println!("cargo:rustc-link-search=native={}/usr/lib", sysroot);
}
