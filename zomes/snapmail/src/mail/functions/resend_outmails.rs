use hdk::prelude::*;
use hdk::prelude::query::ChainQueryFilter;
use zome_utils::*;

use crate::{
   entry_kind::*,
   mail::entries::*,
   mail::{
      get_outmail_delivery_state,
      send_committed_mail,
   },
};


/// Zome Function
/// Re-send mail to each recipient of each OutMail which has an Unsent status
/// Return list of OutMails for which we tried to deliver mail again
#[hdk_extern]
#[snapmail_api]
fn resend_outmails(_: ()) -> ExternResult<Vec<ActionHash>> {
   /// Get all Create OutMail headers with query
   let outmail_query_args = ChainQueryFilter::default()
      .include_entries(true)
      .header_type(HeaderType::Create)
      .entry_type(EntryKind::OutMail.as_type());
   let maybe_outmails = query(outmail_query_args);
   if let Err(err) = maybe_outmails {
      error!("resend_outmails() outmail_result failed: {:?}", err);
      return Err(err);
   }
   let created_outmails: Vec<Element> = maybe_outmails.unwrap();
   debug!(" resend_outmails() outmails count = {}", created_outmails.len());

   /// Check for each OutMail
   let mut hhs = Vec::new();
   for outmail_el in created_outmails {
      let hh = outmail_el.header_address().to_owned();
      let eh = outmail_el.header().entry_hash().unwrap();
      let outmail: OutMail = get_typed_from_el(outmail_el.clone())?;
      let states = get_outmail_delivery_state(hh.clone())?;
      let unsent_recipients: Vec<AgentPubKey> = states.iter()
         .filter(|pair| pair.1 == &DeliveryState::Unsent)
         .map(|(recipient, _)| recipient)
         .cloned()
         .collect();
      if unsent_recipients.is_empty() {
         continue;
      }
      /// Some acks are missing ; send mail again
      hhs.push(hh);
      /// Send mail to each missing ack/pending
      let _ = send_committed_mail(eh, outmail, Some(unsent_recipients))?;
   }
   Ok(hhs)
}