#![allow(unused_doc_comments)]

#[cfg(not(target_arch = "wasm32"))]
use dna_wasm::{DnaWasm, DnaWasmHashed};

pub const ZOME_WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/snapmail.wasm";
pub const OUTPUT: &str = "zome_hash.txt";

/// Output the hash of the Snapmail WASM
/// WARN - Only print the hash to stdout!
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let mut args = std::env::args();
   args.next(); // skip exe
   let zome_wasm_path = if let Some(arg) = args.next() {
      arg
   } else {
      ZOME_WASM_PATH.to_string()
   };
   /// Load Wasm file
   let zome_wasm = &std::fs::read(zome_wasm_path)?;
   /// Create vanilla Dna out of zome
   let dna_wasm = DnaWasm::from(zome_wasm.to_vec());
   let (_, zome_hash) = DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   /// print hash
   print!("{}", zome_hash);
   Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() { }
