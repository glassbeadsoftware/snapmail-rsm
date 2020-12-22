#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use] extern crate shrinkwraprs;

mod validate_link;
mod validate_entry;
mod utils;
mod post_commit;

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

// mod file; FIXME

use hdk3::prelude::*;

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


/// -- Wrapped Common types -- ///

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeBool(bool);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeString(String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeRaw(Vec<u8>);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeHhVec(Vec<HeaderHash>);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeEhVec(Vec<EntryHash>);


/// -- Callbacks -- ///

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    debug!("*** init() callback called!");
    /// Set Global Anchors
    Path::from(path_kind::Directory).ensure()?;
    /// Set access for receive/send
    let mut functions: GrantedFunctions = HashSet::new();
    functions.insert((zome_info()?.zome_name, REMOTE_ENDPOINT.into()));
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
    debug!("*** validation_package() callback called!");
    let dummy = ValidationPackage(vec![]);
    Ok(ValidationPackageCallbackResult::Success(dummy))
}

#[hdk_extern]
fn validate_agent(_: Element) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate_agent() callback called!");
    Ok(ValidateCallbackResult::Valid)
}
