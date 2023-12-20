//
// Created by Yifei Yang on 4/3/23.
//

#include <stdio.h>
#include <string>
#include <sstream>
#include <inttypes.h>

#define ROW_CNT 10000
#define COL_CNT 3

int32_t data[ROW_CNT][COL_CNT];
int32_t** projected;
int32_t** filtered;

/* Note that alignment must be a power of two. */
void *allocate_aligned(size_t size, size_t alignment)
{
  const size_t mask = alignment - 1;
  const uintptr_t mem = (uintptr_t) malloc(size + alignment);
  return (void *) ((mem + mask) & ~mask);
}

__attribute__((export_name("get_data")))
int* get_data() {
//  size_t alloc_size = sizeof(int32_t) * ROW_CNT * COL_CNT;
//  data = (int32_t*) allocate_aligned(alloc_size, ROW_CNT*COL_CNT);
  return &data[0][0];
}

// void print(int32_t** data_print, int64_t row_cnt, int col_cnt, const int print_row_head_cnt) {
//   std::stringstream ss;
//   for (int i = 0; i < col_cnt; ++i) {
//     ss << "[COL " << i << "]\t";
//     if (row_cnt <= print_row_head_cnt * 2) {
//       for (int j = 0; j < row_cnt; ++j) {
//         ss << data_print[i][j] << " ";
//       }
//     } else {
//       for (int j = 0; j < print_row_head_cnt; ++j) {
//         ss << data_print[i][j] << " ";
//       }
//       ss << "... ";
//       for (int j = row_cnt - print_row_head_cnt; j < row_cnt; ++j) {
//         ss << data_print[i][j] << " ";
//       }
//     }
//     ss << "\n";
//   }
//   printf("%s", ss.str().c_str());
// }

// __attribute__((export_name("print_data")))
// void print_data(const int print_row_head_cnt) {
//  int32_t** data_print = new int32_t*[COL_CNT];
//  for (int c = 0; c < COL_CNT; ++c) {
//    data_print[c] = data[c];
//  }
//  print(data_print, ROW_CNT, COL_CNT, print_row_head_cnt);
// }

// col1 + col2, col2 * col3
__attribute__((export_name("project")))
void project() {
 const int num_proj = 2;
 projected = new int32_t*[num_proj];
 for (int c = 0; c < num_proj; ++c) {
   projected[c] = new int32_t[ROW_CNT];
 }

 // compute
 for (int64_t r = 0; r < ROW_CNT; ++r) {
   projected[0][r] = data[0][r] + data[1][r];
   projected[1][r] = data[1][r] * data[2][r];
 }

 // show the result
//  print(projected, ROW_CNT, num_proj, 10);

 for (int c = 0; c < num_proj; ++c) {
   delete[] projected[c];
 }
 delete[] projected;
}

__attribute__((export_name("less_than")))
void less_than(const int col, const int32_t val) {
 int32_t* col_data = data[col];
 filtered = new int32_t*[COL_CNT];
 for (int c = 0; c < COL_CNT; ++c) {
   filtered[c] = new int32_t[ROW_CNT];
 }

 // compute
 int64_t res_cnt = 0;
 for (int64_t r = 0; r < ROW_CNT; ++r) {
   if (col_data[r] < val) {
     for (int c = 0; c < COL_CNT; ++c) {
       filtered[c][res_cnt] = data[c][r];
     }
     ++res_cnt;
   }
 }

 // show the result
//  printf("Num rows: %" PRIu64 "\n", res_cnt);
//  print(filtered, res_cnt, COL_CNT, 10);

 for (int c = 0; c < COL_CNT; ++c) {
   delete[] filtered[c];
 }
 delete[] filtered;
}

int main() {
  printf("rel_op main\n");
  return 0;
}
