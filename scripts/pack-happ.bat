REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Pack DNAs
hc dna pack snapmail.dna.workdir
REM Pack the Happ with everything
hc app pack snapmail.dna.workdir
