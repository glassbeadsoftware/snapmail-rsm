use hdk3::prelude::*;

use strum::AsStaticRef;

// /// Listing all Holochain Link kinds in this DNA
// pub const Acknowledgment: &'static str = "acknowledgement";
// pub const AckInbox: &'static str = "ack_inbox";
// pub const MailInbox: &'static str = "mail_inbox";
// pub const Members: &'static str = "members";
// pub const Handle: &'static str = "handle";
// pub const Pending: &'static str = "pending";
// pub const Pendings: &'static str = "pendings";
// //pub const Reply: &'static str = "reply";
// pub const Receipt: &'static str = "receipt";
// //pub const InitialChunks: &'static str = "initial_chunks";



#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);

// ///
// pub fn link_tag(tag: &str) -> LinkTag {
//    let sb: SerializedBytes = StringLinkTag(tag.into())
//       .try_into()
//       .expect("StringLinkTag should convert to SerializedBytes");
//    LinkTag(sb.bytes().clone())
// }

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