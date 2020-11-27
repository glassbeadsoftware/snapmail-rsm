use hdk3::prelude::*;
use hdk3::prelude::element::ElementEntry;
use crate::{
    handle::*, chunk::*,
    mail::entries::*,
};

#[hdk_extern]
fn validate(input: ValidateData) -> ExternResult<ValidateCallbackResult> {
    debug!("*** validate() called!").ok();

    let maybe_package = input.validation_package;
    let element = input.element;
    let entry = element.clone().into_inner().1;
    let entry = match entry {
        ElementEntry::Present(e) => e,
        _ => return Ok(ValidateCallbackResult::Valid), // WARN - Why not invalid?
    };

    // Determine where to dispatch according to base
    let result = match entry {
        Entry::Agent(agent_hash) => validate_agent_entry(agent_hash, maybe_package),
        Entry::CapClaim(claim) => validate_claim_entry(claim, maybe_package),
        Entry::CapGrant(grant) => validate_grant_entry(grant, maybe_package),
        Entry::App(entry_bytes) => {
            let entry_type = element.header().entry_type().unwrap();
            let app_type_id = if let EntryType::App(app_entry_type) = entry_type {
                app_entry_type.id()
            } else { unreachable!()};
            validate_app_entry(app_type_id, entry_bytes, maybe_package)
        },
    };
    debug!(format!("*** validate() called ; result = {:?}", result)).ok();
    result
}

///
fn validate_app_entry(
    _entry_type_id: EntryDefIndex,
    entry_bytes: AppEntryBytes,
    maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_app_entry() called!").ok();
    let sb = entry_bytes.into_sb();

    /// Validate Path entry
    let maybe_path = Path::try_from(sb.clone());
    if maybe_path.is_ok() {
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate Chunk entry
    /// DEBUG
    let maybe_chunk = FileChunk::try_from(sb.clone());
    if maybe_chunk.is_ok() {
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate Handle entry
    let maybe_handle = Handle::try_from(sb.clone());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_handle_entry(handle, maybe_validation_package);
    }

    /// Validate InMail entry
    let maybe_app_entry = InMail::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _inmail = maybe_app_entry.unwrap();
        // FIXME
        // return validate_inmail_entry(inmail, maybe_validation_package);
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate InAck entry
    let maybe_app_entry = InAck::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _inack = maybe_app_entry.unwrap();
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate PendingMail entry
    let maybe_app_entry = PendingMail::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _pending_mail = maybe_app_entry.unwrap();
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate PendingAck entry
    let maybe_app_entry = PendingAck::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _pending_ack = maybe_app_entry.unwrap();
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate OutMail entry
    let maybe_app_entry = OutMail::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _outmail = maybe_app_entry.unwrap();
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    /// Validate OutAck entry
    let maybe_app_entry = OutAck::try_from(sb.clone());
    if maybe_app_entry.is_ok() {
        let _outack = maybe_app_entry.unwrap();
        // FIXME
        return Ok(ValidateCallbackResult::Valid);
    }

    // /// Validate FileChunk entry
    // let maybe_app_entry = FileChunk::try_from(sb.clone());
    // if maybe_app_entry.is_ok() {
    //     let _filechunk = maybe_app_entry.unwrap();
    //     // FIXME
    //     return Ok(ValidateCallbackResult::Valid);
    // }
    //
    // /// Validate FileChunk entry
    // let maybe_app_entry = FileManifest::try_from(sb.clone());
    // if maybe_app_entry.is_ok() {
    //     let _manifest = maybe_app_entry.unwrap();
    //     // FIXME
    //     return Ok(ValidateCallbackResult::Valid);
    // }

    /// Add entry validation per type here
    /// ..

    /// Done
    // FIXME: should default to invalid
    Ok(ValidateCallbackResult::Invalid("Not authorized".into()))
    //Ok(ValidateCallbackResult::Valid)
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