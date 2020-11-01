use hdk3::prelude::*;

use crate::{
    ZomeString,
    link_kind,
    handle::utils::get_handle_string,
    utils::link_tag,
};

/// Zome Function
/// get an agent's latest handle
#[hdk_extern]
pub fn get_handle(agent_id: AgentPubKey) -> ExternResult<ZomeString> {
    let maybe_current_handle_entry = get_handle_element(agent_id);
    return get_handle_string(maybe_current_handle_entry);
}

/*
/// Return latest entry & entry address of an agent's Handle
pub(crate) fn get_handle_entry(agentId: &AgentPubKey) -> Option<(EntryHash, Entry)> {
    // -- Get DNA's header to retrieve the links on it
    let query_result = query!(EntryType::Dna.into());
    let dna_address = query_result.ok().unwrap()[0].clone();
    debug!(format!("dna_address33: {:?}", dna_address)).ok();
    debug!(format!("agentId33: {:?}", agentId)).ok();
    //let entry_opts = GetEntryOptions::new(StatusRequestKind::default(), false, true, Timeout::default());
    let member_links = get_links!(
        //&*hdk::DNA_ADDRESS,
        &dna_address,
        link_kind::Members
    ).expect("No reason for this to fail");
    debug!(format!("member_links: {:?}", member_links)).ok();

    // Find handle entry whose author is agentId
    for maybe_entry_result in member_links {
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
*/

// pub fn get_handle_entry(agentId: &AgentAddress) -> Option<(Address, Entry)> {
//     get_handle_entry_by_agent(agentId)
// }


/// Return Element of latest Handle Entry for agent
pub(crate) fn get_handle_element(agent_id: AgentPubKey) -> Option<Element> {
    /// Get All Handle links on agent ; should have only one
    let handle_links = get_links!(agent_id.into(), link_tag(link_kind::Handle))
       .expect("No reason for this to fail")
       .into_inner();
    assert!(handle_links.len() <= 1);
    if handle_links.len() == 0 {
        debug!("No handle found for this agent:").ok();
        return None;
    }
    /// Get the Element from the link
    let handle_entry_hash = handle_links[0].target.clone();
    let element = get!(handle_entry_hash)
        .expect("No reason for get_entry to crash")
        .expect("Should have it");

    // let entry = element.into_inner().1;
    // let entry = match entry {
    //     ElementEntry::Present(e) => e,
    //     _ => return None,
    // };

    /// Done
    return Some(element);
}
