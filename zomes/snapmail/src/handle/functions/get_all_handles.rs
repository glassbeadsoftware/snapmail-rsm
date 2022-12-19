use hdk::prelude::*;
use zome_utils::*;
use snapmail_model::*;

use crate:: handle::utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct HandleItem {
   pub name: String,
   pub agentId: AgentPubKey,
   pub handle_eh: EntryHash,
}


/// Get all known users
/// Return (AgentId -> Handle entry address) Map
#[hdk_extern]
//#[snapmail_api]
pub fn get_all_handles(_: ()) -> ExternResult<Vec<HandleItem>> {
   /// Get all Members links
   let member_links = get_members()?;
   trace!("get_all_handles() handle_links size: {:?}", member_links.len());
   /// Find each Handle from links
   let mut handle_list = Vec::new();
   for member_link in member_links {
      let handle_eh = member_link.target.clone().into();
      trace!("**** member_link target: {:?}", handle_eh);
      let maybe_handle_and_hash = get_latest_typed_from_eh::<Handle>(handle_eh)?;
      let handle_and_hash = match maybe_handle_and_hash {
         Some(eh) => eh,
         None => continue,
      };
      let maybe_maybe_element = get(handle_and_hash.1.clone(), GetOptions::latest());
      let record = match maybe_maybe_element {
         Ok(Some(record)) => record,
         _ => continue,
      };
      let item = HandleItem {
         name: handle_and_hash.0.name.clone(),
         agentId: record.action().author().clone(),
         handle_eh: handle_and_hash.2.clone(),
      };
      handle_list.push(item);
    }
   trace!("get_all_handles() handle_map size: {}", handle_list.len());
   /// Done
   Ok(handle_list)
}
