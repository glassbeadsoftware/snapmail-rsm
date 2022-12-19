use hdk::prelude::*;
use snapmail_model::*;

use crate::{
    path_kind,
    handle::utils::*,
};


/// Zome Function
/// DEBUG / TESTING ONLY
#[hdk_extern]
pub fn create_empty_handle(_: ()) -> ExternResult<ActionHash> {
    let new_handle = Handle::empty();
    let ah = create_entry(SnapmailEntry::Handle(new_handle))?;
    Ok(ah)
}

/// Zome Function
/// Set handle for this agent
#[hdk_extern]
//#[snapmail_api]
pub fn set_handle(new_name: String) -> ExternResult<ActionHash> {
    /// -- Create Handle Entry
    let new_handle = Handle::new(new_name.to_string());
    /// -- Check if already have Handle
    let my_agent_address = agent_info()?.agent_latest_pubkey;
    let maybe_current_handle = get_handle_element(my_agent_address.clone());
    if let Some((current_handle, original_ah)) = maybe_current_handle {
        if current_handle.name == new_name.to_string() {
            return Ok(original_ah);
        }
        /// Really new name so just update entry
        let res = update_entry(original_ah, &new_handle)?;
        debug!("updated_handle_ah = {:?}", res);
        return Ok(res);
    }
    /// -- First Handle for this agent
    /// Commit entry and link to AgentHash
    let new_handle_eh = hash_entry(&new_handle)?;
    trace!("First Handle for this agent!!!");
    let new_handle_ah = create_entry(SnapmailEntry::Handle(new_handle))?;
    debug!("new_handle_ah = {:?}", new_handle_ah);
    let _ = create_link(
        EntryHash::from(my_agent_address),
        new_handle_eh.clone(),
        LinkKind::Handle,
        LinkTag::from(()),
    )?;
    debug!("**** Handle linked to agent!");
    /// Link Handle to DNA entry for a global directory
    let directory_address = Path::from(path_kind::Directory).path_entry_hash().expect("Directory Path should hash");
    let _ = create_link(
        directory_address,
        new_handle_eh,
        LinkKind::Members,
        LinkTag::from(()),
    )?;
    /// Done
    return Ok(new_handle_ah);
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
