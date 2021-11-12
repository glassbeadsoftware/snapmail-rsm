use hdk::prelude::*;

use crate::{
   link_kind::*,
   mail::entries::*,
   utils::*,
};

/// Get State of an OutMail
#[hdk_extern]
#[snapmail_api]
pub(crate) fn get_outmail_state(outmail_hh: HeaderHash) -> ExternResult<OutMailState> {
   debug!(" *** get_outmail_state(): ");
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
   /// Get OutMail Entry
   let outmail: OutMail = get_typed_from_el(el_details.element.clone())
      ?;
   //.expect("Should be a OutMail entry");
   let outmail_eh = el_details.element.header().entry_hash().expect("Should have an Entry");
   /// Grab info
   let recepient_count = outmail.bcc.len() + outmail.mail.to.len() + outmail.mail.cc.len();
   let initial_pendings = get_links(outmail_eh.clone(), LinkKind::Pendings.as_tag_opt())?;
   let receipts = get_links(outmail_eh.clone(), LinkKind::Receipt.as_tag_opt())?;

   debug!("  -   recepients: {}", recepient_count);
   debug!("  -     receipts: {}", receipts.len());
   debug!("  - ini-pendings: {}", initial_pendings.len());

   // Check each pending if it has been deleted
   let mut pendings= Vec::new();
   for pending_link in initial_pendings {
      let el = get(pending_link.target.clone(), GetOptions::latest())?
         .ok_or(WasmError::Guest(String::from("details not found")))?;
      let hh: HeaderHash = el.header_address().clone();
      let details = get_details(hh, GetOptions::latest())?
         .ok_or(WasmError::Guest(String::from("details not found")))?;
      //debug!("  -      details: {:?}", details);
      let deletes = match details {
         Details::Element(details) => details.deletes,
         Details::Entry(_) => unreachable!("in get_outmail_state(). Must be a Element."),
      };
      trace!("  -      deletes: {:?}", deletes);
      /// Check if deleted
      if deletes.len() > 0 {
         delete_link(pending_link.create_link_hash)?;
         trace!(" *** Deleted link to: {:?}", pending_link.tag);
         continue;
      }
      pendings.push(pending_link.clone());
   }
   debug!("  -     pendings: {}", pendings.len());

   /// Determine state
   if pendings.len() == recepient_count {
      return Ok(OutMailState::Pending);
   }
   if pendings.len() == 0 {
      if receipts.len() == 0 {
         return Ok(OutMailState::Arrived_NoAcknowledgement);
      }
      if receipts.len() == recepient_count {
         return Ok(OutMailState::FullyAcknowledged);
      }
      return Ok(OutMailState::Arrived_PartiallyAcknowledged);
   }
   if receipts.len() == 0 {
      return Ok(OutMailState::PartiallyArrived_NoAcknowledgement);
   }
   return Ok(OutMailState::PartiallyArrived_PartiallyAcknowledged);
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