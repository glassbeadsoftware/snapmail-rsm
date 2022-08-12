#!/bin/bash

# Compile the WASM
cargo build --release --target wasm32-unknown-unknown
# Pack DNAs
hc dna pack workdir
# Pack the Happ with everything
hc app pack workdir
