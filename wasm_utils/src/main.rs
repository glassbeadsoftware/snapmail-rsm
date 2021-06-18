#![allow(unused_doc_comments)]

use holochain_types::dna::wasm::DnaWasm;

pub const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/snapmail.wasm";

pub const OUTPUT: &str = "dna_hash.txt";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   /// Load DnaFile
   //println!("Loading DNA wasm file: {}", WASM_PATH);
   let wasm = &std::fs::read(WASM_PATH)?;
   let dna_wasm = DnaWasm::from(wasm.to_vec());
   let (_, wasm_hash) = holochain_types::dna::wasm::DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   print!("{}", wasm_hash);
   Ok(())
}
