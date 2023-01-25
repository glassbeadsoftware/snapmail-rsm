use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SignalProtocol {
    ReceivedMail(MailItem),
    ReceivedAck(ReceivedAck),
    ReceivedFile(FileManifest),
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReceivedAck {
    pub from: AgentPubKey,
    pub for_mail: ActionHash,
}
