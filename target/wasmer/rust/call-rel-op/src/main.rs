//! Running a WASI compiled WebAssembly module with Wasmer.
//!
//! This example illustrates how to run WASI modules with
//! Wasmer.
//!
//! If you need more manual control over the instantiation, including custom
//! imports, then check out the ./wasi_manual_setup.rs example.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example wasi --release --features "cranelift,wasi"
//! ```
//!

use std::error::Error;
use std::fs::File;
use std::{mem, env};
use std::path::PathBuf;
use std::sync::Arc;
use std::{path::Path, time};

use std::io::Read;

use wasmer::{Module, Store, Value};
use wasmer_wasix::types::__WASI_STDIN_FILENO;
use wasmer_wasix::virtual_fs::{DeviceFile, FileSystem, PassthruFileSystem, RootFileSystemBuilder};
use wasmer_wasix::{default_fs_backing, Pipe, WasiEnv};
use arrow::{array::Array, record_batch::RecordBatch};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::errors::Result;

const BATCH_SIZE: usize = 100000;
const LOOP_CNT: usize = 2;

fn read_parquet() -> Result<Vec<RecordBatch>> {
    let file_path = "../../../../data/test.parquet";
    let file = File::open(file_path).unwrap();
    let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(file)?
        .with_batch_size(BATCH_SIZE)
        .build()?;
    let mut batches = Vec::new();
    for batch in parquet_reader {
        batches.push(batch?);
    }
    Ok(batches)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd_args: Vec<_> = env::args().skip(1).collect();
    if cmd_args.len() < 2 {
        println!("Error: need at least 2 params:\n\t1) src wasm: 'cpp'/'rust'/'go'\n\t2) -p (project) / -lt (less_than)");
        return Ok(())
    }
    let wasm_src = cmd_args[0].parse::<String>()?;
    let rel_op = cmd_args[1].parse::<String>()?;

    // load wasm module
    let cpp_wasm_path = "../../../../src/cpp/relational/rel_op.wasm";    // cpp source
    let rust_wasm_path = "../../../../src/rust/relational/target/wasm32-wasi/release/rel_op.wasm";     // rust source
    let go_wasm_path = "../../../../src/go/relational/wasm-build/rel_op.wasm";       // go source
    let java_wasm_path = "../../../../src/java/target/javascript/classes.wasm";
    let manual_wasm = (env::var("HOME")? + "/exec.wasm");
    let wasm_path: &str;
    if wasm_src == "cpp" {
        wasm_path = cpp_wasm_path;
    } else if wasm_src == "rust" {
        wasm_path = rust_wasm_path
    } else if wasm_src == "go" {
        wasm_path = go_wasm_path
    } else if wasm_src == "java" {
        wasm_path = java_wasm_path;
    } else if wasm_src == "manual" {
        wasm_path = manual_wasm.as_str();
    } else {
        println!("Error: unknown wasm src: '{}'", wasm_src);
        return Ok(())
    }

    let t0 = time::Instant::now();

    let wasm_bytes = std::fs::read(wasm_path)?;

    let t1 = time::Instant::now();
    // Create a Store.
    let mut store = Store::default();

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    let t2 = time::Instant::now();

    let wasi_env = WasiEnv::builder("hello");
        // .run_with_store(module.clone(), &mut store)?;


    let _guard = if tokio::runtime::Handle::try_current().is_err() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Some(runtime)
    } else {
        None
    };
    let _guard = _guard.as_ref().map(|r| r.enter());


    let (instance, env) = wasi_env.instantiate(module, &mut store)?;
    // stdout_rx.read_to_string(&mut buf).unwrap();
    let t3 = time::Instant::now();

    let total_time = t3 - t0;
    let load_time = t1 - t0;
    let compile_time = t2 - t1;
    let function_time_first = t3 - t2;
    println!("total time: {:.3}", total_time.as_micros() as f64 / 1000.0);
    println!("load time: {:.3}", load_time.as_micros() as f64 / 1000.0);
    println!(
        "compile time: {:.3}",
        compile_time.as_micros() as f64 / 1000.0
    );
    // println!(
    //     "Function time first: {:.3}",
    //     function_time_first.as_micros() as f64 / 1000.0
    // );

        // prepare data
        let get_data_func = instance.exports.get_function("get_data")?;
        let args = [];
        let result = get_data_func.call(&mut store, &args)?;
        let data_off = unsafe{(&(*result)[0]).as_raw(&mut store).i32};
    
        let batches = read_parquet()?;
    
        let mut num_rows: usize = 0;
        let num_cols = batches[0].num_columns();
        for batch in &batches {
            num_rows += batch.num_rows();
        }
        let mut curr_offs = vec![0; num_cols];
        for i in 0..num_cols {
            curr_offs[i] = i * num_rows * mem::size_of::<i32>();
        }

        let memory = instance.exports.get_memory("memory")?;
        let wasm_mem_whole = memory.view(&mut store).data_ptr();
        for batch in &batches {
            for i in 0..num_cols {
                // note: data here is in buffers[0]
                let column = batch.column(i);
                let data_ptr = column.data().buffers()[0].as_ptr();
                let copy_len = column.len() * mem::size_of::<i32>();
                unsafe {
                    let wasm_mem = wasm_mem_whole.offset(data_off as isize + curr_offs[i] as isize);
                    std::ptr::copy(data_ptr, wasm_mem, copy_len);
                }
                curr_offs[i] += copy_len;
            }
        }

        println!("Done initializing array");
    
        // project
        if rel_op == "-p" {
        let project_func = instance.exports.get_function("project")?;
        let args = [];
        for _ in 0..LOOP_CNT {
            let t0 = time::Instant::now();
            project_func.call(&mut store, &args)?;
            let t1 = time::Instant::now();
            println!("Func time (project): {:.3} ms\n", (t1 - t0).as_micros() as f64 / 1000.0);
        }

        }

        if rel_op == "-lt" {
            if cmd_args.len() < 4 {
                println!("Error: need the col id and the value of \"-lt\"");
                return Ok(())
            }
            let col = cmd_args[2].parse::<i32>()?;
            let val = cmd_args[3].parse::<i32>()?;
            let less_than_func = instance.exports.get_function("less_than")?;
            let args = [Value::I32(col), Value::I32(val)];
            for _ in 0..LOOP_CNT {
                let t0 = time::Instant::now();
                less_than_func.call(&mut store, &args)?;
                let t1 = time::Instant::now();
                println!("Func time (less_than): {:.3} ms\n", (t1 - t0).as_micros() as f64 / 1000.0);
            }
        }
    

    // eprintln!("Output: {buf}");

    // for _n in 0..3 {
    //     let t4 = time::Instant::now();

    //     let root_fs = get_rootfs()?;

    //     let wasi_env = WasiEnv::builder("hello")
    //         .args(&[input_file, input_file, "2", "3"])
    //         .sandbox_fs(root_fs)
    //         .preopen_dir("/")?
    //         .run_with_store(module.clone(), &mut store)?;

    //     let t5 = time::Instant::now();
    //     let function_time_subsequent = t5 - t4;
    //     println!("Function time subsequent: {:.3}", function_time_subsequent.as_micros() as f64 / 1000.0);
    // }

    Ok(())
}

fn get_rootfs() -> Result<wasmer_wasix::virtual_fs::TmpFileSystem, Box<dyn Error>> {
    let root_fs;
    {
        let _guard = if tokio::runtime::Handle::try_current().is_err() {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();
            Some(runtime)
        } else {
            None
        };
        let _guard = _guard.as_ref().map(|r| r.enter());

        root_fs = RootFileSystemBuilder::new()
            .with_tty(Box::new(DeviceFile::new(__WASI_STDIN_FILENO)))
            .build();

        let host_fs: Arc<dyn FileSystem + Send + Sync> = Arc::new(crate::default_fs_backing());

        root_fs.mount(
            PathBuf::from("/src"),
            &host_fs,
            PathBuf::from("./").canonicalize()?,
        )?;
    }
    Ok(root_fs)
}
