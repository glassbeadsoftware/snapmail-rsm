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
   debug!("*** validate_create_link() called: {}", tag_str);

   for link_kind in LinkKind::iter() {
      if tag_str == link_kind.as_static() {
         return link_kind.validate_types(candidat, None);
      }
      let maybe_hash: ExternResult<AgentPubKey> = link_kind.unconcat_hash(&candidat.link_add.tag);
      //debug!("*** maybe_hash of {} = {:?}", link_kind.as_static(), maybe_hash);

      if let Ok(from) = maybe_hash {
         return link_kind.validate_types(candidat, Some(from));
      }
   }
   Ok(ValidateLinkCallbackResult::Invalid(format!("Unknown tag: {}", tag_str).into()))
}

/// Zome Callback
#[hdk_extern]
fn validate_delete_link(_delete_link_submission: ValidateDeleteLinkData)
   -> ExternResult<ValidateLinkCallbackResult>
{
   debug!("*** validate_delete_link() called!");
   //let _delete_link = validate_delete_link.delete_link;

   // FIXME: Should not be valide by default
   // Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))

   Ok(ValidateLinkCallbackResult::Valid)
}