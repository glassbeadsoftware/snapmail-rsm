use hdk3::prelude::*;

/*
use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
*/

use crate::{
    entry_kind, link_kind,
    protocol::{DirectMessageProtocol, AckMessage}, mail::{
    entries::{
        InMail, PendingAck, OutAck,
    }
}};

/// Zome function
/// Return address of newly created OutAck
#[hdk_extern]
pub fn acknowledge_mail(inmail_address: Address) -> ExternResult<EntryHash> {
    //  1. Make sure its an InMail
    let inmail = hdk::utils::get_as_type::<InMail>(inmail_address.clone())?;
    //  2. Make sure it has not already been acknowledged
    let res = hdk::get_links_count(&inmail_address, LinkMatch::Exactly(link_kind::Acknowledgment), LinkMatch::Any)?;
    if res.count > 0 {
        return Err(ZomeApiError::Internal("Mail has already been acknowledged".to_string()));
    }
    debug!("No Acknowledgment yet").ok();
    // 3. Write OutAck
    let outack = OutAck::new();
    let outack_entry = Entry::App(entry_kind::OutAck.into(), outack.into());
    let outack_address = commit_entry!(&outack_entry)?;
    let _ = create_link!(&inmail_address, &outack_address, link_kind::Acknowledgment, "")?;
    // 4. Try Direct sharing of Acknowledgment
    let res = acknowledge_mail_direct(&inmail.outmail_address, &inmail.from);
    if res.is_ok() {
        return Ok(outack_address);
    }
    let err = res.err().unwrap();
    debug!(format!("Direct sharing of Acknowledgment failed: {}", err)).ok();
    // 5. Otherwise share Acknowledgement via DHT
    let _ = acknowledge_mail_pending(&outack_address, &inmail.outmail_address, &inmail.from)?;
    Ok(outack_address)
}

/// Try sending directly to other Agent if Online
fn acknowledge_mail_direct(outmail_address: &Address, from: &AgentAddress) -> ExternResult<()> {
    /// a. Create DM
    let msg = AckMessage {
        outmail_address: outmail_address.clone(),
    };
    //let payload = serde_json::to_string(&DirectMessageProtocol::Ack(msg)).unwrap();
    let payload: SerializedBytes = DirectMessageProtocol::Ack(msg).try_into().unwrap();
    /// b. Send DM
    let result = call_remote!(
        from.clone(),
        zome_info!()?.zome_name,
        "receive".to_string().into(),
        payload,
    );
    if let Err(err) = result {
        return Err(err);
    }
    /// c. Check Response
    let response = result.unwrap();
    debug!(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(err) = maybe_msg {
        debug!(format!("Received response -> Err: {}", err)).ok();
        return Err(ZomeApiError::Internal(format!("{}", err)));
    }
    match maybe_msg.unwrap() {
        DirectMessageProtocol::Success(_) => Ok(()),
        _ => Err(ZomeApiError::Internal("Failed".to_string())),
    }
}

/// Create & Commit PendingAck
/// Return address of newly created PendingAck
/// Return PendingAck's address
fn acknowledge_mail_pending(outack_address: &Address, outmail_address: &Address, from: &AgentAddress) -> ExternResult<EntryHash> {
    // Get Handle address first
    let maybe_handle_address = crate::handle::get_handle_entry(from);
    if let None = maybe_handle_address {
        return Err(ZomeApiError::Internal("No handle has been set for ack receiving agent".to_string()));
    }
    let handle_address = maybe_handle_address.unwrap().0;
    // Commit PendingAck
    let pending_ack = PendingAck::new(outmail_address.clone());
    //let pending_ack_entry = Entry::App(entry_kind::PendingAck.into(), pending_ack.into());
    let pending_ack_address = commit_entry!(&pending_ack_entry)?;
    let _ = create_link!(&outack_address, &pending_ack_address, link_tag(link_kind::Pending))?;
    let _ = create_link!(&handle_address, &pending_ack_address, link_tag(link_kind::AckInbox + &*hdk::AGENT_ADDRESS.to_string()))?;
    debug!(format!("pending_ack_address: {:?} (for {} ; from: {})", pending_ack_address, handle_address, from)).ok();
    Ok(pending_ack_address)
}
