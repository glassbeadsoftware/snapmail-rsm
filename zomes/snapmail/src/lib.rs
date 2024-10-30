
extern crate strum;
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;

// FIXME update to latest hdk
#[macro_use] extern crate snapmail_proc_macro;

#[cfg(not(target_arch = "wasm32"))]
pub mod api_error;

mod dm;
mod dm_protocol;
pub mod signal_protocol;
mod callbacks;
pub mod handle;
pub mod mail;
pub mod file;
pub mod create_entry;

pub use snapmail_model::*;
pub use dm::*;
pub use dm_protocol::*;
pub use signal_protocol::*;
pub use create_entry::*;


//--------------------------------------------------------------------------------------------------

use hdk::prelude::*;

#[hdk_extern]
fn get_zome_info(_:()) -> ExternResult<ZomeInfo> {
  return zome_info();
}


#[hdk_extern]
fn get_dna_info(_:()) -> ExternResult<DnaInfo> {
  return dna_info();
}


#[hdk_extern]
fn get_record_author(dh: AnyDhtHash) -> ExternResult<AgentPubKey> {
  return zome_utils::get_author(dh);
}