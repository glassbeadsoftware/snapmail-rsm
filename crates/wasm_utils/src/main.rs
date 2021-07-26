#![allow(unused_doc_comments)]

#[cfg(not(target_arch = "wasm32"))]
use dna_wasm::{DnaWasm, DnaWasmHashed};

pub const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/snapmail.wasm";
pub const OUTPUT: &str = "dna_hash.txt";

/// Output the hash of the Snapmail WASM
/// WARN - Only print the hash to stdout!
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let mut args = std::env::args();
   args.next(); // skip exe
   let wasm_path = if let Some(arg) = args.next() {
      arg
   } else {
      WASM_PATH.to_string()
   };
   /// Load DnaFile
   let wasm = &std::fs::read(wasm_path)?;
   let dna_wasm = DnaWasm::from(wasm.to_vec());
   let (_, wasm_hash) = DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   print!("{}", wasm_hash);
   Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() { }