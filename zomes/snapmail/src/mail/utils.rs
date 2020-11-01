use hdk3::prelude::*;

// use hdk::{
//     error::{
//         ZomeApiResult,
//     //    ZomeApiError,
//     },
//     holochain_persistence_api::{
//     cas::content::Address
// }, holochain_core_types::{
//     entry::Entry,
//     time::Timeout,
//     },
// };

use holochain_wasm_utils::{
    api_serialization::get_entry::{
        GetEntryOptions, StatusRequestKind, GetEntryResultType,
    },
    holochain_core_types::link::LinkMatch,
};
use crate::{
    link_kind, entry_kind,
    mail::entries::*,
    AgentAddress,
};

/// Get State of InMail at given address
/// If get_entry() returns nothing we presume the entry has been deleted
pub(crate) fn get_outmail_state(outmail_address: &Address) -> ZomeApiResult<OutMailState> {
    // 1. Get OutMail
    let maybe_outmail = hdk::utils::get_as_type::<OutMail>(outmail_address.clone());
    if let Err(_) = maybe_outmail {
        // return Err(ZomeApiError::Internal("No OutMail at given address".to_string()));
        return Ok(OutMailState::Deleted);
    }
    let outmail = maybe_outmail.unwrap();
    let receipient_count = outmail.bcc.len() + outmail.mail.to.len() + outmail.mail.cc.len();
    // 2. Get Pendings links
    let pendings = hdk::get_links_count(&outmail_address, LinkMatch::Exactly(link_kind::Pendings), LinkMatch::Any)?;
    // 3. Get Receipt links
    let receipts = hdk::get_links_count(&outmail_address, LinkMatch::Exactly(link_kind::Receipt), LinkMatch::Any)?;
    // 4. Determine state
    if pendings.count == receipient_count {
        return Ok(OutMailState::Pending);
    }
    if pendings.count == 0 {
        if receipts.count == 0 {
            return Ok(OutMailState::Arrived_NoAcknowledgement);
        }
        if receipts.count == receipient_count {
            return Ok(OutMailState::Received);
        }
        return Ok(OutMailState::Arrived_PartiallyAcknowledged);
    }
    if receipts.count == 0 {
        return Ok(OutMailState::PartiallyArrived_NoAcknowledgement);
    }
    return Ok(OutMailState::PartiallyArrived_PartiallyAcknowledged);
}

/// Get State of InMail at given address
/// If get_entry() returns nothing we presume the entry has been deleted
pub(crate) fn get_inmail_state(inmail_address: &Address) -> ZomeApiResult<InMailState> {
    // 1. Should have InMail
    let maybe_inmail = hdk::utils::get_as_type::<InMail>(inmail_address.clone());
    if let Err(_) = maybe_inmail {
        return Ok(InMailState::Deleted);
        // return Err(ZomeApiError::Internal("No InMail at given address".to_string()));
    }
    // 2. Get OutAck
    let links_result = hdk::get_links(&inmail_address,LinkMatch::Exactly(link_kind::Acknowledgment), LinkMatch::Any)?;
    if links_result.links().len() < 1 {
        return Ok(InMailState::Arrived);
    }
    let ack_link = links_result.links()[0].clone();
    // 3. Get PendingAck
    let links_result = hdk::get_links(&ack_link.address,LinkMatch::Exactly(link_kind::Pending), LinkMatch::Any)?;
    // If link found, it means Ack has not been received
    if links_result.links().len() > 0 {
        return Ok(InMailState::Acknowledged);
    }
    Ok(InMailState::AckReceived)
}

/// Conditions: Must be a single author entry type
pub(crate) fn get_entry_and_author(address: &Address) -> ZomeApiResult<(AgentAddress, Entry)> {
    let get_options = GetEntryOptions {
        status_request: StatusRequestKind::Latest,
        entry: true,
        headers: true,
        timeout: Timeout::default(),
    };
    let maybe_entry_result = hdk::get_entry_result(address, get_options);
    if let Err(err) = maybe_entry_result {
        debug!(format!("Failed getting address: {}", err)).ok();
        return Err(err);
    }
    let entry_result = maybe_entry_result.unwrap();
    let entry_item = match entry_result.result {
        GetEntryResultType::Single(item) => {
            item
        },
        _ => panic!("Asked for latest so should get Single"),
    };
    assert!(entry_item.headers.len() > 0);
    assert!(entry_item.headers[0].provenances().len() > 0);
    let author = entry_item.headers[0].provenances()[0].source();
    let entry = entry_item.entry.expect("Should have Entry");
    Ok((author, entry))
}

pub(crate) fn get_pending_mail(pending_address: &Address) -> ZomeApiResult<(AgentAddress, PendingMail)> {
    let (author, entry) = get_entry_and_author(pending_address)?;
    let pending = crate::into_typed::<PendingMail>(entry).expect("Should be PendingMail");
    Ok((author, pending))
}

pub(crate) fn get_pending_ack(ack_address: &Address) -> ZomeApiResult<(AgentAddress, PendingAck)> {
    let (author, entry) = get_entry_and_author(ack_address)?;
    let ack = crate::into_typed::<PendingAck>(entry).expect("Should be PendingAck");
    Ok((author, ack))
}

/// Return address of created InAck
pub(crate) fn create_and_commit_inack(outmail_address: &Address, from: &AgentAddress) -> ZomeApiResult<Address> {
    debug!(format!("Create inAck for: {} ({})", outmail_address, from)).ok();
    // Create InAck
    let inack = InAck::new();
    let inack_entry = Entry::App(entry_kind::InAck.into(), inack.into());
    let inack_address = hdk::commit_entry(&inack_entry)?;
    let json_from = serde_json::to_string(from).expect("Should stringify");
    // Create link from OutMail
    let link_address = hdk::link_entries(
        outmail_address,
        &inack_address,
        link_kind::Receipt,
        json_from.as_str().into(),
    )?;
    debug!(format!("inAck link address: {}", link_address)).ok();
    Ok(inack_address)
}
