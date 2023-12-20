
const ROW_CNT: usize = 10000;
const COL_CNT: usize = 3;
static mut A : [[i32; ROW_CNT]; COL_CNT] = [[0; ROW_CNT]; COL_CNT];

#[no_mangle]
unsafe fn get_data() -> *const i32 {
    A[0].as_ptr()
}

#[no_mangle]
unsafe fn print_data(print_row_head_cnt: usize) {
    for c in 0..COL_CNT {
        if ROW_CNT <= print_row_head_cnt * 2 {
            println!("[COL {}]\t{:?}", c, &A[c]);
        } else {
            println!("[COL {}]\t{:?} ... {:?}", c, &A[c][.. print_row_head_cnt],
                     &A[c][ROW_CNT - print_row_head_cnt ..]);
        }
    }
}

fn print(data_print: &mut Vec<Vec<i32>>, row_cnt: usize, col_cnt: usize, print_row_head_cnt: usize) {
    for c in 0..col_cnt {
        if row_cnt <= print_row_head_cnt * 2 {
            println!("[COL {}]\t{:?}", c, &data_print[c]);
        } else {
            println!("[COL {}]\t{:?} ... {:?}", c, &data_print[c][.. print_row_head_cnt],
                     &data_print[c][row_cnt - print_row_head_cnt .. row_cnt]);
        }
    }
}

#[no_mangle]
unsafe fn project() {
    let num_proj = 2;
    let mut projected = vec![vec![0; ROW_CNT]; num_proj];

    // compute
    for r in 0..ROW_CNT {
        projected[0][r] = A[0][r] + A[1][r];
        projected[1][r] = A[1][r] * A[2][r];
    }

    // show the result
    // print(&mut projected, ROW_CNT, num_proj, 10);
}

#[no_mangle]
unsafe fn less_than(col: i32, val: i32) {
    let mut filtered = vec![vec![0; ROW_CNT]; COL_CNT];
    let col_data = &A[col as usize];

    // compute
    let mut res_cnt: usize = 0;
    for r in 0..ROW_CNT {
        if col_data[r] < val {
            for c in 0..COL_CNT {
                filtered[c][res_cnt] = A[c][r]
            }
            res_cnt += 1;
        }
    }

    // show the result
    // println!("Num rows: {}", res_cnt);
    // print(&mut filtered, res_cnt, COL_CNT, 10);
}

fn main() {
    println!("Hello, world!");
}
