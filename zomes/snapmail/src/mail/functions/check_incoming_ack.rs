use hdk3::prelude::*;

// use hdk::{
//     error::{ExternResult, ZomeApiError},
//     holochain_persistence_api::{
//         cas::content::Address
//     },
// };
// use holochain_wasm_utils::{
//     holochain_core_types::link::LinkMatch,
// };

use crate::mail;
use crate::link_kind;

/// Return list of outMail addresses for which we succesfully linked a new InAck out of PendingAcks
pub fn check_incoming_ack() -> ExternResult<Vec<HeaderHash>> {
    let maybe_my_handle_address = crate::handle::get_my_handle_entry();
    if let None = maybe_my_handle_address {
        return Err(ZomeApiError::Internal("This agent does not have a Handle set up".to_string()));
    }
    let my_handle_address = maybe_my_handle_address.unwrap().0;
    // Lookup `ack_inbox` links on my agentId
    let links_result = get_links!(
        &my_handle_address,
        link_tag(link_kind::AckInbox),
    )?;
    debug!(format!("incoming_ack links_result: {:?} (for {})", links_result, &my_handle_address)).ok();
    // For each link
    let mut updated_outmails = Vec::new();
    for pending_ack_address in &links_result.addresses() {
        //  - Get entry on the DHT
        let maybe_pending_ack = mail::get_pending_ack(pending_ack_address);
        if let Err(err) = maybe_pending_ack {
            debug!(format!("Getting PendingAck from DHT failed: {}", err)).ok();
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        // Create InAck
        let maybe_inack_address = mail::create_and_commit_inack(&pending_ack.outmail_address, &author);
        if let Err(err) = maybe_inack_address {
            debug!(format!("Creating InAck from PendignAck failed: {}", err)).ok();
            continue;
        }
        //  - Delete link from my agentId
        let res = remove_link!(
            &my_handle_address,
            &pending_ack_address,
            link_tag(link_kind::AckInbox),
        );
        if let Err(err) = res {
            debug!("Remove ``ack_inbox`` link failed:").ok();
            debug!(err).ok();
            continue;
        }
        // Delete PendingAck
        let res = hdk::remove_entry(pending_ack_address);
        if let Err(err) = res {
            debug!(format!("Delete PendignAck failed: {}", err)).ok();
        }
        // Add to return list
        updated_outmails.push(pending_ack.outmail_address.clone());
    }
    Ok(updated_outmails)
}