use hdk::prelude::*;

use strum::IntoEnumIterator;
use strum::AsStaticRef;

use crate::{
   link_kind::*,
};

/// Validation sub callback
pub fn validate_create_link(signed_create_link: SignedHashed<CreateLink>)
   -> ExternResult<ValidateCallbackResult>
{
   let create_link = signed_create_link.hashed.into_inner().0;
   let tag_str = String::from_utf8_lossy(&create_link.tag.0);
   trace!("*** `validate_create_link()` called: {}", tag_str);

   for link_kind in LinkKind::iter() {
      /// Get the entries linked
      let base =
         must_get_entry(create_link.base_address.clone().into())?
         .as_content()
         .to_owned();
      let target =
         must_get_entry(create_link.target_address.clone().into())?
            .as_content()
            .to_owned();
      /// Try validating static link kind
      if tag_str == link_kind.as_static() {
         return link_kind.validate_types(base, target, None);
      }
      /// Or try validating dynamic link kind
      let maybe_hash: ExternResult<AgentPubKey> = link_kind.unconcat_hash(&create_link.tag);
      //debug!("*** maybe_hash of {} = {:?}", link_kind.as_static(), maybe_hash);
      if let Ok(from) = maybe_hash {
         return link_kind.validate_types(base, target, Some(from));
      }
   }
   Ok(ValidateCallbackResult::Invalid(format!("Unknown tag: {}", tag_str).into()))
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
