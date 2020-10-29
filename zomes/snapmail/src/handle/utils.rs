use hdk::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_core_types::{
        link::LinkMatch,
    },
    holochain_core_types::time::Timeout,
};

use crate::{
    utils::into_typed,
    link_kind,
    handle::Handle,
};

///
pub(crate) fn get_handle_string(maybe_handle_entry: Option<(Address, Entry)>) -> ZomeApiResult<String> {
    if let Some((_, current_handle_entry)) = maybe_handle_entry {
        let current_handle = into_typed::<Handle>(current_handle_entry)
            .expect("Should be a Handle entry");
        return Ok(current_handle.name);
    }
    return Ok("<noname>".to_string());
}

/// Get 'Members' links on the DNA entry
pub(crate) fn get_members() -> Vec<ZomeApiResult<GetEntryResult>>{
    // Get DNA entry address
    let query_result = hdk::query(EntryType::Dna.into(), 0, 0);
    let dna_address = query_result.ok().unwrap()[0].clone();
    // Get 'Members' links on DNA
    let entry_opts = GetEntryOptions::new(StatusRequestKind::default(), true, true, Timeout::default());
    let entry_results = hdk::get_links_result(
        //&*hdk::DNA_ADDRESS,
        &dna_address,
        LinkMatch::Exactly(link_kind::Members),
        LinkMatch::Any,
        GetLinksOptions::default(),
        entry_opts,
    ).expect("No reason for this to fail");
    entry_results
}