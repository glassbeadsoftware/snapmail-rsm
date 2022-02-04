use hdk::prelude::*;

use crate::{
   link_kind::*,
   mail::entries::*,
   utils::*,
};

/// Get State of an OutMail
#[hdk_extern]
#[snapmail_api]
pub fn get_outmail_state(outmail_hh: HeaderHash) -> ExternResult<OutMailState> {
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
   let recipient_count = outmail.recipients().len();
   let initial_pendings = get_links(outmail_eh.clone(), LinkKind::Pendings.as_tag_opt())?;
   let sents = get_links(outmail_eh.clone(), LinkKind::Sents.as_tag_opt())?;
   let receipts = get_links(outmail_eh.clone(), LinkKind::Receipt.as_tag_opt())?;

   debug!("  -   recipients: {}", recipient_count);
   debug!("  -     receipts: {}", receipts.len());
   debug!("  -     pendings: {}", initial_pendings.len());
   debug!("  -        sents: {}", sents.len());


   if receipts.len() == recipient_count {
      return Ok(OutMailState::AllAcknowledged);
   }

   if receipts.len() == initial_pendings.len() + sents.len() {
      return Ok(OutMailState::AllSent);
   }

   return Ok(OutMailState::Unsent);
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