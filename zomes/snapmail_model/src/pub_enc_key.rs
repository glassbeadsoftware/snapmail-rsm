use hdi::prelude::*;
use tracing::*;

//use zome_utils::*;

//use crate::link_kind::*;

/// Entry representing the Public Encryption Key of an Agent
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct PubEncKey {
   pub value: X25519PubKey,
}

impl PubEncKey {
   pub fn new(value: X25519PubKey) -> Self {
      Self {
         value,
      }
   }
}



// -- VALIDATION -- //

///
pub fn validate_PubEncKey_entry(_: PubEncKey) -> ExternResult<ValidateCallbackResult> {
   trace!("*** validate_PubEncKey_entry() called!");
   Ok(ValidateCallbackResult::Valid)
}

// #[hdk_extern]
// fn validate_PubEncKey_delete(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
//    Ok(ValidateCallbackResult::Invalid("Agent must always have a Handle".into()))
// }
