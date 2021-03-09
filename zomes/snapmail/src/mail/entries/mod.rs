mod inmail;
mod outmail;
mod pending_mail;
mod pending_ack;
mod outack;
mod inack;

use hdk::prelude::*;

pub use self::{
    inmail::*, pending_mail::*, outmail::*,
    pending_ack::*, inack::*, outack::*,
};

use crate::{
    file::FileManifest,
};

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum InMailState {
    // PendingMail available
    Incoming,
    // InMail written, no pendingMail
    Arrived,
    // OutAck written, PendingAck available
    Acknowledged,
    // OutAck written, no PendingAck
    AckReceived,
    //
    Deleted,
}


#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum OutMailState {
    // Has a pending link for each receipient
    Pending,
    // Has less pending links than receipients, and no receipt links
    PartiallyArrived_NoAcknowledgement,
    // Has less pending links than receipients, and less receipt links than receipients
    PartiallyArrived_PartiallyAcknowledged,
    // Has no pending links, and a receipt link for each receipient
    Arrived_NoAcknowledgement,
    // Has no pending links, and less receipt links than receipients
    Arrived_PartiallyAcknowledged,
    // Has no pendings links, and a receipt link for each receipient
    Received,
    //
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum MailState {
    In(InMailState),
    Out(OutMailState),
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct MailItem {
    pub address: HeaderHash,
    pub author: AgentPubKey,
    pub mail: Mail,
    pub state: MailState,
    pub bcc: Vec<AgentPubKey>,
    pub date: i64,
}


#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum ReceipientKind {
    TO,
    CC,
    BCC,
}

/// Core content of all *Mail Entries
/// Mail can have Zero public receipient (but must have at least one public or private receipient)
#[derive(Serialize, Deserialize, Debug, SerializedBytes, PartialEq, Clone)]
pub struct Mail {
    pub date_sent: u64,
    pub subject: String,
    pub payload: String,
    pub to: Vec<AgentPubKey>,
    pub cc: Vec<AgentPubKey>,
    pub attachments: Vec<AttachmentInfo>,
}

/// Metadata for a mail attachment
#[derive(Serialize, Deserialize, PartialEq, Debug, SerializedBytes, Clone)]
pub struct AttachmentInfo {
    pub manifest_eh: EntryHash,
    pub data_hash: String,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
}

impl AttachmentInfo {
    fn from_manifest(manifest: FileManifest, manifest_eh: EntryHash) -> Self {
        Self {
            manifest_eh: manifest_eh.clone(),
            data_hash: manifest.data_hash.clone(),
            filename: manifest.filename.clone(),
            filetype: manifest.filetype.clone(),
            orig_filesize: manifest.orig_filesize,
        }
    }
}