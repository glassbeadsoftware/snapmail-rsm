#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;

mod validate_create_link;
mod validate_delete_link;
mod validate;
mod utils;

mod constants;
mod link_kind;
mod entry_kind;
mod path_kind;

mod dm;
mod dm_protocol;

mod playground;
mod chunk;
mod signal_protocol;

mod handle;
mod mail;

// mod file;

use hdk3::prelude::*;
use chunk::*;
use handle::*;
use mail::entries::*;

pub use playground::*;
pub use dm::*;
pub use dm_protocol::*;
pub use utils::*;
pub use constants::*;
pub use link_kind::*;
pub use entry_kind::*;
pub use path_kind::*;
pub use signal_protocol::*;

holochain_externs!();

/// Careful of order
entry_defs![
   /// --  Handle
   Handle::entry_def(),
   /// -- Mail
   InMail::entry_def(),
   OutMail::entry_def(),
   OutAck::entry_def(),
   InAck::entry_def(),
   PendingAck::entry_def(),
   PendingMail::entry_def(),
   /// -- Other
   Path::entry_def(),
   Post::entry_def(),
   FileChunk::entry_def()
];


/// Get EntryType out of a EntryDef
pub fn def_to_type(entry_name: &str) -> EntryType {
    /// Sadly hardcoded since index is based on vec above.
    let entry_index = match entry_name {
        entry_kind::Handle => 0,
        entry_kind::InMail => 1,
        entry_kind::OutMail => 2,
        entry_kind::OutAck => 3,
        entry_kind::InAck => 4,
        entry_kind::PendingAck => 5,
        entry_kind::PendingMail => 6,
        _  => unreachable!(),
    };
    let app_type = AppEntryType::new(
        EntryDefIndex::from(entry_index),
        ZomeId::from(0), // since we have only one zome in our DNA (thank god)
        EntryVisibility::Public, // Everything Public for now...
    );
    EntryType::App(app_type)
}

// -- Wrapped Common types -- //

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeBool(bool);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeString(String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeRaw(Vec<u8>);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeHeaderHashVec(Vec<HeaderHash>);

// -- Callbacks -- //

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    debug!("*** init() called!").ok();
    /// Set Global Anchors
    Path::from(path_kind::Directory).ensure()?;
    /// Set access for receive/send
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, "receive_dm".into()));
    create_cap_grant(
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
