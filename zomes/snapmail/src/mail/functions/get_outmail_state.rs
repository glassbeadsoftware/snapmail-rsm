use hdk::prelude::*;

use crate::{
   mail::{entries::*, utils::*},
   utils::*,
};

use std::collections::HashMap;
use crate::mail::get_inacks;

/// Get State of an OutMail
#[hdk_extern]
#[snapmail_api]
pub fn get_outmail_state(outmail_hh: HeaderHash) -> ExternResult<OutMailState> {
   debug!(" *** get_outmail_state() START - {}", outmail_hh);
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
   let outmail: OutMail = get_typed_from_el(el_details.element.clone())
      ?;
   //.expect("Should be a OutMail entry");
   let outmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");
   /// Grab info
   let recipient_count= outmail.recipients().len();
   let inacks = get_inacks(Some(outmail_hh))?;
   let confirmations = get_confirmations(outmail_eh.to_owned())?;
   //let pendings = get_links(outmail_eh.clone(), LinkKind::Pendings.as_tag_opt())?;

   debug!("  -    recipients: {}", recipient_count);
   debug!("  -     delivered: {}", confirmations.len());
   debug!("  -         acked: {}", inacks.len());
   //debug!("  -      pendings: {}", pendings.len());

   if recipient_count == inacks.len() {
      return Ok(OutMailState::AllAcknowledged);
   }

   if recipient_count == confirmations.len() {
      return Ok(OutMailState::AllSent);
   }

   return Ok(OutMailState::Unsent);
}


/// Get full state of an OutMail
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
   let outmail: OutMail = get_typed_from_el(el_details.element.clone())
      ?;
   let outmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");

   let inacks = get_inacks(Some(outmail_hh))?;
   let acked_recipients: Vec<AgentPubKey> = inacks.iter().map(|inack| inack.from.to_owned()).collect();

   let confirmations = get_confirmations(outmail_eh.to_owned())?;
   let confirmed_recipients: Vec<AgentPubKey> = confirmations.iter().map(|x| x.destination.clone()).collect();

   /// Determine state of delivery for each recipient and insert result in hashmap
   let mut map = HashMap::new();
   for recipient in outmail.recipients() {
      let mut state = DeliveryState::Unsent;
      if acked_recipients.contains(&recipient) {
         state = DeliveryState::Acknowledged
      }
      if confirmed_recipients.contains(&recipient) {
         state = DeliveryState::Sent
      }
      map.insert(recipient, state);
   }
   /// Done
   Ok(map)
}


// /// Delete Pendings links from outmail to `to` agent
// fn delete_pendings_link(outmail_eh: &EntryHash, to: &AgentPubKey) -> ExternResult<HeaderHash> {
//    let pendings_links_result = get_links(
//       outmail_eh.clone(),
//       //None,
//       Some(LinkKind::Pendings.concat_hash(to)),
//    )?;
//    debug!("pendings_links_result: {:?}", pendings_links_result);
//    if pendings_links_result.len() != 1 {
//       return error("Pendings link not found");
//    }
//    let res = delete_link(pendings_links_result[0].create_link_hash.clone());
//    res
// }