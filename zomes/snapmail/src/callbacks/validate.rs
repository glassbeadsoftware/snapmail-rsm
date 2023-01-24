use hdk::prelude::*;
use zome_utils::*;
use snapmail_model::*;

use crate::{
    callbacks::validate_entry::validate_entry,
    callbacks::validate_link::validate_create_link,
};


/// Zome Callback
#[hdk_extern]
fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    std::panic::set_hook(Box::new(zome_panic_hook));
    debug!("*** validate() Op = {:?}", op);
    match op {
            Op::StoreElement { record } => {
            /// Check Action
            // N/A
            /// Check Entry
            let entry = record.clone().into_inner().1;
            let entry = match entry {
                ElementEntry::Present(e) => e,
                _ => return Ok(ValidateCallbackResult::Valid), // No entry in record so nothing to check.
            };
            let maybe_entry_type = record.action().entry_type();
            return validate_entry(entry, maybe_entry_type);
        },
        Op::StoreEntry { action, entry } => {
            let actual_action= action.hashed.into_inner().0;
            return validate_entry(entry, Some(actual_action.entry_type()));
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
