use hdk::prelude::*;

use crate::{
    mail::entries::{
        Mail,
        AttachmentInfo,
    },
    file::FileManifest,
};

/// Entry representing an authored mail. It is private.
#[hdk_entry(id = "outmail")]
#[derive(Clone, PartialEq)]
pub struct OutMail {
    pub mail: Mail,
    pub bcc: Vec<AgentPubKey>,
}

///
impl OutMail {
    pub fn new(mail: Mail, bcc: Vec<AgentPubKey>) -> Self {
        Self {
            mail, bcc,
        }
    }

    pub fn create(
        subject: String,
        payload: String,
        to: Vec<AgentPubKey>,
        cc: Vec<AgentPubKey>,
        bcc: Vec<AgentPubKey>,
        file_manifest_list: Vec<(EntryHash, FileManifest)>,
    ) -> Self {
        assert_ne!(0, to.len() + cc.len() + bcc.len());
        // TODO: remove duplicate receipients

        let attachments: Vec<AttachmentInfo> = file_manifest_list
            .iter().map(|(eh, manifest)| AttachmentInfo::from_manifest(manifest.clone(), eh.clone()))
            .collect();

        let date_sent = crate::snapmail_now();
        let mail = Mail { date_sent, subject, payload, to, cc, attachments };
        OutMail::new(mail, bcc)
    }
}