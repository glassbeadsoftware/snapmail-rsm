use hdk::prelude::*;
use snapmail_model::*;
use zome_utils::*;

use crate::{
   handle::*,
   file::*,
   pub_enc_key::*,
};

///
pub fn validate_entry(entry: Entry, maybe_entry_type: Option<&EntryType>) -> ExternResult<ValidateCallbackResult> {
   /// Determine where to dispatch according to base
   let result = match entry {
      Entry::CounterSign(_data, _bytes) => Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into())), //validate_counter_sign_entry(data, bytes, maybe_package),
      Entry::Agent(agent_hash) => validate_agent_entry(agent_hash),
      Entry::CapClaim(claim) => validate_claim_entry(claim),
      Entry::CapGrant(grant) => validate_grant_entry(grant),
      Entry::App(entry_bytes) => {
         let app_type_id = if let EntryType::App(app_entry_type) = maybe_entry_type.unwrap() {
            app_entry_type.id()
         } else { unreachable!()};
         validate_app_entry(app_type_id, entry_bytes)
      },
   };
   /// Done
   trace!("*** validate_entry() result = {:?}", result);
   result
}

///
#[allow(unreachable_patterns)]
fn validate_app_entry(
   entry_type_id: EntryDefIndex,
   entry_bytes: AppEntryBytes,
) -> ExternResult<ValidateCallbackResult>
{
   trace!("*** validate_app_entry() callback called!");
   let sb = entry_bytes.into_sb();
   let entry_kind = EntryKind::from_index(&entry_type_id);

   match entry_kind {
      EntryKind::Handle => {
         let maybe_handle = Handle::try_from(sb.clone());
         if let Err(_err) = maybe_handle {
            return error("Failed to deserialize Handle");
         }
         let handle = maybe_handle.unwrap();
         let res = validate_handle_entry(handle);
         res
      },
      EntryKind::PubEncKey => {
         let maybe_key = PubEncKey::try_from(sb.clone());
         if let Err(_err) = maybe_key {
            return error("Failed to deserialize PubEncKey");
         }
         let key = maybe_key.unwrap();
         let res = validate_PubEncKey_entry(key);
         res
      },
      EntryKind::Path => {
         let maybe_content = PathEntry::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize PathEntry");
         }
         // FIXME validation
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::InMail => {
         let maybe_content = InMail::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize InMail");
         }
         // FIXME validation
         // return validate_inmail_entry(inmail, maybe_validation_package);
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::InAck => {
         let maybe_content = InAck::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize InAck");
         }
         // FIXME validation
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::PendingMail => {
         let maybe_content = PendingMail::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize PendingMail");
         }
         // FIXME validation
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::PendingAck => {
         let maybe_content = PendingAck::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize PendingAck");
         }
         // FIXME
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::DeliveryConfirmation => {
         let maybe_content = DeliveryConfirmation::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize DeliveryConfirmation");
         }
         // FIXME
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::OutMail => {
         let maybe_content = OutMail::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize OutMail");
         }
         // FIXME
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::OutAck => {
         let maybe_content = OutAck::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize OutAck");
         }
         // FIXME
         Ok(ValidateCallbackResult::Valid)
      },
      EntryKind::FileManifest => {
         let maybe_content = FileManifest::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize FileManifest");
         }
         let manifest = maybe_content.unwrap();
         let res = validate_file(manifest);
         res
      },
      EntryKind::FileChunk => {
         let maybe_content = FileChunk::try_from(sb.clone());
         if let Err(_err) = maybe_content {
            return error("Failed to deserialize FileChunk");
         }
         let chunk = maybe_content.unwrap();
         let res = validate_chunk(chunk);
         res
      },
      /// Add entry validation per type here
      /// ..

      /// Unreachable but doesnt compile without it. Yay Rust
      _ => Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into())),
   }
}

///
fn validate_agent_entry(
   _agent_hash: AgentPubKey,
) -> ExternResult<ValidateCallbackResult>
{
   trace!("*** validate_agent_entry() called!");
   // FIXME
   Ok(ValidateCallbackResult::Valid)
}

///
fn validate_claim_entry(
   _claim: CapClaim,
) -> ExternResult<ValidateCallbackResult>
{
   trace!("*** validate_claim_entry() called!");
   // FIXME validation
   Ok(ValidateCallbackResult::Valid)
   //Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
}

///
fn validate_grant_entry(
   _grant: ZomeCallCapGrant,
) -> ExternResult<ValidateCallbackResult>
{
   trace!("*** validate_grant_entry() called!");
   // FIXME validation
   Ok(ValidateCallbackResult::Valid)
   //Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
}


//
//fn validate_counter_sign_entry(
//    _data: Box<CounterSigningSessionData, Global>,
//    _bytes: AppEntryBytes,
//    _maybe_validation_package: Option<ValidationPackage>,
//) -> ExternResult<ValidateCallbackResult>
//{
//    trace!("*** validate_counter_sign_entry() called!");
//    // FIXME validation
//    //Ok(ValidateCallbackResult::Valid)
//    Ok(ValidateCallbackResult::Invalid("Validation failed: Not authorized".into()))
//}