// src/z_user.c

// 我怎么知道头文件叫zlib.h？是固定惯例，还是哪里设置的？
#include "zlib.h"

// … rest of code that makes use of zlib.


// 好问题！让我先看一下你的 C 文件里实际是怎么写的，然后给你详细解释。
// 用户问的是：怎么知道头文件叫 zlib.h？是固定惯例还是哪里设置的？

// 这是个好问题。让我先看看 z_user.c 文件里实际包含了什么头文件，然后解释一下这个问题。

// 头文件名 zlib.h 实际上是 zlib 库本身的标准头文件名，这是一个约定俗成的事实标准，不是在 Cargo 或 build.rs 里设置的。让我查看一下相关文件来给用户更准确的回答。

// 明白了！你在 `z_user.c:3` 里写了 `#include "zlib.h"`。这是个很好的问题，让我从几个层面来解释：

// ---

// ## 头文件名 `zlib.h` 的来源

// ### 1. **是 zlib 库本身的固定命名（事实标准）**

// `zlib.h` 不是在 Cargo 或 build.rs 里"设置"的，而是 **zlib C 库自诞生以来就叫这个名字**，已经有近 30 年历史了。

// 这就好比：
// - C 标准库的头文件永远叫 `stdio.h`、`stdlib.h`
// - OpenSSL 的头文件永远叫 `openssl/ssl.h`
// - zlib 的头文件就叫 `zlib.h`

// 这是 **库作者定的名字**，是业界公认的事实标准。

// ### 2. **怎么验证？**

// 既然 `DEP_Z_INCLUDE` 指向 `/usr/include`，我们可以直接看看那个目录里有什么：

// ```bash
// ls -la /usr/include/zlib.h
// ```

// 这个文件是 zlib 库安装时放到系统里的，名字就是 `zlib.h`。

// ### 3. **那 `DEP_Z_INCLUDE` 到底起什么作用？**

// `DEP_Z_INCLUDE` 不是告诉程序"头文件叫什么名字"，而是告诉程序 **"头文件在哪个目录里"**：

// ```
// 你写的代码：       #include "zlib.h"       ← 名字固定死了，写在代码里
                                           
// 编译时需要：       头文件搜索路径           ← DEP_Z_INCLUDE 提供这个
//                    /usr/include/
//                     └── zlib.h             ← C 编译器在这个目录下找到它
// ```

// **对比一下：**

// | 东西 | 谁定的 | 哪里体现 |
// |---|---|---|
// | 头文件**名字**叫 `zlib.h` | zlib 库作者 | 你在 C 代码里硬编码写死 `#include "zlib.h"` |
// | 头文件**在哪**（路径） | 系统/构建环境 | `DEP_Z_INCLUDE=/usr/include` 由 `libz-sys` 动态提供 |

// ### 4. **为什么不能把路径也硬编码？**

// 因为不同系统上 zlib 安装的位置可能不同：

// | 系统 | zlib 头文件可能的位置 |
// |---|---|
// | Ubuntu/Debian | `/usr/include` |
// | CentOS/Fedora | `/usr/include` |
// | macOS (Intel) | `/usr/include` |
// | macOS (Apple Silicon) | `/opt/homebrew/include` |
// | Windows (vcpkg) | `C:\vcpkg\installed\x64-windows\include` |
// | 本地从源码编译 | 任意 `out/` 目录 |

// `libz-sys` 的 build.rs 会帮你**探测**当前系统上 zlib 实际装在哪，然后通过 `cargo:include=...`（即 `DEP_Z_INCLUDE`）告诉你正确的路径，这样你的代码就可以跨平台移植了。

// ---

// ## 总结

// - **`zlib.h` 这个名字**：是 zlib C 库的标准头文件名，**写死在你的 C 代码里**（`z_user.c:3`），属于行业惯例/库的约定。
// - **`DEP_Z_INCLUDE` 的值**：是头文件所在的**目录路径**，由 `libz-sys` 的 build.rs 动态探测后传递过来，解决跨平台路径不一致的问题。

// 所以你需要这两样东西配合：**你知道名字 + libz-sys 告诉你位置** = 编译成功 ✅