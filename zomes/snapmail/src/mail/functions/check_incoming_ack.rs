use hdk3::prelude::*;

use crate::{
    mail,
    utils::*,
    ZomeEhVec,
    mail::entries::PendingAck,

};

/// Zome Function
/// Check for PendingAcks and convert to InAcks
/// Return list of OutMail EntryHashes for which we succesfully linked a new InAck out of PendingAcks
#[hdk_extern]
pub fn check_incoming_ack(_:()) -> ExternResult<ZomeEhVec> {
    /// Lookup `ack_inbox` links on my agentId
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let links_result = get_links(
        my_agent_eh.clone(),
        // FIXME: should be LinkKind::AckInbox.as_tag_opt(),
        None,
    )?.into_inner();
    debug!("incoming_ack links_result: {:?} (for {})", links_result, &my_agent_eh);
    /// Check each link
    let mut updated_outmails = Vec::new();
    for link in &links_result {
        let pending_ack_eh = link.target.clone();
        let maybe_latest = get_latest_entry_from_eh::<PendingAck>(pending_ack_eh.clone())?;
        if maybe_latest.is_none() {
            debug!("Header not found for pending mail entry");
            continue;
        }
        let (_pending_ack, pending_ack_hh, _) = maybe_latest.unwrap();
        debug!("pending_ack_hh: {}", pending_ack_hh);
        /// Get entry on the DHT
        let maybe_pending_ack = get_typed_and_author::<PendingAck>(&pending_ack_eh);
        if let Err(err) = maybe_pending_ack {
            debug!("Getting PendingAck from DHT failed: {}", err);
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        /// Create InAck
        let maybe_inack_hh = mail::commit_inack(pending_ack.outmail_eh.clone(), &author);
        if let Err(err) = maybe_inack_hh {
            debug!("Creating InAck from PendignAck failed: {}", err);
            continue;
        }
        /// Delete link from this agent address
        let res = delete_link(link.create_link_hash.clone());
        if let Err(err) = res {
            debug!("Remove ``ack_inbox`` link failed:");
            debug!(err);
            continue;
        }
        /// Delete PendingAck
        let res = delete_entry(pending_ack_hh);
        if let Err(err) = res {
            debug!("Delete PendignAck failed: {}", err);
        }
        /// Add to return list
        updated_outmails.push(pending_ack.outmail_eh.clone());
    }
    debug!("incoming_ack updated_outmails.len() = {} (for {})", updated_outmails.len(), &my_agent_eh);
    Ok(ZomeEhVec(updated_outmails))
}