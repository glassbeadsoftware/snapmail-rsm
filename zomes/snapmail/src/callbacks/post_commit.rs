use hdk::prelude::*;

use crate::{
   file::*,
   mail::entries::*,
   mail::functions::*,
   entry_kind::*,
   utils::*,
};
use crate::strum::AsStaticRef;

/// Zome Callback
#[hdk_extern(infallible)]
fn post_commit(signedHeaderList: Vec<SignedHeaderHashed>) {
   //debug!("post_commit() called: {:?}", hhList);
   debug!("post_commit() called");
   for signedHeader in signedHeaderList {
      //debug!(" - {:?}", signedHeader.header().entry_type());
      let header = signedHeader.header();

      //let hash = signedHeader.as_hash().get_raw_39();
      //let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      // debug!(" - {} ({:?})", hash64, signedHeader.header().entry_type());

      if header.entry_type().is_none() {
         continue;
      }
      let (entry_hash, entry_type) = header.entry_data().unwrap();

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


fn post_commit_app(eh: EntryHash, app_type: AppEntryType) -> ExternResult<()>{
   let entry_kind = EntryKind::from_index(&app_type.id());
   debug!(" - {} ({})",  entry_kind.as_static(), eh);
   match entry_kind {
      EntryKind::Handle => {},
      EntryKind::PubEncKey => {},
      EntryKind::Path => {},
      EntryKind::InMail => {
         let _inmail = get_typed_from_eh::<InMail>(eh)?;
      },
      EntryKind::InAck => {
         let _inack = get_typed_from_eh::<InAck>(eh)?;
      },
      EntryKind::PendingMail => {
         let _pending_mail = get_typed_from_eh::<PendingMail>(eh)?;
      },
      EntryKind::PendingAck => {
         let _pending_ack = get_typed_from_eh::<PendingAck>(eh)?;
      },
      EntryKind::OutMail => {
         let outmail = get_typed_from_eh::<OutMail>(eh.clone())?;
         send_committed_mail(&eh, outmail)?;
      },
      EntryKind::OutAck => {
         let outack = get_typed_from_eh::<OutAck>(eh.clone())?;
         send_committed_ack(&eh, outack)?;
      },
      EntryKind::FileManifest => {
         let _manifest = get_typed_from_eh::<FileManifest>(eh)?;
      },
      EntryKind::FileChunk => {
         let _chunk = get_typed_from_eh::<FileChunk>(eh)?;
      }
      // Add type handling here
      // ..
   }
   // Done
   Ok(())
}

