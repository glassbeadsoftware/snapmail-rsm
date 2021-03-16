use hdk::prelude::*;
use hdk::prelude::metadata::EntryDhtStatus;

use crate::{
    link_kind::*,
    utils::*,
};

/// Zome function
/// Ack is considered received if there is no pendingAck link or PendingAck has delete status
#[hdk_extern]
pub fn has_ack_been_received(inmail_hh: HeaderHash) -> ExternResult<bool> {
    /// Get InMail (make sure InMail exists)
    let eh = hh_to_eh(inmail_hh.clone())?;
    /// Get OutAck
    let links_result = get_links(eh, LinkKind::Acknowledgment.as_tag_opt())?.into_inner();
    if links_result.len() < 1 {
        return error("No acknowledgment has been sent for this mail");
    }
    let outack_eh = links_result[0].target.clone();
    /// Get OutAck pending link
    let links_result = get_links(outack_eh, LinkKind::Pending.as_tag_opt())?.into_inner();
    /// If no link than return OK
    if links_result.is_empty() {
        return Ok(true);
    }
    /// Otherwise get PendingAck crud status
    let pending_eh = links_result[0].target.clone();
    let maybe_pending_history = get_details(pending_eh, GetOptions::content())?;
    if maybe_pending_history.is_none() {
        return error("No history found for PendingAck");
    }
    let history = match maybe_pending_history.unwrap() {
        Details::Entry(entry_details) => entry_details,
        Details::Element(_) => unreachable!("in has_ack_been_received()"),
    };
    debug!(" has_ack_been_received() history: {:?}", history);
    /// Return Ok if status == deleted
    if let EntryDhtStatus::Dead = history.entry_dht_status {
        return Ok(true);
    }
    /// Done
    Ok(false)
}