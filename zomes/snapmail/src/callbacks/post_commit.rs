use hdk::prelude::*;
// use hdk::prelude::element::ElementEntry;

use crate::{
   handle::*,
   //chunk::*,
   file::*,
   mail::entries::*,
   entry_kind::*,
   utils::*,
};

/// Zome Callback
#[hdk_extern]
fn post_commit(signedHeaderList: Vec<SignedHeaderHashed>) -> ExternResult<PostCommitCallbackResult> {
   //debug!("post_commit() called: {:?}", hhList);
   debug!("post_commit() called");
   for signedHeader in signedHeaderList {
      //debug!(" - {:?}", signedHeader.header().entry_type());
      let header = signedHeader.header();
      let hash = signedHeader.as_hash().get_raw_39();
      let hash64 = format!("u{}", base64::encode_config(hash, base64::URL_SAFE_NO_PAD));
      debug!(" - {} ({:?})", hash64, signedHeader.header().entry_type());

      if header.entry_type().is_none() {
         continue;
      }
      let (entry_hash, entry_type) = header.entry_data().unwrap();

      match entry_type {
         EntryType::AgentPubKey => {},
         EntryType::CapClaim => {},
         EntryType::CapGrant => {},
         EntryType::App(app_type) => { return post_commit_app(entry_hash.clone(), app_type.clone()); },
      }
   }
   Ok(PostCommitCallbackResult::Success)
}


fn post_commit_app(eh: EntryHash, app_type: AppEntryType) -> ExternResult<PostCommitCallbackResult> {
   let entry_kind = EntryKind::from_index(&app_type.id());

   match entry_kind {
      EntryKind::Handle => {},
      EntryKind::PubEncKey => {},
      EntryKind::Path => {},
      EntryKind::InMail => {
         let in_mail = get_typed_from_eh::<InMail>(eh)?;
      },
      EntryKind::InAck => {
         let in_ack = get_typed_from_eh::<InAck>(eh)?;
      },
      EntryKind::PendingMail => {
         let pending_mail = get_typed_from_eh::<PendingMail>(eh)?;
      },
      EntryKind::PendingAck => {
         let pending_ack = get_typed_from_eh::<PendingAck>(eh)?;
      },
      EntryKind::OutMail => {},
      EntryKind::OutAck => {},
      EntryKind::FileManifest => {
         let manifest = get_typed_from_eh::<FileManifest>(eh)?;
      },
      EntryKind::FileChunk => {
         let chunk = get_typed_from_eh::<FileChunk>(eh)?;
      },
      /// Add type handling here
      /// ..

      ///
      _ => unreachable!(),
   }
   // Done
   Ok(PostCommitCallbackResult::Success)
}

