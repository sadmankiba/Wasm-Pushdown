# Wasm Pushdown

Wasm (WebAssembly) is a platform-independent portable bytecode form that various programming languages can compile to. Wasm bytecode can be executed in a lightweight, secure, and isolated way that makes it a great alternative for Docker and micro-VMs. Many cloud databases allow running User-Defined Functions (UDFs) along with SQL queries in database servers to extend functionality and better performance. 

This project evaluates the performance of UDFs written in various source languages, exported as Wasm bytecodes and executed in different target languages. We benchmarked with standard UDFs from [`LangBench`](https://github.com/dsrg-uoft/LangBench) and two relational operations- projection, and filtering. 

Our findings 
- Target languages and runtimes do not have much effect on performance of UDF execution.
- UDFs originally written in higher-level language like Java takes more time to execute in Wasm compared to lower-level language such as C++ and Go.
- Programs with memory-heavy operations face higher penalty when executing as Wasm.

## Directory Structure

```
/
|- source : Contains UDFs and scripts for Wasm compilation in C++, Go, Java and Rust languages
|- target : Contains programs to execute UDFs from Wasm in target languages (e.g. Python, Go) with runtimes `Wasmer` and `Wasmtime`.
|- presentation.pdf : Project presentation
|- report.pdf : Project report
```

## Contributors

* [Suhas Shripad Hebbar](https://github.com/SuhasHebbar)
* [Sadman Sakib](https://github.com/sadmankiba)
* [Varun Kaundinya](https://github.com/kvxrun)
* [Ashutosh Parida](https://github.com/ashu-holmes)