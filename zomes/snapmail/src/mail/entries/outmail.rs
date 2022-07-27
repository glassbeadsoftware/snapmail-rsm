use hdk::prelude::*;
use zome_utils::*;

use crate::{
    mail::entries::{
        Mail,
        AttachmentInfo,
        filter_up,
    },
    file::FileManifest,
};

/// Entry representing an authored mail. It is private.
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct OutMail {
    pub mail: Mail,
    pub reply_of: Option<HeaderHash>,
    pub bcc: Vec<AgentPubKey>,
}

///
impl OutMail {
    ///
    pub fn new(mail: Mail, bcc: Vec<AgentPubKey>, reply_of: Option<HeaderHash>) -> Self {
        Self {
            mail, reply_of, bcc,
        }
    }


    ///
    pub fn create(
        subject: String,
        payload: String,
        reply_of: Option<HeaderHash>,
        to: Vec<AgentPubKey>,
        cc: Vec<AgentPubKey>,
        in_bcc: Vec<AgentPubKey>,
        file_manifest_list: Vec<(EntryHash, FileManifest)>,
    ) -> Self {
        assert_ne!(0, to.len() + cc.len() + in_bcc.len());
        /// Remove duplicate recipients
        let mut bcc = filter_up(&to, &in_bcc);
        bcc = filter_up(&cc, &bcc);
        /// Get attachments
        let attachments: Vec<AttachmentInfo> = file_manifest_list
            .iter().map(|(eh, manifest)| AttachmentInfo::from_manifest(manifest.clone(), eh.clone()))
            .collect();
        /// Make sure reply_of is valid
        if let Some(reply_hh) = reply_of.clone() {
            let maybe = get_local_from_hh(reply_hh);
            assert!(maybe.is_ok());
        }
        /// Create Mail
        let mail = Mail::new(subject, payload, to, cc, attachments);
        /// Done
        OutMail::new(mail, bcc, reply_of)
    }


    /// Merge recipient lists
    pub fn recipients(&self) -> Vec<AgentPubKey> {
        let mut recipients: Vec<AgentPubKey> = self.bcc.clone();
        recipients.append(&mut self.mail.cc.clone());
        recipients.append(&mut self.mail.to.clone());
        recipients.sort();
        recipients.dedup();
        recipients
    }

}
