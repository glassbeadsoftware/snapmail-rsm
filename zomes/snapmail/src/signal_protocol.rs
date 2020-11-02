use hdk3::prelude::*;

use crate::{
    mail::entries::MailItem,
    //file::FileManifest,
};

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub enum SignalProtocol {
    ReceivedMail(MailItem),
    ReceivedAck(ReceivedAck),
    //ReceivedFile(FileManifest),
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct ReceivedMail {
//     pub item: MailItem,
// }

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct ReceivedAck {
    pub from: AgentPubKey,
    pub for_mail: HeaderHash,
}
