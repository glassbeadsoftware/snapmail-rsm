use hdk3::prelude::*;

/*
use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
};
*/
use crate::{
    utils::into_typed,
    entry_kind, link_kind,
    handle::{
        Handle, functions::get_handle_entry,
    },
};

/// Zome Function
/// Set handle for this agent
#[hdk_extern]
pub fn set_handle(name: String) -> ZomeApiResult<Address> {
    let my_agent_address = agent_info!()?.agent_latest_pubkey;
    let new_handle = Handle::new(name.clone());
    //let app_entry = Entry::App(entry_kind::Handle.into(), new_handle.into());
    let maybe_current_handle_entry = get_handle_entry(my_agent_address);
    if let Some((entry_address, current_handle_entry)) = maybe_current_handle_entry {
        // If handle already set to this value, just return current entry address
        let current_handle = into_typed::<Handle>(current_handle_entry)
            .expect("Should be a Handle entry");
        if current_handle.name == name {
            return Ok(entry_address);
        }
        // Really new name so just update entry
        return update_entry!(app_entry.clone(), &entry_address);
    }

    // -- First Handle ever, commit entry
    debug!("First Handle for this agent!!!").ok();
    let entry_address = create_entry!(&app_entry)?;
    let _ = create_link!(my_agent_address, &entry_address, link_kind::Handle)?;

    // TODO: hdk::DNA_ADDRESS doesnt work for linking, get the dna entry address
    //hdk::debug(format!("DNA_ADDRESS42: {:?}", &*hdk::DNA_ADDRESS)).ok();
    // let dna_entry = hdk::get_entry(&*hdk::DNA_ADDRESS)?;
    // hdk::debug(format!("dna_entry1: {:?}", dna_entry)).ok();
    let query_result = query!(EntryType::Dna.into(), 0, 0);
    //hdk::debug(format!("query_result42: {:?}", query_result)).ok();
    let dna_address = query_result.ok().unwrap()[0].clone();
    debug!(format!("dna_address31: {:?}", dna_address)).ok();
    let _ = create_link!(/*&*hdk::DNA_ADDRESS*/ &dna_address, &entry_address, link_kind::Members)?;
    return Ok(entry_address);
}

/// Zome function for testing the update_entry() API function.
#[hdk_extern]
pub fn set_three_handles(name1: String, name2: String, name3: String) -> ZomeApiResult<Address> {
    let res = set_handle(name1)?;
    set_handle(name2)?;
    set_handle(name3)?;
    Ok(res)
}