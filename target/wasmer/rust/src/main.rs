use std::time;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_wasi::WasiState;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut udf = "sum-cpp";

    if args.len() > 1 {
        udf = &args[1];
    } 

    // load wasm module
    let sum_cpp_wasm_path = "../../../../source/cpp/1-basic/wasm-eval-src/wasm-build/wasm-eval-src.wasm";    // cpp source
    let sum_go_wasm_path = "../../../src/go/wasm-eval-src/wasm-build/wasm-eval-src.wasm";     // go source
    let sum_cs_wasm_path = "../../../src/csharp/wasm-eval-src/bin/Debug/net7.0/wasm-eval-src.wasm";       // c# source
    let sum_rust_wasm_path = "../../../src/rust/wasm-eval-src/target/wasm32-wasi/release/wasm-eval-src.wasm";   // rust source
    let sort_cpp_wasm_path = "../../../../source/cpp/3-sort/wasm-build/wasm-eval-src.wasm";
    let sort_go_wasm_path = "../../../../source/go/2-sort/wasm-build/wasm-eval-src.wasm";
    let sort_java_wasm_path = "../../../../source/java/sort/target/wasm/sort.wasm";
    let sudoku_cpp_wasm_path = "../../../../source/cpp/4-sudoku/wasm-build/4-sudoku.wasm";
    let sudoku_go_wasm_path = "../../../../source/go/3-sudoku/wasm-build/wasm-eval-src.wasm";
    let sudoku_java_wasm_path = "../../../../source/java/sudoku/target/wasm/sudoku.wasm";

    let wasm_path = match udf {
        "sum-cpp" => sum_cpp_wasm_path,
        "sum-go" => sum_go_wasm_path,
        "sum-cs" => sum_cs_wasm_path,
        "sum-rust" => sum_rust_wasm_path,
        "sort-cpp" => sort_cpp_wasm_path,
        "sort-go" => sort_go_wasm_path,
        "sort-java" => sort_java_wasm_path,
        "sudoku-cpp" => sudoku_cpp_wasm_path,
        "sudoku-go" => sudoku_go_wasm_path,
        "sudoku-java" => sudoku_java_wasm_path,
        _ => sum_cpp_wasm_path,
    };

    let t0 = time::Instant::now();

    let wasm_bytes = std::fs::read(wasm_path)?;

    let t1 = time::Instant::now();

    // compile wasm module
    let mut store = Store::new(Cranelift::default());
    let module = Module::new(&store, wasm_bytes)?;

    let t2 = time::Instant::now();

    let wasi_env = match udf {
        "sum-cpp" | "sum-go" | "sum-cs" | "sum-rust" => {
            wasmer_wasi::WasiState::new("wasm-eval-tgt")
                .args(&["2", "3"])
                .finalize(&mut store)?
        },
        "sort-cpp" | "sort-go" | "sort-java" => {
            wasmer_wasi::WasiState::new("wasm-eval-tgt")
                .args(&["3", "10"])
                .finalize(&mut store)?
        },
        "sudoku-cpp" | "sudoku-go" | "sudoku-java" => {
            wasmer_wasi::WasiState::new("wasm-eval-tgt")
                .args(&["300600000500007000870090400080050000064000790000020030001040078000300002000005004"])
                .finalize(&mut store)?
        },
        _ => {
            wasmer_wasi::WasiState::new("wasm-eval-tgt")
                .finalize(&mut store)?
        }
    };

    let import_object = wasi_env.import_object(&mut store, &module)?;
    let instance = Instance::new(&mut store, &module, &import_object)?;
    let memory = instance.exports.get_memory("memory")?;
    wasi_env.data_mut(&mut store).set_memory(memory.clone());
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    let t3 = time::Instant::now();

    let total_time = t3 - t0;
    let load_time = t1 - t0;
    let compile_time = t2 - t1;
    let function_time_first = t3 - t2;
    

    let mut exec_sub: f64 = 0.0;
    let num_it = 10;
    for _n in 0..num_it {
        let t4 = time::Instant::now();

        let wasi_env = match udf {
            "sum-cpp" | "sum-go" | "sum-cs" | "sum-rust" => {
                wasmer_wasi::WasiState::new("wasm-eval-tgt")
                    .args(&["2", "3"])
                    .finalize(&mut store)?
            },
            "sort-cpp" | "sort-go" | "sort-java" => {
                wasmer_wasi::WasiState::new("wasm-eval-tgt")
                    .args(&["3", "10"])
                    .finalize(&mut store)?
            },
            "sudoku-cpp" | "sudoku-go" | "sudoku-java" => {
                wasmer_wasi::WasiState::new("wasm-eval-tgt")
                    .args(&["300600000500007000870090400080050000064000790000020030001040078000300002000005004"])
                    .finalize(&mut store)?
            },
            _ => {
                wasmer_wasi::WasiState::new("wasm-eval-tgt")
                    .finalize(&mut store)?
            }
        };

        let import_object = wasi_env.import_object(&mut store, &module)?;
        let instance = Instance::new(&mut store, &module, &import_object)?;
        let memory = instance.exports.get_memory("memory")?;
        wasi_env.data_mut(&mut store).set_memory(memory.clone());
        let start = instance.exports.get_function("_start")?;
        start.call(&mut store, &[])?;

        let t5 = time::Instant::now();
        exec_sub = exec_sub + (t5 - t4).as_micros() as f64;
        
    }

    // println!("Total time: {:.3}", total_time.as_micros() as f64 / 1000.0);
    // println!("Load time: {:.3}", load_time.as_micros() as f64 / 1000.0);
    // println!("Compile time: {:.3}", compile_time.as_micros() as f64 / 1000.0);
    // println!("Function time first: {:.3}", function_time_first.as_micros() as f64 / 1000.0);
    // println!("Function time subsequent: {:.3}", function_time_subsequent.as_micros() as f64 / 1000.0);

    println!("{:.3},{:.3},{:.3},{:.3},{:.3}", 
        load_time.as_micros() as f64 / 1000.0, 
        compile_time.as_micros() as f64 / 1000.0, 
        function_time_first.as_micros() as f64 / 1000.0, 
        total_time.as_micros() as f64 / 1000.0, 
        exec_sub / (num_it as f64 * 1000.0));

    Ok(())
}
