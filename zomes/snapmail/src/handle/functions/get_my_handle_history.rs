use hdk::prelude::*;

use crate::{
    handle::Handle,
};

/// Zome function
#[hdk_extern]
pub fn get_my_handle_history(initial_handle_address: HeaderHash) -> Vec<String> {

    let history_result = get_details(&initial_handle_address, GetOptions::latest());
    if let Err(_e) = history_result {
        error!("get_entry_history() failed");
        return Vec::new();
    }
    let maybe_history = history_result.unwrap();
    if maybe_history.is_none() {
        error!("No history found");
        return Vec::new();
    }
    let history = maybe_history.unwrap();
    debug!("History length: {}", history.items.len());
    debug!("History crud_links length: {}", history.crud_links.len());

    let mut handle_list = Vec::new();

    for item in history.items {
        let handle_entry = item.entry.expect("should have entry");
        trace!("History headers length: {}", item.headers.len());
        trace!("item crud status: {:?}", item.meta.unwrap().crud_status);
        let handle = crate::into_typed::<Handle>(handle_entry).expect("Should be Handle");
        handle_list.push(handle.name);
    }
    debug!("handle_list = {}", handle_list.len());
    handle_list
}