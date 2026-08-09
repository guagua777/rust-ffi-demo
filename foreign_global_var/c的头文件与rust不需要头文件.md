用户问的是我之前说的"头文件 (readline.h 等) — 虽然 Rust FFI 不需要头文件来声明符号，但确认符号存在"这句话是什么意思。

让我用清晰的方式解释 Rust FFI 和 C 在使用外部库时的区别。
好问题！这句话是在对比 **C 语言** 和 **Rust FFI** 使用外部库时的不同机制。让我详细解释：

## C 语言使用 readline 的方式

```c
// C 代码必须 #include 头文件
#include <stdio.h>
#include <readline/readline.h>  // ← 必须包含！

int main() {
    printf("version: %d\n", rl_readline_version);
    return 0;
}
```

**编译过程：**
1. **预处理阶段**：`#include <readline/readline.h>` 把头文件内容插入进来，其中包含了类似这样的声明：
   ```c
   extern int rl_readline_version;
   ```
2. **编译阶段**：编译器因为看到了声明，才知道 `rl_readline_version` 是个什么东西（类型、名字），不会报错
3. **链接阶段**：链接器去 `libreadline.so` 里找这个符号的实际地址

> **关键点**：C 没有头文件声明 → 编译期就报错，根本到不了链接阶段。

---

## Rust FFI 使用 readline 的方式

```rust
// Rust 不需要 include 任何 .h 文件！
// 你自己在 extern "C" 块里手写声明

#[link(name = "readline")]
unsafe extern "C" {
    static rl_readline_version: libc::c_int;  // ← 你自己写的声明
}
```

**编译过程：**
1. **编译阶段**：Rust 编译器直接相信你写的 `extern "C"` 块，**不会去验证** `rl_readline_version` 这个东西是否真的存在于某个库中
2. **链接阶段**：链接器根据 `#[link(name = "readline")]` 去 `libreadline.so` 里找符号，如果找不到才报错

> **关键点**：Rust 不读取 C 头文件，完全信任你手写的声明。所以即使没有 `readline.h`，只要库文件存在，就能编译通过。

---

## 用一个表格总结区别

| 阶段 | C 语言 | Rust FFI |
|------|--------|----------|
| **符号声明来源** | 从 `.h` 头文件 `#include` | 开发者手写 `extern "C"` 块 |
| **编译期检查** | 头文件不存在/没有声明 → 编译失败 | 不检查声明是否真实存在，直接相信你 |
| **链接期检查** | 检查符号在库中是否存在 | 检查符号在库中是否存在 |
| **需要安装 -dev 包的原因** | 主要为了获取 `.h` 头文件 | 主要为了获取 `libreadline.so` 符号链接 |

---

## 举个极端例子

即使你在 Rust 里瞎写一个根本不存在的函数，**编译阶段也不会报错**：

```rust
#[link(name = "readline")]
unsafe extern "C" {
    fn this_function_does_not_exist_12345();  // ← 瞎写的
}

fn main() {
    // 只要不调用它，编译+链接都可能通过！
    // 如果调用了，链接阶段才会报 undefined reference
}
```

而在 C 语言里，没有头文件声明的函数，编译就直接报错了（或至少警告）。

这就是我说的 **"Rust FFI 不需要头文件来声明符号"** 的含义 —— Rust 编译器不会去读 C 头文件，一切以你手写的 `extern "C"` 块为准。