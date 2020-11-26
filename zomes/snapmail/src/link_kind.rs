use hdk3::prelude::*;
use holo_hash::hash_type::HashType;

use strum::AsStaticRef;
use crate::utils::*;

pub const LinkSeparator: &'static str = "___";

/// Listing all Link kinds for this DNA
#[derive(AsStaticStr, Clone, Debug, Serialize, Deserialize, SerializedBytes, PartialEq)]
pub enum LinkKind {
   Acknowledgment,
   AckInbox,
   MailInbox,
   Members,
   Handle,
   Pending,
   Pendings,
   Receipt,
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);

impl LinkKind {
   pub fn as_tag(self) -> LinkTag {
      let sb: SerializedBytes = StringLinkTag(self.as_static().into())
         .try_into()
         .expect("StringLinkTag should convert to SerializedBytes");
      LinkTag(sb.bytes().clone())
   }

   pub fn as_tag_opt(self) -> Option<LinkTag> {
      Some(self.as_tag())
   }

   /// Create LinkTag with concatenated raw data
   pub fn concat(self, suffix: &[u8]) -> LinkTag {
      let mut vec = self.as_static().as_bytes().to_vec();
      vec.extend(LinkSeparator.as_bytes());
      vec.extend(suffix);
      LinkTag(vec)
   }

   /// Retrieve raw data from LinkTag
   pub fn unconcat(self, tag: &LinkTag) -> ExternResult<Vec<u8>> {
      let raw_tag = tag.as_ref();
      let mut prefix = self.as_static().as_bytes().to_vec();
      prefix.extend(LinkSeparator.as_bytes());
      if raw_tag.len() <= prefix.len() {
         return error("Unconcat of link failed");
      }
      let tag_prefix = raw_tag[..prefix.len()].to_vec();
      if tag_prefix != prefix {
         return error("Unconcat for incorrect LinkKind");
      }
      let suffix = raw_tag[prefix.len()..].to_vec();
      Ok(suffix)
   }

   /// Create LinkTag with concatenated hash
   pub fn concat_hash<T: HashType>(self, hash: &HoloHash<T>) -> LinkTag {
      let raw = hash.get_raw_39();
      return self.concat(raw);
   }

   /// Retrieve hash from LinkTag
   pub fn unconcat_hash<T: HashType>(self, tag: &LinkTag) -> ExternResult<HoloHash<T>> {
      let suffix = self.unconcat(tag)?;
      let maybe_hash = HoloHash::from_raw_39(suffix);
      if let Err(_err) = maybe_hash {
         return error("unconcat_hash() failed");
      }
      Ok(maybe_hash.unwrap())
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