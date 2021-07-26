REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Compile the exe
cargo build --package wasm_utils
REM Compute hash of dna
.\target\debug\hash_wasm  > dna_hash.txt
