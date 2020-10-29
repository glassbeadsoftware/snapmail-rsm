use hdk::prelude::*;
use hdk::holochain_persistence_api::cas::content::Address;

use crate::{
    mail::entries::Mail, file::{FileChunk, FileManifest},
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub enum DirectMessageProtocol {
    Failure(String),
    Success(String),
    // Chunk(FileChunk),
    // FileManifest(FileManifest),
    // Mail(MailMessage),
    // Ack(AckMessage),
    // RequestChunk(Address),
    // RequestManifest(Address),
    UnknownEntry,
    Ping,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct MailMessage {
    pub outmail_address: Address,
    pub mail: Mail,
    pub manifest_address_list: Vec<Address>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct AckMessage {
    pub outmail_address: Address,
}
