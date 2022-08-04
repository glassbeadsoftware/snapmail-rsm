use hdk::prelude::*;
use snapmail_model::*;
use zome_utils::*;

use crate::{
    mail::{utils::get_confirmations, get_outacks},
};


/// Zome function
#[hdk_extern]
#[snapmail_api]
pub fn has_ack_been_delivered(inmail_ah: ActionHash) -> ExternResult<bool> {
   /// Make sure its an inmail
   let inmail_eh = get_eh(inmail_ah.clone())?;
   let _ = get_typed_from_eh::<InMail>(inmail_eh)?;
   /// Get inmail's outack
   let inacks = get_outacks(Some(inmail_ah))?;
   if inacks.is_empty() {
      return Ok(false)
   }
   let inack_eh = hash_entry(inacks[0].clone())?;
   /// Check for OutAck's confirmation
   let confirmations = get_confirmations(inack_eh)?;
   return Ok(!confirmations.is_empty());
}
