use hdk3::prelude::*;

use crate::{
    ZomeString,
    link_kind::*, path_kind,
    utils::{
        try_from_element,
    },
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
        let header_address = handle_element.header_address().clone();
        let current_handle: Handle = try_from_element(handle_element)
            .expect("Should be a Handle entry");
        if current_handle.name == name.to_string() {
            return Ok(header_address);
        }
        /// Really new name so just update entry
        return Ok(update_entry(header_address, &new_handle)?);
    }

    /// -- First Handle for this agent
    /// Commit entry and link to AgentHash
    let entry_address = hash_entry(&new_handle)?;
    debug!("First Handle for this agent!!!").ok();
    let header_address = create_entry(&new_handle)?;
    let _ = create_link(
        EntryHash::from(my_agent_address),
        entry_address.clone(),
        LinkKind::Handle.as_tag()
    )?;
    debug!("**** Handle linked to agent!").ok();

    /// Link Handle to DNA entry for a global directory

    // TODO: hdk::DNA_ADDRESS doesnt work for linking, get the dna entry address
    //debug!(format!("DNA_ADDRESS42: {:?}", &*hdk::DNA_ADDRESS)).ok();
    // let dna_entry = hdk::get_entry(&*hdk::DNA_ADDRESS)?;
    // debug!(format!("dna_entry1: {:?}", dna_entry)).ok();

    //let query_result = query!(EntryType::Dna.into());
    //debug!(format!("query_result42: {:?}", query_result)).ok();
    //let dna_address = query_result.ok().unwrap()[0].clone();

    // let dna_entry_hash = EntryHash::from_raw_bytes_and_type(
    //     zome_info!()?.dna_hash.into_inner(),
    //     *entry_address.hash_type(),
    // );
    // debug!(format!("dna_address31: {:?}", dna_entry_hash)).ok();
    // let _ = create_link!(dna_entry_hash, entry_address, link_tag(link_kind::Members))?;

    let directory_address = Path::from(path_kind::Directory).hash().expect("Directory Path should hash");
    let _ = create_link(directory_address, entry_address, LinkKind::Members.as_tag())?;

    /// Done
    return Ok(header_address);
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