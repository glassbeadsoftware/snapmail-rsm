#!/bin/bash

# Compile the WASM
cargo build --release --target wasm32-unknown-unknown
# Pack DNAs
hc dna pack snapmail.dna.workdir
# Pack the Happ with everything
hc app pack snapmail.dna.workdir
