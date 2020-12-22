use hdk3::prelude::*;

use strum::IntoEnumIterator;
use strum::AsStaticRef;

use crate::{
   link_kind::*,
};

/// Zome Callback
#[hdk_extern]
fn validate_create_link(candidat: ValidateCreateLinkData)
   -> ExternResult<ValidateLinkCallbackResult>
{
   let tag_str = String::from_utf8_lossy(&candidat.link_add.tag.0);
   debug!("*** `validate_create_link()` callback called: {}", tag_str);

   for link_kind in LinkKind::iter() {
      /// Try validating static link kind
      if tag_str == link_kind.as_static() {
         return link_kind.validate_types(candidat, None);
      }
      /// Or try validating dynamic link kind
      let maybe_hash: ExternResult<AgentPubKey> = link_kind.unconcat_hash(&candidat.link_add.tag);
      //debug!("*** maybe_hash of {} = {:?}", link_kind.as_static(), maybe_hash);
      if let Ok(from) = maybe_hash {
         return link_kind.validate_types(candidat, Some(from));
      }
   }
   Ok(ValidateLinkCallbackResult::Invalid(format!("Unknown tag: {}", tag_str).into()))
}


// TODO: Make sure there is only one handle per Agent
// ///
// fn validate_handle_link(
//    agent_hash: AgentPubKey,
//    submission: ValidateCreateLinkData,
// ) -> ExternResult<ValidateLinkCallbackResult>
// {
//    debug!("*** validate_handle_link() START");
//    assert!(submission.link_add.tag == LinkKind::Handle.as_tag());
//
//    // TODO: Only one handle per agent
//    //let my_agent_address = agent_info!()?.agent_latest_pubkey;
//    //let maybe_current_handle_element = get_handle_element(my_agent_address.clone());
//    let maybe_current_handle: ExternResult<Handle> = try_from_entry(submission.target);
//    if maybe_current_handle.is_err() {
//       return Ok(ValidateLinkCallbackResult::Invalid("Not linked to a Handle Entry".into()));
//    }
//    let _handle_entry = maybe_current_handle.unwrap();
//    /// Can only set handle for self
//    if submission.link_add.author != agent_hash {
//       return Ok(ValidateLinkCallbackResult::Invalid("Not self authored".into()));
//    }
//    // TODO: Check if new Handle is different from currrent
//    Ok(ValidateLinkCallbackResult::Valid);
// }

/// Zome Callback
/// TODO: Should not be valide by default
#[hdk_extern]
fn validate_delete_link(_delete_link_submission: ValidateDeleteLinkData)
   -> ExternResult<ValidateLinkCallbackResult>
{
   debug!("*** validate_delete_link() callback called!");
   //let _delete_link = validate_delete_link.delete_link;
   // Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
   Ok(ValidateLinkCallbackResult::Valid)
}