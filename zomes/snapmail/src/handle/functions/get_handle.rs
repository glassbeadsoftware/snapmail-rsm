use hdk3::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        link::LinkMatch,
    },
    holochain_core_types::time::Timeout,
};

use crate::{
    AgentAddress,
    link_kind,
    handle::utils::get_handle_string,
};

/// Zome Function
/// get an agent's latest handle
pub fn get_handle(agentId: AgentAddress) -> ZomeApiResult<String> {
    let maybe_current_handle_entry = get_handle_entry(&agentId);
    return get_handle_string(maybe_current_handle_entry);
}

/// Return handle entry address and entry
pub(crate) fn get_handle_entry(agentId: &AgentAddress) -> Option<(Address, Entry)> {
    let query_result = query!(EntryType::Dna.into(), 0, 0);
    let dna_address = query_result.ok().unwrap()[0].clone();
    debug!(format!("dna_address33: {:?}", dna_address)).ok();
    debug!(format!("agentId33: {:?}", agentId)).ok();
    let entry_opts = GetEntryOptions::new(StatusRequestKind::default(), false, true, Timeout::default());
    let entry_results = get_link_details!(
        //&*hdk::DNA_ADDRESS,
        &dna_address,
        LinkMatch::Exactly(link_kind::Members),
        LinkMatch::Any,
        GetLinksOptions::default(),
        entry_opts,
    ).expect("No reason for this to fail");
    debug!(format!("entry_results33: {:?}", entry_results)).ok();

    // Find handle entry whose author is agentId
    for maybe_entry_result in entry_results {
        if let Ok(entry_result) = maybe_entry_result {
            let item = match entry_result.result {
                GetEntryResultType::Single(result_item) => result_item,
                GetEntryResultType::All(history) => history.items[0].clone(),
            };
            let header = item.headers[0].clone();
            let from = header.provenances()[0].clone();
            if from.source() == agentId.clone() {
                debug!("agentId33 match").ok();
                return Some((header.entry_address().clone(), item.entry.unwrap()))
            }
        }
    }
    debug!("None33").ok();
    return None;
}

// pub fn get_handle_entry(agentId: &AgentAddress) -> Option<(Address, Entry)> {
//     get_handle_entry_by_agent(agentId)
// }

/// Return (handle entry address, handle entry) pair
pub fn _get_handle_entry_by_agent(agentId: &AgentAddress) -> Option<(Address, Entry)> {
    let link_results = get_links!(
        agentId,
        LinkMatch::Exactly(link_kind::Handle),
        LinkMatch::Any,
    ).expect("No reason for this to fail");
    let links_result = link_results.links();
    assert!(links_result.len() <= 1);
    if links_result.len() == 0 {
        hdk::debug("No handle found for this agent:").ok();
        return None;
    }
    let entry_address = &links_result[0].address;
    let entry = get!(entry_address)
        .expect("No reason for get_entry to crash")
        .expect("Should have it");
    return Some((entry_address.clone(), entry));
}
