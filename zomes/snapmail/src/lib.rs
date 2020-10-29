#[macro_use] extern crate shrinkwraprs;


mod validate_create_link;
mod validate_delete_link;
mod validate;
mod utils;
mod constants;
mod link_kind;
mod entry_kind;

mod playground;
mod chunk;
mod handle;

/*
mod file;
mod mail;
mod protocol;
mod signal_protocol;

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

// -- Send & Receive Hack -- //

#[hdk_extern]
pub fn receive(/*from: Address, */ dm: DirectMessageProtocol) -> ExternalResult<ZomeBool> {
    //mail::receive(from, JsonString::from_json(&msg_json))
    match dm {
        DirectMessageProtocol::Ping => Ok(true),
        _ => Ok(false),
    }
}


// -- Wrapped Common types -- //

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeBool(bool);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeString(String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeRaw(Vec<u8>);


// -- Callbacks -- //

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    debug!("*** init() called!").ok();
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn validation_package(input: AppEntryType) -> ExternResult<ValidationPackageCallbackResult> {
    debug!("*** validation_package() called!").ok();
    let dummy = ValidationPackage(vec![]);
    Ok(ValidationPackageCallbackResult::Success(dummy))
}

#[hdk_extern]
fn validate_agent(_: Element) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate_agent() called!").ok();
    Ok(ValidateCallbackResult::Valid)
}
