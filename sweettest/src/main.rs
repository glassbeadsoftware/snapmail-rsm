#![allow(unused_doc_comments)]

#[cfg(not(target_arch = "wasm32"))]
pub mod test;
#[cfg(not(target_arch = "wasm32"))]
pub mod test_mail;
#[cfg(not(target_arch = "wasm32"))]
pub mod setup;

pub const DNA_FILEPATH: &str = "./workdir/snapmail-dna.dna";

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "multi_thread")]
async fn main() {
   let args: Vec<String> = std::env::args().collect();
   //println!("{:?}", args);
   let arg= if args.len() > 1 {
      args[1].clone()
   } else {
      String::new()
   };
   crate::test::test(arg).await;
}

/// Dummy main for wasm32 target
#[cfg(target_arch = "wasm32")]
fn main() { }
