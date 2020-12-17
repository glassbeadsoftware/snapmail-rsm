use hdk3::prelude::*;

use crate::{
    handle::utils::*,
    handle::Handle,
   utils::*,
};

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct GetAllHandlesOutput(Vec<(String, AgentPubKey, EntryHash)>);

/// Get all known users
/// Return (AgentId -> Handle entry address) Map
#[hdk_extern]
pub fn get_all_handles(_: ()) -> ExternResult<GetAllHandlesOutput> {
    /// Get all Members links
    let handle_links = get_members()?;
    debug!("handle_links size: {:?}", handle_links.len());

    /// Find handle entry whose author is agentId
    let mut handle_list = Vec::new();
    for handle_link in handle_links {
         let maybe_handle_entry_hash = get_latest_for_entry::<Handle>(handle_link.target)?;
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
         /*
     if let Ok(entry_result) = handle_link {
         let item = match entry_result.result {
             GetEntryResultType::Single(result_item) => result_item,
             GetEntryResultType::All(history) => history.items[0].clone(),
         };
         let entry = item.entry.unwrap();
         let handle_entry = crate::into_typed::<Handle>(entry).expect("Should be Handle");
         let header = item.headers[0].clone();
         let from = header.provenances()[0].clone();
         */
         handle_list.push((
            handle_entry_hash.0.name.clone(),
            element.header().author().clone(),
            handle_entry_hash.2.clone(),
         ));
    }
    debug!("handle_map size: {}", handle_list.len());
    /// Done
    return Ok(GetAllHandlesOutput(handle_list))
}
