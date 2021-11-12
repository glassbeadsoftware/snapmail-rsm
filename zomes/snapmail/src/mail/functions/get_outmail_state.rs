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
   let pendings = get_links(outmail_eh.clone(), LinkKind::Pending.as_tag_opt())?;
   let receipts = get_links(outmail_eh.clone(), LinkKind::Receipt.as_tag_opt())?;

   debug!("  - recepients: {}", recepient_count);
   debug!("  -   pendings: {}", pendings.len());
   debug!("  -   receipts: {}", receipts.len());

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
