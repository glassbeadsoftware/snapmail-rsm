use hdk::prelude::*;

use crate::{
   mail::{entries::*, utils::*}, utils::*
};

use std::collections::HashMap;
use crate::mail::get_inacks;

/// Get State of an OutMail
#[hdk_extern]
#[snapmail_api]
pub fn get_outmail_state(outmail_hh: HeaderHash) -> ExternResult<OutMailState> {
   debug!(" *** get_outmail_state() START - {}", outmail_hh);

   /// Check if deleted
   /// Get OutMail Details
   let maybe_details = get_details(outmail_hh.clone(), GetOptions::latest())?;
   if maybe_details.is_none() {
      return error("No OutMail at given address");
   }
   let el_details = match maybe_details.unwrap() {
      Details::Element(details) => details,
      Details::Entry(_) => unreachable!("in get_outmail_state()"),
   };
   /// Check if deleted
   if el_details.deletes.len() > 0 {
      return Ok(OutMailState::Deleted);
   }
   //debug!(" get_outmail_state() - el_details: {:?}", el_details);


   /// Get OutMail Entry
   let outmail: OutMail = get_typed_from_el(el_details.element.clone())?;
   //let outmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");

   /// Check if AllAcknowledged
   let recipient_count= outmail.recipients().len();
   let inacks = get_inacks(Some(outmail_hh.clone()))?;
   if recipient_count == inacks.len() {
      return Ok(OutMailState::AllAcknowledged);
   }
   /// Check all deliveries
   let map = get_outmail_delivery_state(outmail_hh.clone())?;
   let mut has_pending = false;
   /// OutMail is Unsent if at least one delivery is Unsent
   for state in map.values() {
      if state == &DeliveryState::Unsent {
         return Ok(OutMailState::Unsent);
      }
      if state == &DeliveryState::Pending {
         has_pending = true;
      }
   }
   if has_pending {
      return Ok(OutMailState::AllSent);
   }
   return Ok(OutMailState::AllReceived);
}


/// Return delivery state for each OutMail's recipient
#[hdk_extern]
#[snapmail_api]
pub fn get_outmail_delivery_state(outmail_hh: HeaderHash) -> ExternResult<HashMap<AgentPubKey, DeliveryState>> {
   debug!(" *** get_outmail_delivery_state(): ");
   /// Get OutMail Details
   let maybe_details = get_details(outmail_hh.clone(), GetOptions::latest())?;
   if maybe_details.is_none() {
      return error("No OutMail at given address");
   }
   let el_details = match maybe_details.unwrap() {
      Details::Element(details) => details,
      Details::Entry(_) => unreachable!("in get_outmail_state()"),
   };
   /// Get OutMail Entry
   let outmail: OutMail = get_typed_from_el(el_details.element.clone())?;
   let outmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");

   /// Determine state of delivery for each recipient and insert result in hashmap
   let mut map = HashMap::new();
   for recipient in outmail.recipients() {
      /// Check pending
      let confirmation_created = try_confirming_pending_has_been_received(outmail_eh.clone(), &recipient)?;
      if confirmation_created {
         map.insert(recipient.clone(), DeliveryState::Delivered);
      } else {
         let state = get_delivery_state(outmail_eh.clone(), &recipient)?;
         map.insert(recipient.clone(), state);
      }
   }

   /// Done
   Ok(map)
}
