use hdk3::prelude::*;

use crate::{
    mail,
    utils::*,
    link_kind::*,
    ZomeEhVec,
    mail::entries::PendingAck,

};

/// Zome Function
/// Check for PendingAcks and convert to InAcks
/// Return list of OutMail EntryHashes for which we succesfully linked a new InAck out of PendingAcks
#[hdk_extern]
pub fn check_incoming_ack(_:()) -> ExternResult<ZomeEhVec> {
    let maybe_element = crate::handle::get_my_handle_element();
    if let None = maybe_element {
        return error("This agent does not have a Handle set up");
    }
    let my_handle_element = maybe_element.unwrap();
    let my_handle_eh = get_eh(&my_handle_element)?;
    /// Lookup `ack_inbox` links on my agentId
    let links_result = get_links(
        my_handle_eh.clone(),
        LinkKind::AckInbox.as_tag_opt(),
    )?.into_inner();
    debug!("incoming_ack links_result: {:?} (for {})", links_result, &my_handle_eh).ok();
    /// Check each link
    let mut updated_outmails = Vec::new();
    for link in &links_result {
        let pending_ack_eh = link.target.clone();
        let maybe_latest = get_latest_for_entry::<PendingAck>(pending_ack_eh.clone())?;
        if maybe_latest.is_none() {
            debug!("Header not found for pending mail entry").ok();
            continue;
        }
        let (_pending_ack, pending_ack_hh, _) = maybe_latest.unwrap();
        debug!("pending ack address: {}", pending_ack_hh).ok();
        /// Get entry on the DHT
        let maybe_pending_ack = mail::get_pending_ack(&pending_ack_eh);
        if let Err(err) = maybe_pending_ack {
            debug!("Getting PendingAck from DHT failed: {}", err).ok();
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        /// Create InAck
        let maybe_inack_address = mail::create_and_commit_inack(pending_ack.outmail_eh.clone(), &author);
        if let Err(err) = maybe_inack_address {
            debug!("Creating InAck from PendignAck failed: {}", err).ok();
            continue;
        }
        /// Delete link from this agent address
        let res = delete_link(link.create_link_hash.clone());
        if let Err(err) = res {
            debug!("Remove ``ack_inbox`` link failed:").ok();
            debug!(err).ok();
            continue;
        }
        /// Delete PendingAck
        let res = delete_entry(pending_ack_hh);
        if let Err(err) = res {
            debug!("Delete PendignAck failed: {}", err).ok();
        }
        /// Add to return list
        updated_outmails.push(pending_ack.outmail_eh.clone());
    }
    Ok(ZomeEhVec(updated_outmails))
}