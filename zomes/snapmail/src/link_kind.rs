use hdk3::prelude::*;
use strum::AsStaticRef;


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

   pub fn concat(self, suffix: &str) -> LinkTag {
      /// concat with hardcoded separator
      let str = format!("{}___{}", self.as_static(), suffix);
      /// convert
      let sb: SerializedBytes = StringLinkTag(str.into())
         .try_into()
         .expect("StringLinkTag should convert to SerializedBytes");
      LinkTag(sb.bytes().clone())
   }
}