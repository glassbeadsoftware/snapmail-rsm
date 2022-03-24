#!/bin/sh

# Compile the WASM
cargo build --release --target wasm32-unknown-unknown
# Compile the exe
cargo build --package wasm_utils
# Compute hash of zome
value=`./target/debug/hash_zome`
echo "$value" > zome_hash.txt
echo
echo "ZOME HASH = $value"
