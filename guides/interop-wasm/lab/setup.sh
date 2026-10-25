#!/bin/sh
# Installs one prebuilt tool into /usr/local/bin. Needs network. The versions are pinned here.
set -eu
GH=https://github.com
case "$1" in
  wasm-bindgen)
    v=0.2.129
    curl -sSfL "$GH/wasm-bindgen/wasm-bindgen/releases/download/$v/wasm-bindgen-$v-aarch64-unknown-linux-musl.tar.gz" |
      tar xz -C /usr/local/bin --strip-components=1 "wasm-bindgen-$v-aarch64-unknown-linux-musl/wasm-bindgen"
    ;;
  wasmtime)
    v=v50.0.0
    curl -sSfL "$GH/bytecodealliance/wasmtime/releases/download/$v/wasmtime-$v-aarch64-linux.tar.xz" |
      tar xJ -C /usr/local/bin --strip-components=1 "wasmtime-$v-aarch64-linux/wasmtime"
    ;;
  wasm-opt)
    v=version_133
    curl -sSfL "$GH/WebAssembly/binaryen/releases/download/$v/binaryen-$v-aarch64-linux.tar.gz" |
      tar xz -C /usr/local --strip-components=1 "binaryen-$v/bin/wasm-opt" "binaryen-$v/lib"
    ;;
  wasm-pack)
    v=v0.15.0
    curl -sSfL "$GH/wasm-bindgen/wasm-pack/releases/download/$v/wasm-pack-$v-aarch64-unknown-linux-musl.tar.gz" |
      tar xz -C /usr/local/bin --strip-components=1 "wasm-pack-$v-aarch64-unknown-linux-musl/wasm-pack"
    ;;
  wasmer)
    v=v7.5.0
    curl -sSfL "$GH/wasmerio/wasmer/releases/download/$v/wasmer-linux-aarch64.tar.gz" |
      tar xz -C /usr/local/bin --strip-components=1 bin/wasmer
    ;;
  *) echo "unknown tool: $1" >&2; exit 1 ;;
esac
