REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Pack DNAs
hc dna pack workdir -o workdir/snapmail.dna
REM Pack the Happ with everything
hc app pack workdir -o workdir/snapmail.happ
