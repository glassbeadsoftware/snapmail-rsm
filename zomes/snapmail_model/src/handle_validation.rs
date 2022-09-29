use hdi::prelude::*;

// use tracing::*;

use crate::handle::Handle;


/// Validates the handle's name field
fn check_name(name: String) -> ExternResult<ValidateCallbackResult> {
    // TODO: Do check with a regex
    // Check: min & max character count
    debug!("*** check_name: {} ({})", name, name.len());
    if name.len() < 2 {
        return Ok(ValidateCallbackResult::Invalid("Name too short".into()));
    }
    if name.len() > 32 {
        return Ok(ValidateCallbackResult::Invalid("Name too long".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}

///
pub fn validate_handle_entry(handle: Handle) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate_handle_entry() called!");
    return check_name(handle.name);
}

// #[hdk_extern]
// fn validate_handle_delete(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
//     Ok(ValidateCallbackResult::Invalid("Agent must always have a Handle".into()))
// }

// #[hdk_extern]
// fn validate_handle_create(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
//     // FIXME: Check if author has already created a handle
//     let record = input.record;
//     let entry = record.into_inner().1;
//     let entry = match entry {
//         ElementEntry::Present(e) => e,
//         _ => return Ok(ValidateCallbackResult::Invalid("Entry not present".into())),
//     };
//     let handle: Handle = entry.try_into()?;
//     return check_name(handle.name);
// }

// #[hdk_extern]
// fn validate_handle_update(package: ValidateData) -> ExternResult<ValidateCallbackResult> {
//     //EntryValidationData::Modify{new_entry: new_handle, old_entry: old_handle, old_entry_action:_, validation_data: _};
//     if new_handle.name == old_handle.name {
//         return Ok(ValidateCallbackResult::Invalid("Trying to modify with same data".into()));
//     }
//     return validate_name(new_handle.name);
// }
