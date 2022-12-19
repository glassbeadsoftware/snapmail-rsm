use hdk::prelude::*;
use hdk::prelude::query::ChainQueryFilter;
use snapmail_model::*;
use zome_utils::*;

use crate::{
   mail::{
      get_delivery_state,
      send_committed_ack,
   },
};


/// Zome Function
/// Re-send outack which has an Unsent Delivery status
/// Return list of OutAcks which we tried to deliver again
#[hdk_extern]
//#[snapmail_api]
fn resend_outacks(_: ()) -> ExternResult<Vec<ActionHash>> {
   let query_args = ChainQueryFilter::default()
      .include_entries(true)
      .action_type(ActionType::Create)
      .entry_type(UnitEntryTypes::OutAck.try_into().unwrap());
   let maybe_outacks = query(query_args);
   if let Err(err) = maybe_outacks {
      error!("resend_outacks() query failed: {:?}", err);
      return Err(err);
   }
   let created_outacks: Vec<Record> = maybe_outacks.unwrap();
   debug!(" resend_outacks() outacks len = {}", created_outacks.len());
   let mut ahs = Vec::new();
   for outack_el in created_outacks {
      let ah = outack_el.action_address().to_owned();
      let eh = outack_el.action().entry_hash().unwrap();
      let outack: OutAck = get_typed_from_record(outack_el.clone())?;
      let inmail: InMail = get_typed_from_eh(outack.inmail_eh.clone())?;
      let state = get_delivery_state(eh.to_owned(), &inmail.from)?;
      if state != DeliveryState::Unsent {
         continue;
      }
      /// Some acks are missing ; send mail again
      ahs.push(ah);
      /// Send mail to each missing ack/pending
      let _res = send_committed_ack(eh, outack);
   }
   Ok(ahs)
}
