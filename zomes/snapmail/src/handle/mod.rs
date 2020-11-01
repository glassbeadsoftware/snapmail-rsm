mod functions;
mod utils;
mod validation;



use hdk3::prelude::*;

pub use functions::*;
pub use validation::*;


// use hdk::{
//     entry_definition::ValidatingEntryType,
// };

use crate::{
    entry_kind, link_kind,
};

/// Entry representing the username of an Agent
#[hdk_entry(id = "Handle", visibility = "public")]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Handle {
    pub name: String,
}

/*
//pub fn handle_def() -> ValidatingEntryType {
entry_def!(Handle EntryDef {
    id: entry_kind::Handle,
    crdt_type: CrdtType,
    //description: "Entry for an Agent's public username",
    required_validations: RequiredValidations::default(),
    visibility: EntryVisibility::Public,
});
*/

// impl From<EntryAndHash<Handle>> for Handle {
//     fn from(entry_and_hash: EntryAndHash<Handle>) -> Self {
//         entry_and_hash.0
//     }
// }

//entry_defs!(vec![Handle::entry_def()]);


impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

