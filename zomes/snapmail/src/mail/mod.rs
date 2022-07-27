pub mod functions;
mod receive;
mod utils;

pub(crate) use utils::*;
pub use functions::*;
pub use receive::*;

use crate::{
   mail::{
      Mail,
      AttachmentInfo,
      filter_up,
   },
   file::FileManifest,
};



pub fn sign_mail(mail: &Mail) -> ExternResult<Signature> {
   let me = agent_info()?.agent_latest_pubkey;
   let signature = sign(me, mail)?;
   Ok(signature)
}




///
pub fn create_outmail(
   subject: String,
   payload: String,
   reply_of: Option<ActionHash>,
   to: Vec<AgentPubKey>,
   cc: Vec<AgentPubKey>,
   in_bcc: Vec<AgentPubKey>,
   file_manifest_list: Vec<(EntryHash, FileManifest)>,
) -> OutMail {
   assert_ne!(0, to.len() + cc.len() + in_bcc.len());
   /// Remove duplicate recipients
   let mut bcc = filter_up(&to, &in_bcc);
   bcc = filter_up(&cc, &bcc);
   /// Get attachments
   let attachments: Vec<AttachmentInfo> = file_manifest_list
      .iter().map(|(eh, manifest)| AttachmentInfo::from_manifest(manifest.clone(), eh.clone()))
      .collect();
   /// Make sure reply_of is valid
   if let Some(reply_ah) = reply_of.clone() {
      let maybe = get_local_from_ah(reply_ah);
      assert!(maybe.is_ok());
   }
   /// Create Mail
   let mail = Mail::new(subject, payload, to, cc, attachments);
   /// Done
   OutMail::new(mail, bcc, reply_of)
}