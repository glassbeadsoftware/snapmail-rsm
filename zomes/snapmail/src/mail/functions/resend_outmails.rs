use hdk::prelude::*;
use hdk::prelude::query::ChainQueryFilter;
use snapmail_model::*;
use zome_utils::*;

use crate::{
   mail::{
      get_outmail_delivery_state,
      send_committed_mail,
   },
};


/// Zome Function
/// Re-send mail to each recipient of each OutMail which has an Unsent status
/// Return list of OutMails for which we tried to deliver mail again
#[hdk_extern]
//#[snapmail_api]
fn resend_outmails(_: ()) -> ExternResult<Vec<ActionHash>> {
   /// Get all Create OutMail actions with query
   let outmail_query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(UnitEntryTypes::OutMail.try_into().unwrap());
   let maybe_outmails = query(outmail_query_args);
   if let Err(err) = maybe_outmails {
      error!("resend_outmails() outmail_result failed: {:?}", err);
      return Err(err);
   }
   let created_outmails: Vec<Record> = maybe_outmails.unwrap();
   debug!(" resend_outmails() outmails count = {}", created_outmails.len());

   /// Check for each OutMail
   let mut ahs = Vec::new();
   for outmail_el in created_outmails {
      let ah = outmail_el.action_address().to_owned();
      let eh = outmail_el.action().entry_hash().unwrap();
      let outmail: OutMail = get_typed_from_record(outmail_el.clone())?;
      let states = get_outmail_delivery_state(ah.clone())?;
      let unsent_recipients: Vec<AgentPubKey> = states.iter()
         .filter(|pair| pair.1 == &DeliveryState::Unsent)
         .map(|(recipient, _)| recipient)
         .cloned()
         .collect();
      if unsent_recipients.is_empty() {
         continue;
      }
      /// Some acks are missing ; send mail again
      ahs.push(ah);
      /// Send mail to each missing ack/pending
      let _ = send_committed_mail(eh, outmail, Some(unsent_recipients))?;
   }
   Ok(ahs)
}
