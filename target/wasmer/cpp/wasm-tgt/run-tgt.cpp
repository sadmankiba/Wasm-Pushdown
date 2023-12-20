#include <string>
#include <chrono>
#include <cstdio>
#include "wasmer.h"

#define BUF_SIZE 128

int main () {
    /* load wasm module */
    std::string cpp_wasm_path = "../../source/cpp/1-basic/wasm-eval-src/wasm-build/wasm-eval-src.wasm";

    auto t0 = std::chrono::steady_clock::now();

    FILE* file = fopen(cpp_wasm_path.c_str(), "rb");
    if (!file) {
        printf("> Error opening wasm bytecode.\n");
        return 1;
    }
    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    fseek(file, 0L, SEEK_SET);

    wasm_byte_vec_t wasm_bytes;
    wasm_byte_vec_new_uninitialized(&wasm_bytes, file_size);
    if (fread(wasm_bytes.data, file_size, 1, file) != 1) {
        printf("> Error reading bytecode.\n");
        return 1;
    } 
    fclose(file);

    auto t1 = std::chrono::steady_clock::now();

    /* compile wasm module */
    wasm_engine_t* engine = wasm_engine_new();
    wasm_store_t* store = wasm_store_new(engine);
    wasm_module_t* module = wasm_module_new(store, &wasm_bytes);
    if (!module) {
        printf("> Error building module.\n");
        return 1;
    }

    auto t2 = std::chrono::steady_clock::now();

    /* prepare configuration 
    
    - set args
    - get imports
    - initialize instance
    - set exports
    */
    wasi_config_t* config = wasi_config_new("wasm-eval-tgt");
    wasi_config_arg(config, "4");
    wasi_config_arg(config, "9");
    wasi_config_capture_stdout(config);
    wasi_env_t* wasi_env = wasi_env_new(store, config);

    wasm_extern_vec_t imports;
    bool get_imports_result = wasi_get_imports(store, wasi_env, module, &imports);
    if (!get_imports_result) {
        printf("> Error getting imports.\n");
        return 1;
    }

    wasm_instance_t* instance = wasm_instance_new(store, module, &imports, NULL);
    if (!instance) {
        printf("> Error building instance.\n");
        return 1;
    }
    if (!wasi_env_initialize_instance(wasi_env, store, instance)) {
        printf("> Error initializing instance in wasi env\n");
        return 1;
    }

    wasm_extern_vec_t exports;
    wasm_instance_exports(instance, &exports);
    if (exports.size == 0) {
        printf("> Error accessing exports.\n");
        return 1;
    }

    /* run function */
    wasm_func_t *run_func = wasi_get_start_function(instance);
    if (!run_func) {
        printf("> Error getting start function.\n");
        return 1;
    }

    wasm_val_vec_t args = WASM_EMPTY_VEC;
    wasm_val_vec_t res = WASM_EMPTY_VEC;

    if (wasm_trap_t* trap = wasm_func_call(run_func, &args, &res)) {
        wasm_message_t msg;
        wasm_trap_message(trap, &msg);
        printf("> Error running function. %s\n", msg.data);
        return 1;
    }

    char buffer[BUF_SIZE] = {0};
    size_t data_read_size = wasi_env_read_stdout(wasi_env, buffer, BUF_SIZE);
    if (data_read_size == -1) {
        printf("> Error reading stdout.\n");
        return 1;
    }
    printf("> stdout: %s\n", buffer);

    auto t3 = std::chrono::steady_clock::now();

    /* measure stats */
    auto total_time = std::chrono::duration_cast<std::chrono::milliseconds>(t3 - t0).count();
    auto load_time = std::chrono::duration_cast<std::chrono::milliseconds>(t1 - t0).count();
    auto compile_time = std::chrono::duration_cast<std::chrono::milliseconds>(t2 - t1).count();
    auto func_run_time = std::chrono::duration_cast<std::chrono::milliseconds>(t3 - t2).count();

    printf("Total time: %ld ms\n", total_time);
    printf("Load time: %ld ms\n", load_time);
    printf("Compile time: %ld ms\n", compile_time);
    printf("Function run time: %ld ms\n", func_run_time);

    wasm_func_delete(run_func);
    wasi_env_delete(wasi_env);
    wasm_instance_delete(instance);
    wasm_extern_vec_delete(&exports);
    wasm_extern_vec_delete(&imports);

    wasm_byte_vec_delete(&wasm_bytes);
    wasm_module_delete(module);
    wasm_store_delete(store);
    wasm_engine_delete(engine);
}