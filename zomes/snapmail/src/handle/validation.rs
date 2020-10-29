use hdk3::prelude::*;
use crate::handle::Handle;

/// Validates the handle's name field
fn validate_name(name: String) -> ExternResult<ValidateCallbackResult> {
    // TODO: Do check with a regex
    // Check: min & max character count
    if name.len() < 2 {
        return Ok(ValidateCallbackResult::Invalid("Name too short".into()));
    }
    if name.len() > 32 {
        return Ok(ValidateCallbackResult::Invalid("Name too long".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}

///
fn validate_handle(handle: Handle, _maybe_validation_package: Option<ValidationPackage>) -> ExternResult<ValidateLinkCallbackResult> {
    return validate_name(handle.name);
}

#[hdk_extern]
fn validate_handle_create(package: ValidateData) -> ExternResult<ValidateCallbackResult> {
    // FIXME: Check if author has already created a handle
    let handle = package.element.try_into()?;
    return validate_name(handle.name);
}

#[hdk_extern]
fn validate_handle_delete(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("Agent must always have a Handle".into()))
}

// #[hdk_extern]
// fn validate_handle_update(package: ValidateData) -> ExternResult<ValidateCallbackResult> {
//     //EntryValidationData::Modify{new_entry: new_handle, old_entry: old_handle, old_entry_header:_, validation_data: _};
//     if new_handle.name == old_handle.name {
//         return Ok(ValidateCallbackResult::Invalid("Trying to modify with same data".into()));
//     }
//     return validate_name(new_handle.name);
// }


///
fn validate_create_link_from_handle(_handle: Handle, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Valid)
}

///
fn _validate_delete_link_from_handle(_handle: Handle, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Valid)
}