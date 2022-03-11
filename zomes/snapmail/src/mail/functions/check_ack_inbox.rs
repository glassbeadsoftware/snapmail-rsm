use hdk::prelude::*;
use zome_utils::*;

use crate::{
    mail,
    mail::entries::PendingAck,
    link_kind::*,
};

/// Zome Function
/// Check for PendingAcks and convert to InAcks
/// Return list of OutMail EntryHashes for which we succesfully linked a new InAck out of PendingAcks
#[hdk_extern]
#[snapmail_api]
pub fn check_ack_inbox(_:()) -> ExternResult<Vec<EntryHash>> {
    /// Lookup `ack_inbox` links on my agentId
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let links_result = get_links(
        my_agent_eh.clone(),
        LinkKind::AckInbox.as_tag_opt(),
        //None,
    )?;
    debug!("incoming_ack links_result: {:?} (for {})", links_result, &my_agent_eh);
    /// Check each link
    let mut updated_outmails = Vec::new();
    for link in &links_result {
        /// Get entry on the DHT
        let pending_ack_eh = link.target.clone();
        let maybe_el = get(pending_ack_eh.clone(), GetOptions::latest())?;
        if maybe_el.is_none() {
            warn!("Header not found for pending ack entry");
            continue;
        }
        let pending_ack_hh = maybe_el.unwrap().header_address().clone();
        debug!("pending_ack_hh: {}", pending_ack_hh);
        let maybe_pending_ack = get_typed_and_author::<PendingAck>(&pending_ack_eh);
        if let Err(err) = maybe_pending_ack {
            warn!("Getting PendingAck from DHT failed: {}", err);
            continue;
        }
        let (author, pending_ack) = maybe_pending_ack.unwrap();
        /// Check signature
        let maybe_verified = verify_signature(author.clone(), pending_ack.from_signature.clone(), pending_ack.outmail_eh.clone());
        match maybe_verified {
            Err(err) => {
                let response_str = "Verifying PendingAck failed";
                error!("{}: {}", response_str, err);
                continue;
            }
            Ok(false) => {
                error!("Failed verifying PendingAck signature");
                continue;
            }
            Ok(true) => debug!("Valid PendingAck signature"),
        }
        /// Create InAck
        let maybe_inack_hh = mail::create_inack(pending_ack.outmail_eh.clone(), &author, pending_ack.from_signature);
        if let Err(err) = maybe_inack_hh {
            error!("Creating InAck from PendignAck failed: {}", err);
            continue;
        }
        /// Delete link from this agent address
        let res = delete_link(link.create_link_hash.clone());
        if let Err(err) = res {
            error!("Remove ``ack_inbox`` link failed:");
            error!(?err);
            continue;
        }
        // /// Delete PendingAck
        // let res = delete_entry(pending_ack_hh.clone());
        // if let Err(err) = res {
        //     error!("Delete PendignAck failed: {}", err);
        // }
        /// Add to return list
        updated_outmails.push(pending_ack.outmail_eh.clone());
    }
    debug!("incoming_ack updated_outmails.len() = {} (for {})", updated_outmails.len(), &my_agent_eh);
    Ok(updated_outmails)
}
