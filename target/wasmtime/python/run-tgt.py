#!/usr/bin/env python3
from wasmtime import Engine, Linker, Module, Store, WasiConfig

import sys
import datetime

SUM_CPP_UDF = "sum-cpp"
SORT_CPP_UDF = "sort-cpp"
SUDK_CPP_UDF = "sudk-cpp"
SUM_RUST_UDF = "sum-rust"
SUM_GO_UDF= "sum-go"
SORT_GO_UDF = "sort-go"
SUDK_GO_UDF = "sudk-go"
SUM_JAVA_UDF = "sum-java"
SORT_JAVA_UDF = "sort-java"
SUDK_JAVA_UDF = "sudk-java"
PHP_UDF = "php"

args = sys.argv[1:]
if len(args) < 1:
    print(f"Usage: python3 2-run-lang.py {SUM_CPP_UDF}|{SORT_CPP_UDF}|{SUDK_CPP_UDF}|{SUM_RUST_UDF}|{SUM_GO_UDF}|{PHP_UDF}\n")
    exit(1)

src_lang = args[0]

sum_cpp_wasm_path = '../../../source/cpp/1-basic/wasm-eval-src/wasm-build/wasm-eval-src.wasm'
sort_cpp_wasm_path = '../../../source/cpp/3-sort/wasm-build/wasm-eval-src.wasm'
sudk_cpp_wasm_path = '../../../source/cpp/4-sudoku/wasm-build/4-sudoku.wasm'
sum_rust_wasm_path = '../../../source/rust/1-sum/target/wasm32-wasi/release/wasm-eval-src.wasm'
sum_go_wasm_path = '../../../source/go/1-sum/wasm-eval-src/wasm-build/wasm-eval-src.wasm'
sort_go_wasm_path = '../../../source/go/2-sort/wasm-build/wasm-eval-src.wasm'
sudk_go_wasm_path = '../../../source/go/3-sudoku/wasm-build/wasm-eval-src.wasm'
sum_java_wasm_path = '../../../source/java/sum/target/wasm/sum.wasm'
sort_java_wasm_path = '../../../source/java/sort/target/wasm/sort.wasm'
sudk_java_wasm_path = '../../../source/java/sudoku/target/wasm/sudoku.wasm'
php_wasm_path = '../../../source/php/php-cgi-8.2.6.wasm'
php_source_path = '../../../source/php/1-hello-world.php'

if src_lang == SUM_CPP_UDF:
    wasm_path = sum_cpp_wasm_path
elif src_lang == SORT_CPP_UDF:
    wasm_path = sort_cpp_wasm_path
elif src_lang == SUDK_CPP_UDF:
    wasm_path = sudk_cpp_wasm_path
elif src_lang == SUM_RUST_UDF:
    wasm_path = sum_rust_wasm_path
elif src_lang == SUM_GO_UDF:
    wasm_path = sum_go_wasm_path
elif src_lang == SORT_GO_UDF:
    wasm_path = sort_go_wasm_path
elif src_lang == SUDK_GO_UDF:
    wasm_path = sudk_go_wasm_path
elif src_lang == SUM_JAVA_UDF:
    wasm_path = sum_java_wasm_path
elif src_lang == SORT_JAVA_UDF:
    wasm_path = sort_java_wasm_path
elif src_lang == SUDK_JAVA_UDF:
    wasm_path = sudk_java_wasm_path
elif src_lang == PHP_UDF:
    wasm_path = php_wasm_path
else:
    print("Error: src lang not found")
    exit(1)

t0 = datetime.datetime.now()

wasm_bytes = open(wasm_path, 'rb').read()
t1 = datetime.datetime.now()

engine = Engine()
module = Module(engine, wasm_bytes)
t2 = datetime.datetime.now()

config = WasiConfig()
if src_lang == SUM_CPP_UDF:
    config.argv = ('sum', '4', '11')
elif src_lang == SORT_CPP_UDF:
    config.argv = ('sort_bench', '3', '10')
elif src_lang == SUDK_CPP_UDF:
    config.argv = ('process', '300600000500007000870090400080050000064000790000020030001040078000300002000005004')
elif src_lang == SUM_RUST_UDF:
    config.argv = ('sum', '6', '11')
elif src_lang == SUM_GO_UDF:
    config.argv = ('main', '8', '11')
elif src_lang == SORT_GO_UDF:
    config.argv = ('main', '3', '10')
elif src_lang == SUDK_GO_UDF:
    config.argv = ('main', '300600000500007000870090400080050000064000790000020030001040078000300002000005004')
elif src_lang == SUM_JAVA_UDF:
    config.argv = ('sum', '4', '11')
elif src_lang == SORT_JAVA_UDF:
    config.argv = ('sort', '3', '10')
elif src_lang == SUDK_JAVA_UDF:
    config.argv = ('main', '300600000500007000870090400080050000064000790000020030001040078000300002000005004')
elif src_lang == PHP_UDF:
    config.argv = ('add', php_source_path)
else:
    pass

config.preopen_dir(".", "/")
config.inherit_stdout()
linker = Linker(engine)
linker.define_wasi()
store = Store(linker.engine)
store.set_wasi(config)
instance = linker.instantiate(store, module)

start = instance.exports(store)["_start"]
start(store)
t3 = datetime.datetime.now()

num_it = 10
t_sub = 0
for i in range(num_it):
    t4 = datetime.datetime.now()
    
    # Got error on C++ src, if not re-instantiated
    instance = linker.instantiate(store, module)
    start = instance.exports(store)["_start"]
    
    start(store)
    t5 = datetime.datetime.now()
    t_sub += (t5 - t4).total_seconds() * 1000

load_time = (t1 - t0).total_seconds() * 1000
compile_time = (t2 - t1).total_seconds() * 1000
func_exec_time = (t3 - t2).total_seconds() * 1000
total_time = (t3 - t0).total_seconds() * 1000

if False:
    print("load time:", load_time, "ms") 
    print("mod compile time:", compile_time, "ms")
    print("func exec time:", func_exec_time, "ms")
    print("total time:", total_time, "ms")
    print("Function time subsequent (avg on {} calls): {:.3f} ms".format(num_it, t_sub / num_it))

print("{:.3f}".format(load_time), ",", "{:.3f}".format(compile_time), ",", "{:.3f}".format(func_exec_time), \
        ",", "{:.3f}".format(total_time), ",", "{:.3f}".format(t_sub / num_it), sep="")
