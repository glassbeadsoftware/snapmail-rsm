use hdk3::prelude::*;

use crate::{
    utils::try_from_entry,
    handle::Handle,
    handle::utils::get_members,
};

/// Get all agentIds that have a certain handle
/// Return [AgentId]
#[hdk_extern]
pub fn find_agent(handle: String) -> ExternResult<Vec<AgentPubKey>> {
    let entry_results = get_members();
    let mut agent_list = Vec::new();
    /// Find handle entry whose author is agentId
    for maybe_entry_result in entry_results {
        if let Ok(entry_result) = maybe_entry_result {
            let item = match entry_result.result {
                GetEntryResultType::Single(result_item) => result_item,
                GetEntryResultType::All(history) => history.items[0].clone(),
            };
            let entry = item.entry.unwrap();
            let handle_entry = try_from_entry(entry).expect("Should be Handle");
            let header = item.headers[0].clone();
            let from = header.provenances()[0].clone();
            if handle_entry.name == handle {
                agent_list.push(from.source());
            }
        }
    }
    /// Done
    debug!("agent_list size: {}", agent_list.len());
    return Ok(agent_list)
}
