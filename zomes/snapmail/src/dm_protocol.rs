use hdk::prelude::*;

use crate::{
    mail::entries::Mail,
    file::{FileChunk, FileManifest},
};


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MailMessage {
    pub outmail_eh: EntryHash,
    pub mail: Mail,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AckMessage {
    pub outmail_eh: EntryHash,
}
