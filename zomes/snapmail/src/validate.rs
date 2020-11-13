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
    let result = match entry {
        Entry::Agent(agent_hash) => validate_agent_entry(agent_hash, maybe_package),
        Entry::CapClaim(claim) => validate_claim_entry(claim, maybe_package),
        Entry::CapGrant(grant) => validate_grant_entry(grant, maybe_package),
        Entry::App(entry_bytes) => validate_app_entry(entry_bytes, maybe_package),
    };
    debug!(format!("*** validate() called ; result = {:?}", result)).ok();
    result
}


///
fn validate_app_entry(
    base_entry_bytes: AppEntryBytes,
    maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_app_entry() called!").ok();
    let sb = base_entry_bytes.into_sb();

    /// Call validate Path entry
    let maybe_path = Path::try_from(sb.clone());
    if maybe_path.is_ok() {
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }
    /// Try validate Handle entry
    let maybe_handle = Handle::try_from(sb.clone());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_handle_entry(handle, maybe_validation_package);
    }

    /// Try validate Chunk entry
    let maybe_chunk = FileChunk::try_from(sb.clone());
    if maybe_chunk.is_ok() {
        return Ok(ValidateCallbackResult::Valid);
    }

    // let maybe_inmail = InMail::try_from(sb.clone());
    // if maybe_inmail.is_ok() {
    //     let inmail = maybe_inmail.unwrap();
    //     return validate_inmail_entry(inmail, maybe_validation_package);
    // }
    /// Add validate entry per type here...
    /// Done
    // FIXME: should default to invalid
    //Ok(ValidateCallbackResult::Invalid("Not authorized".into()))
    Ok(ValidateCallbackResult::Valid)
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