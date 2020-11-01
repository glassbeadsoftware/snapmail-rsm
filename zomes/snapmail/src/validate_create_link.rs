use hdk3::prelude::*;

use crate::{
   handle::*,
   utils::*,
   link_kind,
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
    debug!("*** validate_create_link_from_app() called! {:?}", submission.link_add).ok();

   /// Check Members
   if submission.link_add.tag == link_tag(link_kind::Members) {
      // FIXME
      return Ok(ValidateLinkCallbackResult::Valid);
   }

    /// Check for Handle
    let maybe_handle = Handle::try_from(base_entry_bytes.clone().into_sb());
    if maybe_handle.is_ok() {
        let handle = maybe_handle.unwrap();
        return validate_create_link_from_handle(handle, submission);
    }
    // /// Check for Path
    // let maybe_path = Path::try_from(base_entry_bytes.clone().into_sb());
    // if maybe_path.is_ok() {
    //     let _path = maybe_path.unwrap();
    //     return Ok(ValidateLinkCallbackResult::Valid);
    // }

    /// Add link per app entry here ...
    /// Done
    Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
}

///
fn validate_create_link_from_agent(
    agent_hash: AgentPubKey,
    submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_agent() called!").ok();
    /// -- Check if its a Handle link
    if submission.link_add.tag == link_tag(link_kind::Handle) {
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
    Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
}

///
fn validate_create_link_from_claim(
    _claim: CapClaim,
    _submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_claim() called!").ok();
    // FIXME
    Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
}

///
fn validate_create_link_from_grant(
    _grant: ZomeCallCapGrant,
    _submission: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult>
{
    debug!("*** validate_create_link_from_grant() called!").ok();
    // FIXME
    Ok(ValidateLinkCallbackResult::Invalid("Not authorized".into()))
}