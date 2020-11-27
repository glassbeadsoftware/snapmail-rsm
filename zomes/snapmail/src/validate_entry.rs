use hdk3::prelude::*;
use hdk3::prelude::element::ElementEntry;

use crate::{
    handle::*,
    chunk::*,
    mail::entries::*,
    entry_kind::*,
    utils::*,
};

/// Zome Callback
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
    entry_type_id: EntryDefIndex,
    entry_bytes: AppEntryBytes,
    maybe_validation_package: Option<ValidationPackage>,
) -> ExternResult<ValidateCallbackResult>
{
    debug!("*** validate_app_entry() called!").ok();
    let sb = entry_bytes.into_sb();
    let entry_kind = EntryKind::from_index(&entry_type_id);

    match entry_kind {
        EntryKind::Handle => {
            let maybe_handle = Handle::try_from(sb.clone());
            if let Err(_err) = maybe_handle {
                return error("Failed to deserialize Handle");
            }
            let handle = maybe_handle.unwrap();
            return validate_handle_entry(handle, maybe_validation_package);
        },
        EntryKind::Path => {
            let maybe_content = Path::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize Path");
            }
            // FIXME
            Ok(ValidateCallbackResult::Valid)
        },
        EntryKind::InMail => {
            let maybe_content = InMail::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize InMail");
            }
            // FIXME
            // return validate_inmail_entry(inmail, maybe_validation_package);
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::InAck => {
            let maybe_content = InAck::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize InAck");
            }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::PendingMail => {
            let maybe_content = PendingMail::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize PendingMail");
            }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::PendingAck => {
            let maybe_content = PendingAck::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize PendingAck");
            }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::OutMail => {
            let maybe_content = OutMail::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize OutMail");
            }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::OutAck => {
            let maybe_content = OutAck::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize OutAck");
            }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },
        EntryKind::FileManifest => {
            // let maybe_content = FileManifest::try_from(sb.clone());
            // if let Err(err) = maybe_content {
            //     return error("Failed to deserialize FileManifest");
            // }
            // FIXME
            return Ok(ValidateCallbackResult::Valid);
        },

        /// DEBUG
        EntryKind::FileChunk => {
            let maybe_content = FileChunk::try_from(sb.clone());
            if let Err(_err) = maybe_content {
                return error("Failed to deserialize FileChunk");
            }
            return Ok(ValidateCallbackResult::Valid);
        },

        /// Add entry validation per type here
        /// ..

        /// Unreachable but doesnt compile without it. Yay Rust
        _ => Ok(ValidateCallbackResult::Invalid("Not authorized".into())),
    }
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