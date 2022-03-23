REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Compile the exe
cargo build --package wasm_utils
REM Compute hash of zome
.\target\debug\hash_zome  > zome_hash.txt
REM Show hash
.\target\debug\hash_zome
