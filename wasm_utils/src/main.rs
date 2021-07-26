#![allow(unused_doc_comments)]

#[cfg(not(target_arch = "wasm32"))]
pub mod wasm;

pub const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/snapmail.wasm";
pub const OUTPUT: &str = "dna_hash.txt";


/// Output the hash of the Snapmail WASM
/// WARN - Only print the hash to stdout!
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   /// Load DnaFile
   let wasm = &std::fs::read(WASM_PATH)?;
   let dna_wasm = crate::wasm::DnaWasm::from(wasm.to_vec());
   let (_, wasm_hash) = crate::wasm::DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   print!("{}", wasm_hash);
   Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() { }