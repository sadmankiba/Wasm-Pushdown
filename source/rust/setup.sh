#!/bin/sh

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
source "$HOME/.cargo/env"
rustup target install wasm32-wasi
