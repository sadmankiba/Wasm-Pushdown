#!/bin/sh

SRC_DIR=$1

# env
source ~/.wasienv/wasienv.sh

# build
cd $SRC_DIR
rm -rf wasm-build
mkdir -p wasm-build
cd wasm-build
wasimake cmake -DCMAKE_BUILD_TYPE=Release ..
ts=$(date +%s)
tsn=$(date +%N)
make
# echo Compile time: $((($(date +%N) - $tsn) / 1000000 + ($(date +%s) - $ts) * 1000)) ms
echo Compile time: $((($(date +%s%N) - $ts$tsn) / 1000000)) ms
