use hdk::prelude::*;

use crate::{
    mail::entries::Mail,
    file::{FileChunk, FileManifest},
};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DirectMessageProtocol {
    Failure(String),
    Success(String),
    Mail(MailMessage),
    Ack(AckMessage),
    Chunk(FileChunk),
    FileManifest(FileManifest),
    RequestChunk(EntryHash),
    RequestManifest(EntryHash),
    UnknownEntry,
    Ping,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct MailMessage {
    pub mail: Mail,
    pub outmail_eh: EntryHash,
    pub mail_signature: Signature,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
pub struct AckMessage {
    pub outmail_eh: EntryHash,
    pub ack_signature: Signature,
}
