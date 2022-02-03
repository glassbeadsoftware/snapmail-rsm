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

/// Possible states of an InMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InMailState {
    /// PendingMail available
    Incoming,
    /// InMail written, no pendingMail
    Arrived,
    /// OutAck written, PendingAck available
    Acknowledged,
    /// OutAck written, no PendingAck
    AckReceived,
    ///
    Deleted,
}

/// State of a single delivery of a mail or ack to a unique recipient
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeliveryState {
    /// Initial state
    Unsent,
    /// Pending entry has been created and shared or DM has been sent and received
    Sent,
    /// Ack received and stored
    Acknowledged,
}


/// Possible states of an OutMail entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum OutMailState {
    /// (orange) Initial state
    Unsent,
    /// (black) All deliveries have been sent (pending or sent link)
    AllSent,
    /// (green) Has a receipt link for each recipient
    AllAcknowledged,
    /// (red) Delete requested by owner
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MailState {
    In(InMailState),
    Out(OutMailState),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MailItem {
    pub address: HeaderHash,
    pub author: AgentPubKey,
    pub mail: Mail,
    pub state: MailState,
    // pub delivery_states: Map<AgentPubKey, DeliveryState>
    pub bcc: Vec<AgentPubKey>,
    pub date: i64,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RecipientKind {
    TO,
    CC,
    BCC,
}

/// Core content of all *Mail Entries
/// Mail can have Zero public recipient (but must have at least one public or private recipient)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Mail {
    pub date_sent: u64,
    pub subject: String,
    pub payload: String,
    pub to: Vec<AgentPubKey>,
    pub cc: Vec<AgentPubKey>,
    pub attachments: Vec<AttachmentInfo>,
}

impl Mail {
    pub fn new(subject: String,
               payload: String,
               to: Vec<AgentPubKey>,
               in_cc: Vec<AgentPubKey>,
               attachments: Vec<AttachmentInfo>,
    ) -> Self {
        assert_ne!(0, attachments.len() + payload.len() + subject.len());
        /// Remove duplicate recipients
        let cc = filter_up(&to, &in_cc);
        /// Create Mail
        let date_sent = crate::snapmail_now();
        /// Done
        Self {
            date_sent,
            subject,
            payload,
            to,
            cc,
            attachments,
        }
    }
}

pub fn sign_mail(mail: &Mail) -> ExternResult<Signature> {
    let me = agent_info()?.agent_latest_pubkey;
    let signature = sign(me, mail)?;
    Ok(signature)
}


/// Metadata for a mail attachment
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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


/// Remove elements of first list present in second list
pub(crate) fn filter_up(upper_list: &Vec<AgentPubKey>, lower_list: &Vec<AgentPubKey>) -> Vec<AgentPubKey> {
    let mut new_lower_list = lower_list.clone();
    new_lower_list.retain(|x| !upper_list.contains(x));
    new_lower_list
}