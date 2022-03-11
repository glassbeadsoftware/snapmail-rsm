use hdk::prelude::*;

use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum::EnumProperty;

use zome_utils::*;

use crate::{
   handle::*,
   mail::entries::*,
   file::*,
   pub_enc_key::*,
};

/// !! Keep Order synced with EntryKind !!
entry_defs![
   /// -- PubEncKey
   PubEncKey::entry_def(),
   /// -- Handle
   Handle::entry_def(),
   /// -- Mail
   InMail::entry_def(),
   OutMail::entry_def(),
   OutAck::entry_def(),
   InAck::entry_def(),
   PendingMail::entry_def(),
   PendingAck::entry_def(),
   DeliveryConfirmation::entry_def(),
   /// -- File
   FileChunk::entry_def(),
   FileManifest::entry_def(),
   /// -- Other
   PathEntry::entry_def()
];

/// Listing all Entry kinds for this DNA
/// !! Visibility prop value must match hdk_entry visibility !!
#[derive(AsStaticStr, EnumIter, EnumProperty, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntryKind {
   /// !! Keep Order synced with entry_defs!() !!
   #[strum(props(Visibility = "public"))]
   PubEncKey,
   #[strum(props(Visibility = "public"))]
   Handle,
   #[strum(props(Visibility = "private"))]
   InMail,
   #[strum(props(Visibility = "private"))]
   OutMail,
   #[strum(props(Visibility = "private"))]
   OutAck,
   #[strum(props(Visibility = "private"))]
   InAck,
   #[strum(props(Visibility = "public"))]
   PendingMail,
   #[strum(props(Visibility = "public"))]
   PendingAck,
   #[strum(props(Visibility = "private"))]
   DeliveryConfirmation,
   #[strum(props(Visibility = "private"))]
   FileChunk,
   #[strum(props(Visibility = "private"))]
   FileManifest,
   #[strum(props(Visibility = "public"))]
   Path,
}

impl FromStr for EntryKind {
   type Err = ();
   fn from_str(input: &str) -> Result<EntryKind, Self::Err> {
      match input {
         "PubEncKey"  => Ok(EntryKind::PubEncKey),
         "Handle"  => Ok(EntryKind::Handle),
         "InMail"  => Ok(EntryKind::InMail),
         "InAck"  => Ok(EntryKind::InAck),
         "PendingMail" => Ok(EntryKind::PendingMail),
         "PendingAck"  => Ok(EntryKind::PendingAck),
         "DeliveryConfirmation"  => Ok(EntryKind::DeliveryConfirmation),
         "OutMail"  => Ok(EntryKind::OutMail),
         "OutAck"  => Ok(EntryKind::OutAck),
         "FileChunk" => Ok(EntryKind::FileChunk),
         "FileManifest" => Ok(EntryKind::FileManifest),
         "Path" => Ok(EntryKind::Path),
         //
         "AppPubKey" => {
            error!("EntryKind::from_str() FAILED on AppPubKey");
            Err(())
         },
         "App" => {
            error!("EntryKind::from_str() FAILED on App");
            Err(())
         },
         "AgentPubKey" => {
            error!("EntryKind::from_str() FAILED on AgentPubKey");
            Err(())
         },
         //
         _ => {
            error!("EntryKind::from_str() FAILED on input: {}", input);
            Err(())
         },
      }
   }
}


impl EntryKind {

   ///
   pub fn visibility(&self) -> EntryVisibility {
      let visibility_str = self.get_str("Visibility").unwrap();
      match visibility_str {
         "public" => EntryVisibility::Public,
         "private" => EntryVisibility::Private,
         _ => unreachable!(),
      }
   }

   /// Not optimal but works
   pub fn from_index(index: &EntryDefIndex) -> Self {
      for entry_kind in EntryKind::iter() {
         if entry_kind.index() == index.index() as u8 {
            return entry_kind;
         }
      }
      unreachable!()
   }

   ///
   pub fn index(&self) -> u8 {
      let mut index = 0;
      for entry_kind in EntryKind::iter() {
         if self == &entry_kind {
            // debug!("!!! EntryKind::index({:?}) = {}", self, index);
            return index;
         }
         index += 1;
      }
      error!("!!! EntryKind::index() Failed !!!");
      unreachable!();
   }

   ///
   pub fn as_type(&self) -> EntryType {
      let app_type = AppEntryType::new(
         EntryDefIndex::from(self.index()),
         ZomeId::from(0), // since we have only one zome in our DNA (thank god)
         self.visibility(),
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
      Entry::App(_entry_bytes) => get_entry_type_from_eh(eh)?,
      Entry::CounterSign(_data, _bytes) => unreachable!(),
   })
}

/// Try to deserialize entry to given type
pub(crate) fn is_type(entry: Entry, type_candidat: EntryType) -> bool {
   trace!("*** is_type() called: {:?} == {:?} ?", type_candidat, entry);
   let res =  match entry {
      Entry::CounterSign(_data, _bytes) => unreachable!(),
      Entry::Agent(_agent_hash) => EntryType::AgentPubKey == type_candidat,
      Entry::CapClaim(_claim) => EntryType::CapClaim == type_candidat,
      Entry::CapGrant(_grant) => EntryType::CapGrant == type_candidat,
      Entry::App(entry_bytes) => {
         let mut res = false;
         if let EntryType::App(app_entry_type) = type_candidat.clone() {
            res = can_deserialize(app_entry_type.id(), entry_bytes)
         }
         res
       },
   };
   //debug!("*** is_type({:?}) result = {}", type_candidat, res);
   res
}

///
fn can_deserialize(entry_type_id: EntryDefIndex, entry_bytes: AppEntryBytes) -> bool {
   trace!("*** can_deserialize() called! ({:?})", entry_type_id);
   let sb = entry_bytes.into_sb();
   let entry_kind = EntryKind::from_index(&entry_type_id);

   match entry_kind {
      EntryKind::PubEncKey => PubEncKey::try_from(sb.clone()).is_ok(),
      EntryKind::Handle => Handle::try_from(sb.clone()).is_ok(),
      EntryKind::Path => PathEntry::try_from(sb.clone()).is_ok(),
      EntryKind::InMail => InMail::try_from(sb.clone()).is_ok(),
      EntryKind::InAck => InAck::try_from(sb.clone()).is_ok(),
      EntryKind::PendingMail => PendingMail::try_from(sb.clone()).is_ok(),
      EntryKind::PendingAck => PendingAck::try_from(sb.clone()).is_ok(),
      EntryKind::DeliveryConfirmation => DeliveryConfirmation::try_from(sb.clone()).is_ok(),
      EntryKind::OutMail => OutMail::try_from(sb.clone()).is_ok(),
      EntryKind::OutAck => OutAck::try_from(sb.clone()).is_ok(),
      EntryKind::FileManifest => FileManifest::try_from(sb.clone()).is_ok(),
      EntryKind::FileChunk => FileChunk::try_from(sb.clone()).is_ok(),
   }
}

