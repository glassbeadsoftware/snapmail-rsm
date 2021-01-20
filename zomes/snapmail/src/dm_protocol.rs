use hdk3::prelude::*;

use crate::{
    mail::entries::Mail,
    //file::{FileChunk, FileManifest},
};


#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes, PartialEq)]
pub enum DirectMessageProtocol {
    Failure(String),
    Success(String),
    Mail(MailMessage),
    Ack(AckMessage),
    // Chunk(FileChunk),
    // FileManifest(FileManifest),
    // RequestChunk(HeaderHash),
    // RequestManifest(HeaderHash),
    UnknownEntry,
    Ping,
}


#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct MailMessage {
    pub outmail_eh: EntryHash,
    pub mail: Mail,
    //pub manifest_address_list: Vec<HeaderHash>, FIXME
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct AckMessage {
    pub outmail_eh: EntryHash,
}
