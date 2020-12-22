use hdk3::prelude::*;

use crate::{
    ZomeString,
    link_kind::*,
    handle::utils::get_handle_string,
};

/// Zome Function
/// get an agent's latest handle
#[hdk_extern]
pub fn get_handle(agent_id: AgentPubKey) -> ExternResult<ZomeString> {
    let maybe_current_handle_entry = get_handle_element(agent_id);
    return get_handle_string(maybe_current_handle_entry);
}

/// Return Element of latest Handle Entry for agent
pub(crate) fn get_handle_element(agent_id: AgentPubKey) -> Option<Element> {
    /// Get All Handle links on agent ; should have only one
    let handle_links = get_links(agent_id.into(), LinkKind::Handle.as_tag_opt())
       .expect("No reason for this to fail")
       .into_inner();
    assert!(handle_links.len() <= 1);
    if handle_links.len() == 0 {
        debug!("No handle found for this agent:");
        return None;
    }
    /// Get the Element from the link
    let handle_entry_hash = handle_links[0].target.clone();
    let element = get(handle_entry_hash, GetOptions::latest())
        .expect("No reason for get_entry to crash")
        .expect("Should have it");
    /// Done
    return Some(element);
}
