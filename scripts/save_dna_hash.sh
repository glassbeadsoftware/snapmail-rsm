#!/bin/sh

# Compile the WASM
cargo build --release --target wasm32-unknown-unknown
# Compute hash of dna
value=`./target/debug/hash_wasm`
echo "$value" > dna_hash.txt
#echo "NEW DNA HASH = '$value'"
