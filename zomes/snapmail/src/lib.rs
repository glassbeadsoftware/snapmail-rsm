#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]

#[macro_use] extern crate shrinkwraprs;

mod validate_create_link;
mod validate_delete_link;
mod validate;
mod utils;

mod constants;
mod link_kind;
mod entry_kind;
mod path_kind;

mod protocol;

mod playground;
mod chunk;
mod handle;

/*
mod file;
mod mail;
mod signal_protocol;

*/


use hdk3::prelude::*;
//use hdk3::map_extern::ExternResult;
use chunk::*;
use handle::*;

pub use playground::*;
pub use protocol::*;
pub use utils::*;
pub use constants::*;
pub use entry_kind::*;
pub use path_kind::*;

/*
pub use signal_protocol::*;
use mail::entries::*;
*/


holochain_externs!();

//entry_defs![Post::entry_def()];
entry_defs![
   Path::entry_def(),
   Post::entry_def(),
   FileChunk::entry_def(),
   Handle::entry_def()
];

// -- Send & Receive Hack -- //

#[hdk_extern]
pub fn receive(/*from: Address, */ dm: DirectMessageProtocol) -> ExternResult<ZomeBool> {
    debug!("*** receive() called!").ok();
    //mail::receive(from, JsonString::from_json(&msg_json))
    match dm {
        DirectMessageProtocol::Ping => Ok(ZomeBool(true.into())),
        _ => Ok(ZomeBool(false.into())),
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
    /// Set Global Anchors
    Path::from(path_kind::Directory).ensure()?;
    /// Set access for receive/send
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info!()?.zome_name, "receive".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "".into(),
            // empty access converts to unrestricted
            access: ().into(),
            functions,
        }
    )?;
    /// Done
    Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
fn validation_package(_input: AppEntryType) -> ExternResult<ValidationPackageCallbackResult> {
    debug!("*** validation_package() called!").ok();
    let dummy = ValidationPackage(vec![]);
    Ok(ValidationPackageCallbackResult::Success(dummy))
}

#[hdk_extern]
fn validate_agent(_: Element) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate_agent() called!").ok();
    Ok(ValidateCallbackResult::Valid)
}
