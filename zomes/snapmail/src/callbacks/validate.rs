use hdk::prelude::*;
use zome_utils::*;

use crate::{
    callbacks::validate_entry::validate_entry,
    callbacks::validate_link::validate_create_link,
};


/// Zome Callback
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    trace!("*** `validate()` callback called!");
    std::panic::set_hook(Box::new(zome_panic_hook));
    match op {
            Op::StoreElement { element } => {
            /// Check Header
            // N/A
            /// Check Entry
            let entry = element.clone().into_inner().1;
            let entry = match entry {
                ElementEntry::Present(e) => e,
                _ => return Ok(ValidateCallbackResult::Valid), // No entry in element so nothing to check.
            };
            let maybe_entry_type = element.header().entry_type();
            return validate_entry(entry, maybe_entry_type);
        },
        Op::StoreEntry { header, entry } => {
            let actual_header= header.hashed.into_inner().0;
            return validate_entry(entry, Some(actual_header.entry_type()));
        },
        Op::RegisterCreateLink { create_link } => {
            return validate_create_link(create_link);
        },
        Op::RegisterDeleteLink { .. } => {
            // TODO: Should not be valide by default
            // let _delete_link = validate_delete_link.delete_link;
            // Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
            Ok(ValidateCallbackResult::Valid)
        },
        Op::RegisterUpdate { .. } => {
            // TODO: Should not be valide by default
            Ok(ValidateCallbackResult::Valid)
            //Ok(ValidateCallbackResult::Invalid("updating entries isn't valid".to_string()))
        },
        Op::RegisterDelete { .. } => {
            // TODO: Should not be valide by default
            Ok(ValidateCallbackResult::Valid)
            //Ok(ValidateCallbackResult::Invalid("deleting entries isn't valid".to_string()))
        },

        Op::RegisterAgentActivity { .. } => {
            /// TODO: anti-spam? For now no limits or conditions for agent activity.
            Ok(ValidateCallbackResult::Valid)
        },
    }

}
