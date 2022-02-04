use hdk::prelude::*;

use crate::{
   handle::Handle,
   handle::utils::get_members,
   utils::get_typed_and_author,
};

/// Get all agentIds that have a certain handle
/// Return [AgentId]
#[hdk_extern]
#[snapmail_api]
pub fn find_agent(handle: String) -> ExternResult<Vec<AgentPubKey>> {
   let member_links = get_members()?;
   let mut agent_list = Vec::new();
   /// Find handle entry whose author is agentId
   for member_link in member_links {

      let res = get_typed_and_author::<Handle>(&member_link.target);
      if let Err(err) = res {
         warn!("Retrieving Handle from DHT failed: {}", err);
         continue;
      }
      let (author, handle_entry) = res.unwrap();
      if handle_entry.name == handle {
          agent_list.push(author);
      }
   }
   /// Done
   debug!("agent_list size: {}", agent_list.len());
   return Ok(agent_list)
}
