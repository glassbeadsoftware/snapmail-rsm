use hdk::prelude::*;
//use zome_utils::*;
//use snapmail_model::*;

use crate::{
   //file::*,
   mail::functions::*,
};

use snapmail_model::*;

//use crate::strum::AsStaticRef;


/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   //debug!("post_commit() called: {:?}", ahList);
   debug!("post_commit() called. Len = {}", signedActionList.len());
   for signedAction in signedActionList {
      //debug!(" - {:?}", signedAction.action().entry_type());
      let action = signedAction.action();

      //let hash = signedAction.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedAction.action().entry_type());

      if action.entry_type().is_none() {
         continue;
      }
      let (entry_hash, entry_type) = action.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => {
            let res = post_commit_app(entry_hash.clone(), app_type.clone());
            if let Err(e) = res {
               error!("post_commit() error: {:?}", e);
            }
         },
      }
   }
}


///
fn post_commit_app(eh: EntryHash, _app_type: AppEntryDef) -> ExternResult<()> {
   debug!("post_commit_app() called");
   if let Ok(outmail) = zome_utils::get_typed_from_eh::<OutMail>(eh.clone()) {
      send_committed_mail(&eh, outmail, None)?;
   }
   if let Ok(outack) = zome_utils::get_typed_from_eh::<OutAck>(eh.clone()) {
      send_committed_ack(&eh, outack)?;
   }
   // Done
   Ok(())
}

