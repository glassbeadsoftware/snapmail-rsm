use hdk3::prelude::*;

use std::str::FromStr;

use strum::AsStaticRef;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
   playground::*,
   handle::*,
   mail::entries::*,
   chunk::*,
   utils::*,
};

/// !! Keep Order with EntryKind !!
entry_defs![
   /// --  Handle
   Handle::entry_def(),
   /// -- Mail
   InMail::entry_def(),
   OutMail::entry_def(),
   OutAck::entry_def(),
   InAck::entry_def(),
   PendingAck::entry_def(),
   PendingMail::entry_def(),
   /// -- Other
   Path::entry_def(),
   Post::entry_def(),
   FileChunk::entry_def()
];

/// Listing all Link kinds for this DNA
#[derive(AsStaticStr, EnumIter, EnumProperty, Clone, Debug, Serialize, Deserialize, SerializedBytes, PartialEq)]
pub enum EntryKind {
   /// !! Keep Order with entry_defs!() !!
   Handle,
   InMail,
   OutMail,
   OutAck,
   InAck,
   PendingMail,
   PendingAck,
   //
   Path,
   // Post,
   FileChunk,
   FileManifest,
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
      match input {
         "Handle"  => Ok(EntryKind::Handle),
         "InMail"  => Ok(EntryKind::InMail),
         "InAck"  => Ok(EntryKind::InAck),
         "PendingMail" => Ok(EntryKind::PendingMail),
         "PendingAck"  => Ok(EntryKind::PendingAck),
         "OutMail"  => Ok(EntryKind::OutMail),
         "OutAck"  => Ok(EntryKind::OutAck),
         "FileChunk" => Ok(EntryKind::FileChunk),
         "FileManifest" => Ok(EntryKind::FileManifest),
         "Path" => Ok(EntryKind::Path),
         //
         "AppPubKey" => {
            debug!("EntryKind::from_str() FAILED on AppPubKey").ok();
            Err(())
         },
         "App" => {
            debug!("EntryKind::from_str() FAILED on App").ok();
            Err(())
         },
         //
         _ => {
            debug!("EntryKind::from_str() FAILED on input: {}", input).ok();
            Err(())
         },
      }
   }
}


impl EntryKind {

   /// Hackish but works
   pub fn from_index(index: &EntryDefIndex) -> Self {
      for entry_kind in EntryKind::iter() {
         if entry_kind.index() == index.index() as u8 {
            return entry_kind;
         }
      }
      unreachable!()
   }

   ///
   pub fn from_entry_bytes(entry_bytes: AppEntryBytes) -> Self {

      let sb = entry_bytes.into_sb();

      let maybe_handle = Handle::try_from(sb.clone());
      if maybe_handle.is_ok() {
         return EntryKind::Handle;
      }

      let maybe_app_entry = InMail::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::InMail;
      }

      let maybe_app_entry = InAck::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::InAck;
      }

      let maybe_app_entry = PendingMail::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::PendingMail;
      }

      let maybe_app_entry = PendingAck::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::PendingAck;
      }

      let maybe_app_entry = OutMail::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::OutMail;
      }

      let maybe_app_entry = OutAck::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::OutAck;
      }

      let maybe_app_entry = FileChunk::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         return EntryKind::FileChunk;
      }

      // let maybe_app_entry = FileManifest::try_from(sb.clone());
      // if maybe_app_entry.is_ok() {
      //    return EntryKind::FileManifest;
      // }

      let maybe_app_entry = Path::try_from(sb.clone());
      if maybe_app_entry.is_ok() {
         debug!("EntryKind::from_entry_bytes() Path !!!").ok();

         return EntryKind::Path;
      }

      debug!("!!! EntryKind::from_entry_bytes() Failed !!!").ok();
      unreachable!()
   }

   ///
   pub fn index(&self) -> u8 {
      let mut index = 0;
      for entry_kind in EntryKind::iter() {
         if self == &entry_kind {
            // debug!("!!! EntryKind::index({:?}) = {}", self, index).ok();
            return index;
         }
         index += 1;
      }
      debug!("!!! EntryKind::index() Failed !!!").ok();
      unreachable!();
   }

   ///
   pub fn as_type(&self) ->EntryType {
      let app_type = AppEntryType::new(
         EntryDefIndex::from(self.index()),
         ZomeId::from(0), // since we have only one zome in our DNA (thank god)
         EntryVisibility::Public, // Everything Public for now...
      );
      EntryType::App(app_type)
   }
}

/// Get EntryType out of an Entry
pub fn determine_entry_type(eh: EntryHash, entry: &Entry) -> ExternResult<EntryType> {
   Ok(match entry {
      Entry::Agent(_agent_hash) => EntryType::AgentPubKey,
      Entry::CapClaim(_claim) => EntryType::CapClaim,
      Entry::CapGrant(_grant) => EntryType::CapGrant,
      Entry::App(_entry_bytes) => {
         // EntryKind::from_entry_bytes(entry_bytes.clone()).as_type()
         get_entry_type(eh)?
      },
   })
}


