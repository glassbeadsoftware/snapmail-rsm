use hdk3::prelude::*;

#[hdk_extern]
fn validate_create_link(create_link_submission: ValidateCreateLinkData) -> ExternResult<ValidateLinkCallbackResult> {
    let base_entry = create_link_submission.base;
    let target_entry = create_link_submission.target;

    // Determine where to dispatch according to base
    match base_entry {
        Entry::Agent(agent_hash) => validate_create_link_from_agent(agent_hash, target_entry),
        Entry::CapClaim(claim) => validate_create_link_from_claim(claim, target_entry),
        Entry::CapGrant(grant) => validate_create_link_from_grant(grant, target_entry),
        Entry::App(entry_bytes) => validate_create_link_from_app(entry_bytes, target_entry),
    }
}

///
fn validate_create_link_from_app(base_entry_bytes: AppEntryBytes, target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    let maybe_handle = Handle::try_from(base_entry_bytes.into_sb());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_create_link_from_handle(handle, target_entry);
    }
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_create_link_from_agent(_agent_hash: HoloHash<Agent>, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME: Can only set handle for self
    // FIXME: Only one handle per agent
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_create_link_from_claim(_claim: CapClaim, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_create_link_from_grant(_grant: ZomeCallCapGrant, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Invalid)
}