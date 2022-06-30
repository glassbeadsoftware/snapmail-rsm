use hdk::prelude::*;

use crate::{
    link_kind::*, path_kind,
    handle::{
        Handle,
        utils::*,
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
#[snapmail_api]
pub fn set_handle(new_name: String) -> ExternResult<HeaderHash> {
    /// -- Create Handle Entry
    let new_handle = Handle::new(new_name.to_string());
    /// -- Check if already have Handle
    let my_agent_address = agent_info()?.agent_latest_pubkey;
    let maybe_current_handle = get_handle_element(my_agent_address.clone());
    if let Some((current_handle, original_hh)) = maybe_current_handle {
        if current_handle.name == new_name.to_string() {
            return Ok(original_hh);
        }
        /// Really new name so just update entry
        let res = update_entry(original_hh, &new_handle)?;
        debug!("updated_handle_hh = {:?}", res);
        return Ok(res);
    }
    /// -- First Handle for this agent
    /// Commit entry and link to AgentHash
    let new_handle_eh = hash_entry(&new_handle)?;
    trace!("First Handle for this agent!!!");
    let new_handle_hh = create_entry(&new_handle)?;
    debug!("new_handle_hh = {:?}", new_handle_hh);
    let _ = create_link(
        EntryHash::from(my_agent_address),
        new_handle_eh.clone(),
        HdkLinkType::Any,
        LinkKind::Handle.as_tag(),
    )?;
    debug!("**** Handle linked to agent!");
    /// Link Handle to DNA entry for a global directory
    let directory_address = Path::from(path_kind::Directory).path_entry_hash().expect("Directory Path should hash");
    let _ = create_link(
        directory_address,
        new_handle_eh,
        HdkLinkType::Any,
        LinkKind::Members.as_tag())?;
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