use hdk3::prelude::*;
use hdk3::prelude::link::Link;

use crate::{
    link_kind::*,
    mail::entries::*,
    utils::*,
};

// FIXME: Hack

pub(crate) fn get_outmail_state(_outmail_address: &EntryHash) -> ExternResult<OutMailState> {
    return Ok(OutMailState::Received);
}

// /// Get State of InMail at given address
// /// If get_entry() returns nothing we presume the entry has been deleted
// pub(crate) fn get_outmail_state(outmail_address: &HeaderHash) -> ExternResult<OutMailState> {
//     // 1. Get OutMail
//     let maybe_outmail = hdk::utils::get_as_type::<OutMail>(outmail_address.clone());
//     if let Err(_) = maybe_outmail {
//         // return Err(ZomeApiError::Internal("No OutMail at given address".to_string()));
//         return Ok(OutMailState::Deleted);
//     }
//     let outmail = maybe_outmail.unwrap();
//     let receipient_count = outmail.bcc.len() + outmail.mail.to.len() + outmail.mail.cc.len();
//     // 2. Get Pendings links
//     let pendings = hdk::get_links_count(&outmail_address, LinkMatch::Exactly(link_kind::Pendings), LinkMatch::Any)?;
//     // 3. Get Receipt links
//     let receipts = hdk::get_links_count(&outmail_address, LinkMatch::Exactly(link_kind::Receipt), LinkMatch::Any)?;
//     // 4. Determine state
//     if pendings.count == receipient_count {
//         return Ok(OutMailState::Pending);
//     }
//     if pendings.count == 0 {
//         if receipts.count == 0 {
//             return Ok(OutMailState::Arrived_NoAcknowledgement);
//         }
//         if receipts.count == receipient_count {
//             return Ok(OutMailState::Received);
//         }
//         return Ok(OutMailState::Arrived_PartiallyAcknowledged);
//     }
//     if receipts.count == 0 {
//         return Ok(OutMailState::PartiallyArrived_NoAcknowledgement);
//     }
//     return Ok(OutMailState::PartiallyArrived_PartiallyAcknowledged);
// }

/// Get State of InMail at given address
/// If get_entry() returns nothing we presume the entry has been deleted
pub(crate) fn get_inmail_state(inmail_eh: &EntryHash) -> ExternResult<InMailState> {
    // /// 1. Should have InMail
    // let maybe_inmail = hdk::utils::get_as_type::<InMail>(inmail_eh.clone());
    // if let Err(_) = maybe_inmail {
    //     return Ok(InMailState::Deleted);
    //     // return Err(ZomeApiError::Internal("No InMail at given address".to_string()));
    // }
    /// 2. Get OutAck
    let links_result: Vec<Link> = get_links(
    inmail_eh.clone(),
    LinkKind::Acknowledgment.as_tag_opt(),
    )?.into_inner();

    if links_result.len() < 1 {
        return Ok(InMailState::Arrived);
    }
    let ack_link = links_result[0].clone();
    /// 3. Get PendingAck
    let links_result = get_links(ack_link.target, LinkKind::Pending.as_tag_opt())
       ?.into_inner();
    /// If link found, it means Ack has not been received
    if links_result.len() > 0 {
        return Ok(InMailState::Acknowledged);
    }
    Ok(InMailState::AckReceived)
}

/// Conditions: Must be a single author entry type
pub(crate) fn get_entry_and_author<T: TryFrom<SerializedBytes>>(eh: &EntryHash)
    -> ExternResult<(AgentPubKey, T)>
{
    // let get_options = GetEntryOptions {
    //     status_request: StatusRequestKind::Latest,
    //     entry: true,
    //     headers: true,
    //     timeout: Timeout::default(),
    // };
    let maybe_maybe_element = get(eh.clone(), GetOptions);
    if let Err(err) = maybe_maybe_element {
        debug!("Failed getting element: {}", err).ok();
        return Err(err);
    }
    let maybe_element = maybe_maybe_element.unwrap();
    if maybe_element.is_none() {
        return error("no element found at address");
    }
    let element = maybe_element.unwrap();
    //assert!(entry_item.headers.len() > 0);
    //assert!(entry_item.headers[0].provenances().len() > 0);
    let author = element.header().author();
    let app_entry = try_from_element::<T>(element.clone())?;
    Ok((author.clone(), app_entry))
}

///
pub(crate) fn get_pending_mail(pending_eh: &EntryHash) -> ExternResult<(AgentPubKey, PendingMail)> {
    let (author, pending_mail) = get_entry_and_author::<PendingMail>(pending_eh)?;
    Ok((author, pending_mail))
}

///
pub(crate) fn get_pending_ack(pending_eh: &EntryHash) -> ExternResult<(AgentPubKey, PendingAck)> {
    let (author, ack) = get_entry_and_author::<PendingAck>(pending_eh)?;
    Ok((author, ack))
}

/// Return address of created InAck
pub(crate) fn create_and_commit_inack(outmail_eh: EntryHash, from: &AgentPubKey) -> ExternResult<HeaderHash> {
    debug!("Create inAck for: {} ({})", outmail_eh, from).ok();
    /// Create InAck
    let inack = InAck::new();
    let inack_hh = create_entry(&inack)?;
    let inack_eh = hash_entry(&inack)?;
    //debug!("inack_eh: {}", inack_eh).ok();
    /// Create link tag
    //let vec = from.clone().into_inner();
    let recepient = format!("{}", from);
    let tag = LinkKind::Receipt.concat(&recepient);
    /// Create link from OutMail
    let link_hh = create_link(outmail_eh, inack_eh, tag)?;
    debug!("inAck link_hh = {}", link_hh).ok();
    /// Done
    Ok(inack_hh)
}
