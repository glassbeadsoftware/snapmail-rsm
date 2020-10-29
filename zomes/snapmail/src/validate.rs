use hdk3::prelude::*;

#[hdk_extern]
fn validate(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate() called!").ok();

    let maybe_validation_package = input.validation_package;
    let element = input.element;
    let entry = element.into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid), // Why not invalid?
    };

    // Determine where to dispatch according to base
    match entry {
        Entry::Agent(agent_hash) => validate_agent(agent_hash, maybe_validation_package),
        Entry::CapClaim(claim) => validate_claim(claim, maybe_validation_package),
        Entry::CapGrant(grant) => validate_grant(grant, maybe_validation_package),
        Entry::App(entry_bytes) => validate_app(entry_bytes, maybe_validation_package),
    }
}


///
fn validate_app(base_entry_bytes: AppEntryBytes, maybe_validation_package: Option<ValidationPackage>) -> ExternResult<ValidateLinkCallbackResult> {
    // Call validate Handle entry
    let maybe_handle = Handle::try_from(base_entry_bytes.into_sb());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_handle(handle, maybe_validation_package);
    }
    // Add validate entry per type here...
    // Done
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_agent(_agent_hash: HoloHash<Agent>, _maybe_validation_package: Option<ValidationPackage>) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_claim(_claim: CapClaim, _maybe_validation_package: Option<ValidationPackage>) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Invalid)
}

///
fn validate_grant(_grant: ZomeCallCapGrant, _target_entry: Entry) -> ExternResult<ValidateLinkCallbackResult> {
    // FIXME
    Ok(ValidateCallbackResult::Invalid)
}