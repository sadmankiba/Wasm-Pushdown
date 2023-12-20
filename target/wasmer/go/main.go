package main

import (
	"fmt"
	"os"
	"github.com/wasmerio/wasmer-go/wasmer"
	"io/ioutil"
	"time"
)

func main() {
	src_udf := "sum-cpp"
	if len(os.Args) > 1  {
		src_udf = os.Args[1]
	}
	

	t0 := time.Now().UnixNano() / int64(time.Microsecond)

 	sum_cpp_wasmBytes, _ := ioutil.ReadFile("../../../source/cpp/1-basic/wasm-eval-src/wasm-build/wasm-eval-src.wasm") // cpp source
	sum_go_wasmBytes, _ := ioutil.ReadFile("../../../src/go/wasm-eval-src/wasm-build/wasm-eval-src.wasm") // go source
	sum_cs_wasmBytes, _ := ioutil.ReadFile("../../../src/csharp/wasm-eval-src/bin/Debug/net7.0/wasm-eval-src.wasm") // c# source
	sum_rust_wasmBytes, _ := ioutil.ReadFile("../../../src/rust/wasm-eval-src/target/wasm32-wasi/release/wasm-eval-src.wasm") // rust source
	sort_cpp_wasmBytes, _ := ioutil.ReadFile("../../../source/cpp/3-sort/wasm-build/wasm-eval-src.wasm") 
	sort_go_wasmBytes, _ := ioutil.ReadFile("../../../source/go/2-sort/wasm-build/wasm-eval-src.wasm") 
	sort_java_wasmBytes, _ := ioutil.ReadFile("../../../source/java/sort/target/wasm/sort.wasm")
	sudoku_cpp_wasmBytes, _ := ioutil.ReadFile("../../../source/cpp/4-sudoku/wasm-build/4-sudoku.wasm") 
	sudoku_go_wasmBytes, _ := ioutil.ReadFile("../../../source/go/3-sudoku/wasm-eval-src/wasm-build/wasm-eval-src.wasm") 
	sudoku_java_wasmBytes, _ := ioutil.ReadFile("../../../source/java/sudoku/target/wasm/sudoku.wasm") 

	wasmBytes := []byte{}
	switch src_udf {
		case "sum-cpp":
			wasmBytes = sum_cpp_wasmBytes
		case "sum-go":
			wasmBytes = sum_go_wasmBytes
		case "sum-cs":
			wasmBytes = sum_cs_wasmBytes
		case "sum-rust":
			wasmBytes = sum_rust_wasmBytes
		case "sort-cpp":
			wasmBytes = sort_cpp_wasmBytes
		case "sort-go":
			wasmBytes = sort_go_wasmBytes
		case "sort-java":
			wasmBytes = sort_java_wasmBytes
		case "sudoku-cpp":
			wasmBytes = sudoku_cpp_wasmBytes
		case "sudoku-go":
			wasmBytes = sudoku_go_wasmBytes
		case "sudoku-java":
			wasmBytes = sudoku_java_wasmBytes
		default:
			fmt.Println("Invalid source UDF")
			return
	}

	t1 := time.Now().UnixNano() / int64(time.Microsecond)

	store := wasmer.NewStore(wasmer.NewEngine())
	module, _ := wasmer.NewModule(store, wasmBytes)

	t2 := time.Now().UnixNano() / int64(time.Microsecond)

	wasiEnv, _ := wasmer.NewWasiStateBuilder("wasm-eval-tgt").Finalize()
	if src_udf == "sum-cpp" || src_udf == "sum-go" || src_udf == "sum-cs" || src_udf == "sum-rust" {
		wasiEnv, _ = wasmer.NewWasiStateBuilder("wasm-eval-tgt").
  		Argument("2").
  		Argument("3").
  		Finalize()
	} else if src_udf == "sort-cpp" || src_udf == "sort-go" || src_udf == "sort-java" {
		wasiEnv, _ = wasmer.NewWasiStateBuilder("wasm-eval-tgt").
	  		Argument("3").
			Argument("10").
			Finalize()
	} else if src_udf == "sudoku-cpp" || src_udf == "sudoku-go" || src_udf == "sudoku-java" {
		wasiEnv, _ = wasmer.NewWasiStateBuilder("wasm-eval-tgt").
	  		Argument("300600000500007000870090400080050000064000790000020030001040078000300002000005004").
			Finalize()
	}
  	importObject, err := wasiEnv.GenerateImportObject(store, module)
  	check(err)

  	instance, err := wasmer.NewInstance(module, importObject)
  	check(err)

	start, err := instance.Exports.GetWasiStartFunction()
	check(err)
	_, err = start()
	check(err)

	t3 := time.Now().UnixNano() / int64(time.Microsecond)

	
	load_time := float64(t1-t0)/1000.0
	compile_time := float64(t2-t1)/1000.0
	func_exec_time := float64(t3-t2)/1000.0
	total_time := float64(t3-t0)/1000.0

	// fmt.Printf("Load time: %.3f\n", load_time)
	// fmt.Printf("Compile time: %.3f\n", compile_time)
	// fmt.Printf("Function time first: %.3f\n", func_exec_time)
	// fmt.Printf("Total time: %.3f\n", total_time)

	// note: c# source currently cannot take input arguments
	exec_sub := float64(0)
	num_it := 10
	for i := 0; i < num_it; i++ {
		t4 := time.Now().UnixNano() / int64(time.Microsecond)

		instance, err := wasmer.NewInstance(module, importObject)
		check(err)

		start, err := instance.Exports.GetWasiStartFunction()
		check(err)
		_, err = start()
		check(err)

		t5 := time.Now().UnixNano() / int64(time.Microsecond)
		exec_sub += float64(t5-t4) / 1000.0
	}
	// fmt.Printf("Function time subsequent: %.3f\n", exec_sub / float64(num_it))

	fmt.Printf("%.3f,%.3f,%.3f,%.3f,%.3f\n", load_time, compile_time, func_exec_time, total_time, exec_sub / float64(num_it))
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
