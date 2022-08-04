use hdk::prelude::*;
use hdk::prelude::link::Link;
use snapmail_model::*;
use zome_utils::*;

use crate::{
    link_kind::*,
    path_kind,
};


/// Get 'Members' links on the DNA entry
pub(crate) fn get_members() -> ExternResult<Vec<Link>> {
    let path_hash = Path::from(path_kind::Directory).path_entry_hash()?;
    let entry_results = get_links(path_hash, LinkKind::Members, None)?;
    Ok(entry_results)
}


/// Return Record of latest Handle Entry for agent
pub(crate) fn get_handle_element(agent_id: AgentPubKey) -> Option<(Handle, ActionHash)> {
    /// Get All Handle links on agent ; should have only one
    let handle_links = get_links(agent_id, LinkKind::Handle, None)
       .expect("No reason for this to fail");
    assert!(handle_links.len() <= 1);
    if handle_links.len() == 0 {
        warn!("No handle found for this agent:");
        return None;
    }
    /// Get the Entry from the link
    let handle_eh: EntryHash = handle_links[0].target.clone().into();
    let handle_and_hash = get_latest_typed_from_eh::<Handle>(handle_eh.clone())
       .expect("No reason for get_entry to crash")
       .expect("Should have it");
    /// Look for original record
    let original_element = match get(handle_eh.clone(), GetOptions::content()) {
        Ok(Some(record)) => record,
        _ => return None,
    };
    /// Done
    return Some((handle_and_hash.0, original_element.action_address().clone()));
}
