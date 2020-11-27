use hdk3::prelude::*;

use crate::{
   handle::*,
   utils::*,
   link_kind::*,
};

#[hdk_extern]
fn validate_create_link(submission: ValidateCreateLinkData) -> ExternResult<ValidateLinkCallbackResult> {
   debug!("*** validate_create_link() called!").ok();

   let base_entry = submission.base.clone();

   // Determine where to dispatch according to base
   match base_entry {
      Entry::Agent(agent_hash) => validate_create_link_from_agent(agent_hash, submission),
      Entry::CapClaim(claim) => validate_create_link_from_claim(claim, submission),
      Entry::CapGrant(grant) => validate_create_link_from_grant(grant, submission),
      Entry::App(entry_bytes) => validate_create_link_from_app(entry_bytes, submission),
   }
}

///
fn validate_create_link_from_app(
    base_entry_bytes: AppEntryBytes,
    submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
   let tag_str = String::from_utf8_lossy(&submission.link_add.tag.0);
    debug!("*** validate_create_link_from_app() called! {:?} \n -- tag = {}",
           submission.link_add, tag_str).ok();

   // /// Check for Handle
   // let maybe_handle = Handle::try_from(base_entry_bytes.clone().into_sb());
   // if maybe_handle.is_ok() {
   //    let handle = maybe_handle.unwrap();
   //    return validate_create_link_from_handle(handle, submission);
   // }

   /// Check Members
   if submission.link_add.tag == LinkKind::Members.as_tag() {
      // FIXME
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check Acknolwedgment
   if submission.link_add.tag == LinkKind::Acknowledgment.as_tag() {
      // TODO: check from InMail and unicity
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check Pending
   if submission.link_add.tag == LinkKind::Pending.as_tag() {
      // TODO
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check Receipt
   let maybe_hash: ExternResult<AgentPubKey> = LinkKind::Receipt.unconcat_hash(&submission.link_add.tag);
   if let Ok(_from) = maybe_hash {
      // TODO: check from OutMail and unicity per recepient
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check AckInbox
   let maybe_hash: ExternResult<AgentPubKey> = LinkKind::AckInbox.unconcat_hash(&submission.link_add.tag);
   if let Ok(_from) = maybe_hash {
      // TODO
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check MailInbox
   let maybe_hash: ExternResult<AgentPubKey> = LinkKind::MailInbox.unconcat_hash(&submission.link_add.tag);
   if let Ok(_from) = maybe_hash {
      // TODO
      return Ok(ValidateLinkCallbackResult::Valid);
   }

   /// Check Pendings
   let maybe_hash: ExternResult<AgentPubKey> = LinkKind::Pendings.unconcat_hash(&submission.link_add.tag);
   if let Ok(_from) = maybe_hash {
      // TODO
      return Ok(ValidateLinkCallbackResult::Valid);
   }

    // /// Check for Path
    // let maybe_path = Path::try_from(base_entry_bytes.clone().into_sb());
    // if maybe_path.is_ok() {
    //     let _path = maybe_path.unwrap();
    //     return Ok(ValidateLinkCallbackResult::Valid);
    // }

    /// Add link per app entry here
    /// ...

   /// Done
   debug!("*** validate_create_link_from_app() DONE").ok();
   // FIXME should not be valid by default
   //Ok(ValidateLinkCallbackResult::Valid)
   Ok(ValidateLinkCallbackResult::Invalid(format!("Not authorized: {}", tag_str).into()))
}

///
fn validate_create_link_from_agent(
    agent_hash: AgentPubKey,
    submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_agent() START").ok();
    /// -- Check if its a Handle link
    if submission.link_add.tag == LinkKind::Handle.as_tag() {
       // FIXME: Only one handle per agent
       //let my_agent_address = agent_info!()?.agent_latest_pubkey;
       //let maybe_current_handle_element = get_handle_element(my_agent_address.clone());
       let maybe_current_handle: ExternResult<Handle> = try_from_entry(submission.target);
       if maybe_current_handle.is_err() {
          return Ok(ValidateLinkCallbackResult::Invalid("Not linked to a Handle Entry".into()));
       }
       let _handle_entry = maybe_current_handle.unwrap();
        /// Can only set handle for self
        if submission.link_add.author != agent_hash {
            return Ok(ValidateLinkCallbackResult::Invalid("Not self authored".into()));
        }
        // FIXME: Check if new Handle is different from currrent
        return  Ok(ValidateLinkCallbackResult::Valid);
    }
    /// Done
    // FIXME: should not be valid by default
    debug!("*** validate_create_link_from_agent() DONE").ok();
    //Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
    Ok(ValidateLinkCallbackResult::Valid)
}

///
fn validate_create_link_from_claim(
    _claim: CapClaim,
    _submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_claim() called!").ok();
    // FIXME
    Ok(ValidateLinkCallbackResult::Invalid("Claim not authorized".into()))
}

///
fn validate_create_link_from_grant(
    _grant: ZomeCallCapGrant,
    _submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_grant() called!").ok();
    // FIXME
    Ok(ValidateLinkCallbackResult::Invalid("Grant not authorized".into()))
}