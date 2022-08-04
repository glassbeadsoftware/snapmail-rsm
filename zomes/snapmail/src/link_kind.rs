use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;
use holo_hash::hash_type::{self, HashType};

//use std::str::FromStr;
//use strum::AsStaticRef;
//use strum_macros::EnumIter;

use zome_utils::error;

pub const LinkSeparator: &'static str = "___";

/// List of all Link kinds handled by this Zome
#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum LinkKind {
   Members,
   AckInbox,
   MailInbox,
   Handle,
   Pending,
   Pendings,
   EncKey,
   // Acknowledgment,
   // Receipt,
   // Sent,
   // Sents,
}


// /// Public
// impl LinkKind {
//    /// Convert to LinkTag
//    pub fn as_tag(&self) -> LinkTag {
//       let str = self.as_static();
//       LinkTag::new(str.as_bytes().clone())
//    }
//
//    /// Convert to Option<LinkTag>
//    pub fn as_tag_opt(&self) -> Option<LinkTag> {
//       Some(self.as_tag())
//    }
//
//    ///
//    pub fn allowed_base_type(&self) -> EntryType {
//       return self.prop_to_type("BaseType");
//    }
//
//    ///
//    pub fn allowed_target_type(&self) -> EntryType {
//       return self.prop_to_type("TargetType");
//    }
// }


// /// Private
// impl LinkKind {
//    /// Convert an EnumProperty to an EntryType
//    fn prop_to_type(&self, prop_name: &str) -> EntryType {
//       let kind_str = self.get_str(prop_name).unwrap();
//       if kind_str == "AgentPubKey" {
//          return EntryType::AgentPubKey;
//       }
//       let maybe_kind = EntryKind::from_str(kind_str);
//       if let Ok(kind) = maybe_kind {
//          return kind.as_type();
//       }
//       error!("!!! LinkKind::prop_to_type() Failed : {} !!!", kind_str);
//       unreachable!()
//    }
//
//    /// Check if link edges have correct types
//    pub fn validate_types(
//       self,
//       base: Entry,
//       target: Entry,
//       _maybe_hash: Option<AgentPubKey>,
//    ) -> ExternResult<ValidateCallbackResult> {
//       if !is_entry_of_type(base, self.allowed_base_type()) {
//          let msg = format!("Invalid base type for link kind `{}`", self.as_static()).into();
//          return Ok(ValidateCallbackResult::Invalid(msg));
//       }
//       if !is_entry_of_type(target, self.allowed_target_type()) {
//          let msg = format!("Invalid target type for link kind `{}`", self.as_static()).into();
//          return Ok(ValidateCallbackResult::Invalid(msg));
//       }
//       Ok(ValidateCallbackResult::Valid)
//    }
// }


/// Concat
impl LinkKind {
   // /// Create LinkTag with concatenated raw data
   // pub fn concat(&self, suffix: &[u8]) -> LinkTag {
   //    let mut vec = self.as_static().as_bytes().to_vec();
   //    vec.extend(LinkSeparator.as_bytes());
   //    vec.extend(suffix);
   //    LinkTag(vec)
   // }
   //
   // /// Retrieve raw data from LinkTag
   // pub fn unconcat(&self, tag: &LinkTag) -> ExternResult<Vec<u8>> {
   //    let raw_tag = tag.as_ref();
   //    let mut prefix = self.as_static().as_bytes().to_vec();
   //    prefix.extend(LinkSeparator.as_bytes());
   //    if raw_tag.len() <= prefix.len() {
   //       return error("Unconcat of link failed");
   //    }
   //    let tag_prefix = raw_tag[..prefix.len()].to_vec();
   //    if tag_prefix != prefix {
   //       return error("Unconcat for incorrect LinkKind");
   //    }
   //    let suffix = raw_tag[prefix.len()..].to_vec();
   //    Ok(suffix)
   // }


   /// Create LinkTag with concatenated hash
   pub fn from_hash<T: HashType>(hash: &HoloHash<T>) -> LinkTag {
      let raw = hash.get_raw_39();
      return raw.to_vec().into();
   }


   /// Retrieve hash from LinkTag
   pub fn into_hash<T: HashType>(tag: &LinkTag) -> ExternResult<HoloHash<T>> {
      let maybe_hash = HoloHash::from_raw_39(tag.clone().into_inner());
      //debug!("unconcat maybe_hash = {:?}", maybe_hash);
      if let Err(err) = maybe_hash {
         return error(&format!("into_hash() failed: {:?}", err));
      }
      Ok(maybe_hash.unwrap())
   }


   /// Create LinkTag with concatenated hash
   pub fn from_agent(key: &AgentPubKey) -> LinkTag {
      return Self::from_hash::<hash_type::Agent>(key);
   }


   /// Retrieve hash from LinkTag
   pub fn into_agent(tag: &LinkTag) -> ExternResult<AgentPubKey> {
      return Self::into_hash::<hash_type::Agent>(tag);
   }



   // /// Create LinkTag with concatenated string
   // pub fn concat_str(self, suffix: &str) -> LinkTag {
   //    let str = format!("{}{}{}", self.as_static(), LinkSeparator, suffix);
   //    LinkTag(str.as_bytes().to_vec())
   // }
   //
   // /// Retrieve string from LinkTag
   // pub fn unconcat_str(self, tag: &LinkTag) -> ExternResult<String> {
   //    let raw = tag.as_ref();
   //    let str = String::from_utf8_lossy(raw);
   //    let substrs: Vec<&str> = str.split(LinkSeparator).collect();
   //    if substrs.len() != 2 {
   //       return error("Unconcat of link failed");
   //    }
   //    if substrs[0] != self.as_static() {
   //       return error("Unconcat for incorrect LinkKind");
   //    }
   //    Ok(substrs[1].to_string())
   // }
}
