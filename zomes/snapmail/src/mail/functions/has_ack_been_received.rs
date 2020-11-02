use hdk3::prelude::*;

use crate::{
    link_kind,
    mail::entries::{
        InMail,
    },
};

/// Zome function
/// Ack is considered received if there is no pendingAck link or PendingAck has delete status
#[hdk_extern]
pub fn has_ack_been_received(inmail_address: HeaderHash) -> ExternResult<ZomeBool> {
    // 0. Get InMail (make sure InMail exists)
    let _ = hdk::utils::get_as_type::<InMail>(inmail_address.clone())?;
    // 1. Get OutAck
    let links_result = get_links!(&inmail_address, link_tag(link_kind::Acknowledgment))?;
    if links_result.links().len() < 1 {
        return Err(ZomeApiError::Internal("No acknowledgment has been sent for this mail".to_string()));
    }
    let outack_address = links_result.addresses()[0].clone();
    //let outack = hdk::utils::get_as_type::<OutMail>(outack_address)?;
    // 2. Get OutAck pending link
    let links_result = get_links!(&outack_address, link_tag(link_kind::Pending))?;
    // 3. If no link than return OK
    if links_result.links().len() < 1 {
        return Ok(true);
    }
    // 4. Otherwise get PendingAck crud status
    let pending_address = links_result.addresses()[0].clone();
    let maybe_pending_history = get_details!(&pending_address)?;
    if maybe_pending_history.is_none() {
        return Err(ExternResult::Internal("No history found for PendingAck".to_string()));
    }
    // 5. Return Ok if status == deleted
    let history = maybe_pending_history.unwrap();
    for item in history.items {
        if let Some(meta) = item.meta {
            if meta.crud_status == CrudStatus::Deleted {
                return Ok(true);
            }
        }
    }
    Ok(false)
}