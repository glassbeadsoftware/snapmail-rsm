REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Compute hash of dna
.\target\debug\hash_wasm  > dna_hash.txt
