use hdk3::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
};

use crate::{
    AgentAddress,
    handle::utils::get_members,
    handle::Handle,
};

/// Get all known users
/// Return (AgentId -> Handle entry address) Map
#[hdk_extern]
pub fn get_all_handles() -> ZomeApiResult<Vec<(String, AgentAddress, Address)>> {
    // Get all Members links
    let entry_results = get_members();
    debug!(format!("entry_results55 size: {:?}", entry_results.len())).ok();

    // Find handle entry whose author is agentId
    let mut handle_list = Vec::new();
    // Find handle entry whose author is agentId
    for maybe_entry_result in entry_results {
        if let Ok(entry_result) = maybe_entry_result {
            let item = match entry_result.result {
                GetEntryResultType::Single(result_item) => result_item,
                GetEntryResultType::All(history) => history.items[0].clone(),
            };
            let entry = item.entry.unwrap();
            let handle_entry = crate::into_typed::<Handle>(entry).expect("Should be Handle");
            let header = item.headers[0].clone();
            let from = header.provenances()[0].clone();
            handle_list.push((handle_entry.name.clone(), from.source(), header.entry_address().clone()));
        }
    }
    debug!(format!("handle_map size: {}", handle_list.len())).ok();
    return Ok(handle_list)
}
