use hdk::prelude::*;
use hdk::prelude::link::Link;

use crate::{
    utils::*,
    link_kind::*,
    path_kind,
    handle::Handle,
};


/// Get 'Members' links on the DNA entry
pub(crate) fn get_members() -> ExternResult<Vec<Link>> {
    let path_hash = Path::from(path_kind::Directory).hash()?;
    let entry_results = get_links(path_hash, LinkKind::Members.as_tag_opt())?;
    Ok(entry_results.into_inner())
}


/// Return Element of latest Handle Entry for agent
pub(crate) fn get_handle_element(agent_id: AgentPubKey) -> Option<(Handle, HeaderHash)> {
    /// Get All Handle links on agent ; should have only one
    let handle_links = get_links(agent_id.into(), LinkKind::Handle.as_tag_opt())
       .expect("No reason for this to fail")
       .into_inner();
    assert!(handle_links.len() <= 1);
    if handle_links.len() == 0 {
        warn!("No handle found for this agent:");
        return None;
    }
    /// Get the Entry from the link
    let handle_eh = handle_links[0].target.clone();
    let handle_and_hash = get_latest_typed_from_eh::<Handle>(handle_eh.clone())
       .expect("No reason for get_entry to crash")
       .expect("Should have it");
    /// Look for original element
    let original_element = match get(handle_eh.clone(), GetOptions::content()) {
        Ok(Some(element)) => element,
        _ => return None,
    };
    /// Done
    return Some((handle_and_hash.0, original_element.header_address().clone()));
}