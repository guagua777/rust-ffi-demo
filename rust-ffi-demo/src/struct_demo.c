#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

// 由于我们无法直接访问对象内部，因此它通常被称为不透明对象或不透明指针 。
// A dummy struct is created to provide a small amount of type-safety
typedef struct zip_code_database zip_code_database_t;

extern zip_code_database_t * zip_code_database_new(void);

extern void zip_code_database_free(zip_code_database_t *);

extern void zip_code_database_populate(zip_code_database_t *);

extern uint32_t zip_code_database_population_of(const zip_code_database_t *, const char *zip);

int main(void) {
  zip_code_database_t *database = zip_code_database_new();
  zip_code_database_populate(database);

  uint32_t pop1 = zip_code_database_population_of(database, "90210");
  uint32_t pop2 = zip_code_database_population_of(database, "20500");

  zip_code_database_free(database);

  printf("%" PRId32 "\n", (int32_t)pop1 - (int32_t)pop2);
}



// rust为原始代码，提供函数为c使用
// 即c调用rust的库