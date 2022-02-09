use hdk::prelude::*;

use crate::{
   //link_kind::*,
   mail::entries::*,
   utils::*,
};
use crate::mail::get_confirmations;


/// Get State of an OutAck
#[hdk_extern]
#[snapmail_api]
pub fn is_outack_sent(outack_hh: HeaderHash) -> ExternResult<bool> {
   debug!(" *** get_outack_state(): ");
   /// Make sure of type
   let (outack_eh, _outack) = get_typed_from_hh::<OutAck>(outack_hh)?;
   /// Look for a confirmation
   let confirmations = get_confirmations(outack_eh)?;
   /// Return true if there is a delivery confirmation
   return Ok(confirmations.len() > 0)

}
