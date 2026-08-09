#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

// 声明一个函数
// 该函数为一个外部函数
extern uint32_t addition(uint32_t, uint32_t);

int main(void) {
  uint32_t sum = addition(1, 2);
  printf("%" PRIu32 "\n", sum);
}

// gcc --std=c11 -o c-example src/main.c -L target/debug/ -lintegers
// gcc --std=c11 -o c-example src/main.c -L target/debug/ -lintegers
// /usr/bin/ld: cannot find -lintegers: No such file or directory
// collect2: error: ld returned 1 exit status

// gcc --std=c11 -o c-example src/main.c -L target/debug/ -lrust_ffi_demo

// 如基础知识部分所述，在 macOS 和 Linux 上，可以使用 LD_LIBRARY_PATH=target/debug/ ./c-example 运行此程序；
// 在 Windows 上，可以通过将 target\debug\integers.dll 复制到当前目录并运行 .\c-example 来运行此程序。