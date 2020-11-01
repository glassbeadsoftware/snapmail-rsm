use hdk3::prelude::*;
use hdk3::prelude::element::ElementEntry;
use crate::{
    handle::*, chunk::*,
};

#[hdk_extern]
fn validate(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate() called!").ok();

    let maybe_package = input.validation_package;
    let element = input.element;
    let entry = element.into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid), // Why not invalid?
    };

    // Determine where to dispatch according to base
    match entry {
        Entry::Agent(agent_hash) => validate_agent_entry(agent_hash, maybe_package),
        Entry::CapClaim(claim) => validate_claim_entry(claim, maybe_package),
        Entry::CapGrant(grant) => validate_grant_entry(grant, maybe_package),
        Entry::App(entry_bytes) => validate_app_entry(entry_bytes, maybe_package),
    }
}


///
fn validate_app_entry(
    base_entry_bytes: AppEntryBytes,
    maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_app_entry() called!").ok();
    // Call validate Handle entry
    let sb = base_entry_bytes.into_sb();
    let maybe_handle = Handle::try_from(sb.clone());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_handle_entry(handle, maybe_validation_package);
    }
    // Call validate Chunk entry
    let maybe_chunk = FileChunk::try_from(sb.clone());
    if maybe_chunk.is_ok() {
        return Ok(ValidateCallbackResult::Valid);
    }
    // Add validate entry per type here...
    // Done
    Ok(ValidateCallbackResult::Invalid("Not authorized".into()))
}

///
fn validate_agent_entry(
    _agent_hash: AgentPubKey,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_agent_entry() called!").ok();
    // FIXME
    Ok(ValidateCallbackResult::Valid)
}

///
fn validate_claim_entry(
    _claim: CapClaim,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_claim_entry() called!").ok();
    // FIXME
    Ok(ValidateCallbackResult::Invalid("Not authorized".into()))
}

///
fn validate_grant_entry(
    _grant: ZomeCallCapGrant,
    _maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_grant_entry() called!").ok();
    // FIXME
    Ok(ValidateCallbackResult::Invalid("Not authorized".into()))
}