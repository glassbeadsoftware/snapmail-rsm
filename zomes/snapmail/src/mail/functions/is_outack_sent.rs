use hdk::prelude::*;

use crate::{
   //link_kind::*,
   mail::entries::*,
   utils::*,
};

/// Get State of an OutAck
#[hdk_extern]
#[snapmail_api]
pub fn is_outack_sent(outack_hh: HeaderHash) -> ExternResult<bool> {
   debug!(" *** get_outack_state(): ");
   /// Get OutMail Details
   let maybe_details = get_details(outack_hh.clone(), GetOptions::latest())?;
   if maybe_details.is_none() {
      return error("No OutAck at given address");
   }
   let el_details = match maybe_details.unwrap() {
      Details::Element(details) => details,
      Details::Entry(_) => unreachable!("in get_outmail_state()"),
   };
   /// Get OutAck Entry to make sure its the right type
   let _outack: OutAck = get_typed_from_el(el_details.element.clone())
      ?;
   //.expect("Should be a OutMail entry");
   let outack_eh = el_details.element.header().entry_hash().expect("Should have an Entry");
   /// Grab info
   //let pending = get_links(outack_eh.clone(), LinkKind::Pending.as_tag_opt())?;
   //let sents = get_links(outack_eh.clone(), LinkKind::Sent.as_tag_opt())?;
   let links = get_links(outack_eh.clone(), None)?;
   return Ok(links.len() > 0);
}
