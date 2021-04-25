use hdk::prelude::*;

use crate::{
    handle::utils::*,
    handle::Handle,
    utils::*,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllHandlesOutput(Vec<(String, AgentPubKey, EntryHash)>);

/// Get all known users
/// Return (AgentId -> Handle entry address) Map
#[hdk_extern]
#[cfg_attr(not(target_arch = "wasm32"), snapmail_api)]
pub fn get_all_handles(_: ()) -> ExternResult<GetAllHandlesOutput> {
    /// Get all Members links
    let handle_links = get_members()?;
    debug!("get_all_handles() handle_links size: {:?}", handle_links.len());
    /// Find handle entry whose author is agentId
    let mut handle_list = Vec::new();
    for handle_link in handle_links {
         let maybe_handle_entry_hash = get_latest_entry_from_eh::<Handle>(handle_link.target)?;
         if maybe_handle_entry_hash.is_none() {
             continue;
         }
         let handle_entry_hash = maybe_handle_entry_hash.unwrap();
         let maybe_maybe_element = get(handle_entry_hash.1, GetOptions::latest());
         if maybe_maybe_element.is_err() {
             continue;
         }
          let maybe_element = maybe_maybe_element.unwrap();
          if maybe_element.is_none() {
             continue;
          }
         let element = maybe_element.unwrap();

         handle_list.push((
            handle_entry_hash.0.name.clone(),
            element.header().author().clone(),
            handle_entry_hash.2.clone(),
         ));
    }
    debug!("get_all_handles() handle_map size: {}", handle_list.len());
    /// Done
    return Ok(GetAllHandlesOutput(handle_list))
}
