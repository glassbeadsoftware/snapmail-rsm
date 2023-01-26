use hdk::prelude::*;
//use zome_utils::*;
//use snapmail_model::*;

use crate::{
   //file::*,
   mail::functions::*,
   signal_protocol::*,
};

use snapmail_model::*;

//use crate::strum::AsStaticRef;


/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedActionList: Vec<SignedActionHashed>) {
   //debug!("post_commit() called: {:?}", ahList);
   debug!("post_commit() len = {}", signedActionList.len());
   for sah in signedActionList {
      //debug!(" - {:?}", signedAction.action().entry_type());
      let action = sah.action().clone();

      //let hash = signedAction.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedAction.action().entry_type());

      if action.entry_type().is_none() {
         continue;
      }
      let (eh, entry_type) = action.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => {
            let res = post_commit_app(sah, eh.clone(), app_type.clone());
            if let Err(e) = res {
               error!("post_commit() error: {:?}", e);
            }
         },
      }
   }
}


///
fn post_commit_app(sah: SignedActionHashed, eh: EntryHash, _app_type: AppEntryDef) -> ExternResult<()> {
   debug!("post_commit_app() eh = {}", eh);
   if let Ok(outmail) = zome_utils::get_typed_from_eh::<OutMail>(eh.clone()) {
      send_committed_mail(&eh, outmail, None)?;
   }
   if let Ok(outack) = zome_utils::get_typed_from_eh::<OutAck>(eh.clone()) {
      send_committed_ack(&eh, outack)?;
   }
   /// Emit signal when InMail committed
   if let Ok(inmail) = zome_utils::get_typed_from_eh::<InMail>(eh.clone()) {
      let item = MailItem {
         ah: sah.hashed.as_hash().to_owned(),
         author: sah.hashed.content.author().to_owned(),
         mail: inmail.mail.clone(),
         state: MailState::In(InMailState::Unacknowledged),
         bcc: Vec::new(),
         date: zome_utils::now() as i64, // FIXME
         reply: None,
         reply_of: None,
         status: None,
      };
      //debug!("post_commit_app().ReceivedMail: '{}'", item.mail.subject);
      let res = emit_signal(&SignalProtocol::ReceivedMail(item));
      if let Err(err) = res {
          error!("Emit 'ReceivedMail' signal failed: {}", err);
      }
   }
   // Done
   Ok(())
}

