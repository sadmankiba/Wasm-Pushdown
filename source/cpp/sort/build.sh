#!/bin/sh

# env
source ~/.wasienv/wasienv.sh

# build
cd wasm-eval-src
rm -rf wasm-build
mkdir -p wasm-build
cd wasm-build
wasimake cmake -DCMAKE_BUILD_TYPE=Release ..
ts=$(date +%s)
tsn=$(date +%N)
make
echo Compile time: $((($(date +%N) - $tsn) / 1000000 + ($(date +%s) - $ts) * 1000)) ms
