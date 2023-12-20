package main

import (
	"fmt"
)

const ROW_CNT int = 10000
const COL_CNT int = 3

var data [COL_CNT][ROW_CNT]int
var projected [][]int
var filtered [][]int

//export get_data
func get_data() *[COL_CNT][ROW_CNT]int {
	return &data
}

func print_(data_print [][]int, row_cnt int, col_cnt int, print_row_head_cnt int) {
	for c := 0; c < col_cnt; c++ {
		if row_cnt <= print_row_head_cnt*2 {
			// directly printing 'data[c]' will be stuck when building with tinygo
			fmt.Println(data_print[c][0:row_cnt])
		} else {
			fmt.Print("[COL ", c, "]\t", data_print[c][:print_row_head_cnt],
				" ... ", data_print[c][row_cnt-print_row_head_cnt:row_cnt], "\n")
		}
	}
}

//export print_data
func print_data(print_row_head_cnt int) {
	for c := 0; c < COL_CNT; c++ {
		if ROW_CNT <= print_row_head_cnt*2 {
			// directly printing 'data[c]' will be stuck when building with tinygo
			fmt.Println(data[c][0:ROW_CNT])
		} else {
			fmt.Print("[COL ", c, "]\t", data[c][:print_row_head_cnt],
				" ... ", data[c][ROW_CNT-print_row_head_cnt:], "\n")
		}
	}
}

//export project
func project() {
	const num_proj int = 2
	projected = make([][]int, num_proj)
	for c := 0; c < num_proj; c++ {
		projected[c] = make([]int, ROW_CNT)
	}

	// compute
	for r := 0; r < ROW_CNT; r++ {
		projected[0][r] = data[0][r] + data[1][r]
		projected[1][r] = data[1][r] * data[2][r]
	}

	// show the result
	// print_(projected, ROW_CNT, num_proj, 10)
}

//export less_than
func less_than(col int, val int) {
	col_data := data[col]
	filtered = make([][]int, COL_CNT)
	for c := 0; c < COL_CNT; c++ {
		filtered[c] = make([]int, ROW_CNT)
	}

	// compute
	res_cnt := 0
	for r := 0; r < ROW_CNT; r++ {
		if col_data[r] < val {
			for c := 0; c < COL_CNT; c++ {
				filtered[c][res_cnt] = data[c][r]
			}
			res_cnt++
		}
	}

	// show the result
	// fmt.Printf("Num rows: %d\n", res_cnt)
	// print_(filtered, res_cnt, COL_CNT, 10)
}

func main() {
	fmt.Printf("rel_op main\n")
}
