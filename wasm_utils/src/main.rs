#![allow(unused_doc_comments)]

pub const WASM_PATH: &str = "./target/wasm32-unknown-unknown/release/snapmail.wasm";
pub const OUTPUT: &str = "dna_hash.txt";

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
   /// Load DnaFile
   //println!("Loading DNA wasm file: {}", WASM_PATH);
   let wasm = &std::fs::read(WASM_PATH)?;
   let dna_wasm = holochain_types::dna::wasm::DnaWasm::from(wasm.to_vec());
   let (_, wasm_hash) = holochain_types::dna::wasm::DnaWasmHashed::from_content(dna_wasm.clone())
      .await
      .into_inner();
   print!("{}", wasm_hash);
   Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() { }