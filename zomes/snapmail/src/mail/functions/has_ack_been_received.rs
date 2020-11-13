use hdk3::prelude::*;
use hdk3::prelude::metadata::EntryDhtStatus;

use crate::{
    ZomeBool,
    link_kind,
    mail::entries::{
        InMail,
    },
    utils::*,
};

/// Zome function
/// Ack is considered received if there is no pendingAck link or PendingAck has delete status
#[hdk_extern]
pub fn has_ack_been_received(inmail_hh: HeaderHash) -> ExternResult<ZomeBool> {
    /// 0. Get InMail (make sure InMail exists)
    let eh = hh_to_eh(inmail_hh.clone())?;
    /// 1. Get OutAck
    let links_result = get_links!(eh, link_tag(link_kind::Acknowledgment))?.into_inner();
    if links_result.len() < 1 {
        return Err(HdkError::Wasm(WasmError::Zome("No acknowledgment has been sent for this mail".to_string())));
    }
    let outack_eh = links_result[0].target.clone();
    //let outack = hdk::utils::get_as_type::<OutMail>(outack_address)?;
    /// 2. Get OutAck pending link
    let links_result = get_links!(outack_eh, link_tag(link_kind::Pending))?.into_inner();
    /// 3. If no link than return OK
    if links_result.len() < 1 {
        return Ok(ZomeBool(true));
    }
    /// 4. Otherwise get PendingAck crud status
    let pending_eh = links_result[0].target.clone();
    let maybe_pending_history = get_details!(pending_eh)?;
    if maybe_pending_history.is_none() {
        return Err(HdkError::Wasm(WasmError::Zome("No history found for PendingAck".to_string())));
    }
    let history = match maybe_pending_history.unwrap() {
        Details::Entry(entry_details) => entry_details,
        Details::Element(_) => unreachable!(),
    };
    //let history = maybe_pending_history.unwrap();
    debug!(format!(" has_ack_been_received() history: {:?}", history)).ok();
    /// 5. Return Ok if status == deleted
    if let EntryDhtStatus::Dead = history.entry_dht_status {
        return Ok(ZomeBool(true));
    }
    // for item in history.items {
    //     if let Some(meta) = item.meta {
    //         if meta.crud_status == CrudStatus::Deleted {
    //             return Ok(ZomeBool(true));
    //         }
    //     }
    // }
    /// Done
    Ok(ZomeBool(false))
}