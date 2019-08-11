#!/bin/bash

# Build for host platform.
cargo build --release 
strip  target/release/iba-?
strip  target/release/insane-british-anagram

# Build for WASM on the web
cargo build --target  wasm32-unknown-unknown --features "web" --release

# Bind for use in web page
wasm-bindgen --target web     target/wasm32-unknown-unknown/release/insane-british-anagram.wasm --out-dir ./www


# Build for WASM and node.js
cargo build --target wasm32-unknown-unknown  --release


# Bind for use from node.js
wasm-bindgen --target nodejs  target/wasm32-unknown-unknown/release/insane-british-anagram.wasm --out-dir ./nodejs




