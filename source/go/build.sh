#!/bin/sh

SRC_DIR=$1 # e.g. sort
SRC_FILE=$2 # e.g. main.go

# build
cd $SRC_DIR
mkdir -p wasm-build
ts=$(date +%s%N)
tinygo build -o wasm-build/wasm-eval-src.wasm -target wasi $SRC_FILE
echo Compile time: $((($(date +%s%N) - $ts)/1000000)) ms