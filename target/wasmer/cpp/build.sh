#!/bin/sh

# env
source /home/sadman/.wasienv/wasienv.sh

# build
cd wasm-tgt
rm -rf wasm-build
mkdir -p wasm-build
cd wasm-build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build . --target wasm-tgt