package com.example.wasm;

import org.teavm.interop.Address;
import org.teavm.interop.Export;

public class Main {
    static final int ROW_CNT = 100000000;
    static final int COL_CNT = 3;

//     static int[][] data = new int[COL_CNT][ROW_CNT];
    static int[] data = new int[COL_CNT * ROW_CNT];
    static int[][] projected;
    static int[][] filtered;

    public static int index(int rowId, int colId) {
     return ROW_CNT * rowId + colId;
    }

    @Export(name = "get_data")
    public static Address getData() {
        return Address.ofData(data);
    }

//     public static void print_data(int print_row_head_cnt) {
//         for (int c = 0; c < COL_CNT; c++) {
//             if (ROW_CNT <= print_row_head_cnt * 2) {
//                 // directly printing 'data[c]' will be stuck when building with tinygo
//                 printArray(data[c], ROW_CNT);
//             } else {
//                 System.out.print("[COL " + c + "]\t");
//                 printArray(data[c], print_row_head_cnt);
//                 System.out.print(" ... ");
//                 printArray(data[c], ROW_CNT - print_row_head_cnt, ROW_CNT);
//                 System.out.println();
//             }
//         }
//     }

    public static void print_(int[][] data_print, int row_cnt, int col_cnt, int print_row_head_cnt) {
        for (int c = 0; c < col_cnt; c++) {
            if (row_cnt <= print_row_head_cnt * 2) {
                // directly printing 'data[c]' will be stuck when building with tinygo
                printArray(data_print[c], row_cnt);
            } else {
                System.out.print("[COL " + c + "]\t");
                printArray(data_print[c], print_row_head_cnt);
                System.out.print(" ... ");
                printArray(data_print[c], row_cnt - print_row_head_cnt, row_cnt);
                System.out.println();
            }
        }
    }

    @Export(name = "project")
    public static void project() {
        final int num_proj = 2;
        projected = new int[num_proj][ROW_CNT];

        // compute
        for (int r = 0; r < ROW_CNT; r++) {
            projected[0][r] = data[index(0, r)] + data[index(1, r)];
            projected[1][r] = data[index(1, r)] + data[index(2, r)];
        }

        // show the result
        print_(projected, ROW_CNT, num_proj, 10);
    }

    @Export(name = "less_than")
    public static void less_than(int col, int val) {
        filtered = new int[COL_CNT][ROW_CNT];

        // compute
        int res_cnt = 0;
        for (int r = 0; r < ROW_CNT; r++) {
            if (data[index(col, r)] < val) {
                for (int c = 0; c < COL_CNT; c++) {
                    filtered[c][res_cnt] = data[index(c,r)];
                }
                res_cnt++;
            }
        }

        // show the result
        System.out.println("Num rows: " + res_cnt);
        print_(filtered, res_cnt, COL_CNT, 10);
    }

    private static void printArray(int[] arr, int end) {
        for (int i = 0; i < end; i++) {
            System.out.print(arr[i] + " ");
        }
        System.out.println();
    }

    private static void printArray(int[] arr, int start, int end) {
        for (int i = start; i < end; i++) {
            System.out.print(arr[i] + " ");
        }
        System.out.println();
    }

    public static void main(String[] args) {
        System.out.println("rel_op main");
    }
}
