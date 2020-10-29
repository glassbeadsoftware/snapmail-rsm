use hdk3::prelude::*;

#[hdk_extern]
fn validate_delete_link(_delete_link_submission: ValidateDeleteLinkData) -> ExternResult<ValidateLinkCallbackResult> {
    let _delete_link = validate_delete_link.delete_link;

    Ok(ValidateCallbackResult::Valid)
}
