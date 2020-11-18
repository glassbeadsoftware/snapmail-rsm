use hdk3::prelude::*;
use hdk3::prelude::link::Link;

use std::str;

use crate::{
    link_kind, entry_kind,
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
    let links_result: Vec<Link> = get_links!(
    inmail_eh.clone(),
    link_tag(link_kind::Acknowledgment,
    ))
    ?.into_inner();

    if links_result.len() < 1 {
        return Ok(InMailState::Arrived);
    }
    let ack_link = links_result[0].clone();
    /// 3. Get PendingAck
    let links_result = get_links!(ack_link.target, link_tag(link_kind::Pending))
       ?.into_inner();
    /// If link found, it means Ack has not been received
    if links_result.len() > 0 {
        return Ok(InMailState::Acknowledged);
    }
    Ok(InMailState::AckReceived)
}
//
// /// Conditions: Must be a single author entry type
// pub(crate) fn get_entry_and_author(address: &Address) -> ExternResult<(AgentPubKey, Entry)> {
//     let get_options = GetEntryOptions {
//         status_request: StatusRequestKind::Latest,
//         entry: true,
//         headers: true,
//         timeout: Timeout::default(),
//     };
//     let maybe_entry_result = hdk::get_entry_result(address, get_options);
//     if let Err(err) = maybe_entry_result {
//         debug!(format!("Failed getting address: {}", err)).ok();
//         return Err(err);
//     }
//     let entry_result = maybe_entry_result.unwrap();
//     let entry_item = match entry_result.result {
//         GetEntryResultType::Single(item) => {
//             item
//         },
//         _ => panic!("Asked for latest so should get Single"),
//     };
//     assert!(entry_item.headers.len() > 0);
//     assert!(entry_item.headers[0].provenances().len() > 0);
//     let author = entry_item.headers[0].provenances()[0].source();
//     let entry = entry_item.entry.expect("Should have Entry");
//     Ok((author, entry))
// }
//
// pub(crate) fn get_pending_mail(pending_address: &Address) -> ExternResult<(AgentAddress, PendingMail)> {
//     let (author, entry) = get_entry_and_author(pending_address)?;
//     let pending = crate::into_typed::<PendingMail>(entry).expect("Should be PendingMail");
//     Ok((author, pending))
// }
//
// pub(crate) fn get_pending_ack(ack_address: &Address) -> ExternResult<(AgentAddress, PendingAck)> {
//     let (author, entry) = get_entry_and_author(ack_address)?;
//     let ack = crate::into_typed::<PendingAck>(entry).expect("Should be PendingAck");
//     Ok((author, ack))
// }

/// Return address of created InAck
pub(crate) fn create_and_commit_inack(outmail_eh: EntryHash, from: &AgentPubKey) -> ExternResult<HeaderHash> {
    debug!("Create inAck for: {} ({})", outmail_eh, from).ok();
    /// Create InAck
    let inack = InAck::new();
    let inack_hh = create_entry!(&inack)?;
    let inack_eh = hash_entry!(&inack)?;
    //debug!("inack_eh: {}", inack_eh).ok();
    /// Create link tag
    let vec = from.clone().into_inner();
    let recepient = format!("{}", from);

    //let recepient = str::from_utf8_lossy(&vec);
    debug!("recepient: {}", recepient).ok();

    // let recepient = match str::from_utf8(&vec) {
    //     Ok(v) => v,
    //     Err(e) => {
    //         let err_msg = format!("Invalid UTF-8 sequence: {}", e);
    //         debug!(err_msg).ok();
    //         return Err(HdkError::Wasm(WasmError::Zome(err_msg)));
    //
    //     },
    // };
    let tag = link_tag(format!("{}___{}", link_kind::Receipt, recepient).as_str());
    /// Create link from OutMail
    let link_address = create_link!(outmail_eh, inack_eh, tag)?;
    debug!(format!("inAck link address: {}", link_address)).ok();
    /// Done
    Ok(inack_hh)
}
