#!/bin/sh

pushd "$(dirname "$0")" > /dev/null

# build
cd relational
ts=$(date +%s%N)
cargo build --release --target=wasm32-wasi
echo Compile time: $((($(date +%s%N) - $ts)/1000000)) ms

popd > /dev/null
