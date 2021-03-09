use hdk::prelude::*;

use crate::{
    ZomeString,
    link_kind::*, path_kind,
    utils::*,
    handle::{
        Handle,
        functions::get_handle_element,
    },
};

/// Zome Function
/// DEBUG / TESTING ONLY
#[hdk_extern]
pub fn create_empty_handle(_: ()) -> ExternResult<HeaderHash> {
    let new_handle = Handle::empty();
    let hh = create_entry(&new_handle)?;
    Ok(hh)
}

/// Zome Function
/// Set handle for this agent
#[hdk_extern]
pub fn set_handle(name: ZomeString) -> ExternResult<HeaderHash> {
    /// -- Create Handle Entry
    let new_handle = Handle::new(name.to_string());
    /// -- Check if already have Handle
    let my_agent_address = agent_info()?.agent_latest_pubkey;
    let maybe_current_handle_element = get_handle_element(my_agent_address.clone());
    if let Some(handle_element) = maybe_current_handle_element {
        /// If new handle same as current, just return current entry address
        let handle_hh = handle_element.header_address().clone();
        let current_handle: Handle = get_typed_from_el(handle_element)
            .expect("Should be a Handle entry");
        if current_handle.name == name.to_string() {
            return Ok(handle_hh);
        }
        /// Really new name so just update entry
        return Ok(update_entry(handle_hh, &new_handle)?);
    }
    /// -- First Handle for this agent
    /// Commit entry and link to AgentHash
    let new_handle_eh = hash_entry(&new_handle)?;
    debug!("First Handle for this agent!!!");
    let new_handle_hh = create_entry(&new_handle)?;
    debug!("new_handle_hh = {:?}", new_handle_hh);
    let _ = create_link(
        EntryHash::from(my_agent_address),
        new_handle_eh.clone(),
        LinkKind::Handle.as_tag(),
    )?;
    debug!("**** Handle linked to agent!");
    /// Link Handle to DNA entry for a global directory
    let directory_address = Path::from(path_kind::Directory).hash().expect("Directory Path should hash");
    let _ = create_link(directory_address, new_handle_eh, LinkKind::Members.as_tag())?;
    /// Done
    return Ok(new_handle_hh);
}


/*
/// Zome function for testing the update_entry() API function.
#[hdk_extern]
pub fn set_three_handles(name1: String, name2: String, name3: String) -> ExternResult<EntryHash> {
    let res = set_handle(name1)?;
    set_handle(name2)?;
    set_handle(name3)?;
    Ok(res)
}
*/