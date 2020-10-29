#[macro_use] extern crate shrinkwraprs;

mod chunk;

mod handle;
mod playground;
mod validate_create_link;
mod validate_delete_link;
mod validate;

/*
mod file;
mod mail;
mod utils;
mod protocol;
mod signal_protocol;
mod globals;
mod link_kind;
mod entry_kind;
*/


use hdk3::prelude::*;
//use test_wasm_common::*;
use chunk::*;
use handle::*;

/*
pub use signal_protocol::*;
pub use protocol::*;
pub use utils::*;
pub use globals::*;
pub use entry_kind::*;

use mail::entries::*;
*/


holochain_externs!();

entry_defs![Post::entry_def(), FileChunk::entry_def(), Handle::entry_def()];

// -- Callbacks -- //

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}





#[hdk_extern]
fn validation_package(input: AppEntryType) -> ExternResult<ValidationPackageCallbackResult> {
    let wtf = ValidationPackage(vec![]);
    Ok(ValidationPackageCallbackResult::Success(wtf))
}

// #[hdk_extern]
// fn validate_agent(_: Element) -> ExternResult<ValidateCallbackResult> {
//     Ok(ValidateCallbackResult::Valid)
// }
