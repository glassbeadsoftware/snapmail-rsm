use hdk::prelude::*;

use crate::{
    link_kind::*,
    mail::entries::*,
    utils::*,
};


/// Get State of InMail
pub(crate) fn get_inmail_state(inmail_hh: &HeaderHash) -> ExternResult<InMailState> {
    /// Get inMail Details
    let maybe_details = get_details(inmail_hh.clone(), GetOptions::latest())?;
    if maybe_details.is_none() {
        return error("No InMail at given address");
    }
    let el_details = match maybe_details.unwrap() {
        Details::Element(details) => details,
        Details::Entry(_) => unreachable!("in get_outmail_state()"),
    };
    /// Check if deleted
    if el_details.deletes.len() > 0 {
        return Ok(InMailState::Deleted);
    }
    /// Get OutMail Entry
    let inmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");
    /// Get OutAck
    let links_result = get_links(inmail_eh.clone(), LinkKind::Acknowledgment.as_tag_opt())?;
    if links_result.len() < 1 {
        return Ok(InMailState::Arrived);
    }
    let ack_link = links_result[0].clone();
    /// Get PendingAck
    let links_result = get_links(ack_link.target, LinkKind::Pending.as_tag_opt())?;
    /// If link found, it means Ack has not been received
    if links_result.len() > 0 {
        return Ok(InMailState::Acknowledged);
    }
    Ok(InMailState::AckReceived)
}


/// Return address of created InAck
pub(crate) fn commit_inack(outmail_eh: EntryHash, from: &AgentPubKey) -> ExternResult<HeaderHash> {
    debug!("Create inAck for: {} ({})", outmail_eh, from);
    /// Create InAck
    let inack = InAck::new();
    let inack_hh = create_entry(&inack)?;
    let inack_eh = hash_entry(&inack)?;
    //debug!("inack_hh: {}", inack_hh);
    //debug!("inack_eh: {}", inack_eh);
    /// Create link tag
    let tag = LinkKind::Receipt.concat_hash(&from);
    /// Create link from OutMail
    let link_hh = create_link(outmail_eh, inack_eh, tag)?;
    debug!("inAck link_hh = {}", link_hh);
    /// Done
    Ok(inack_hh)
}
