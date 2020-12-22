use hdk3::prelude::*;

use super::Mail;

/// Entry representing a mail on the DHT waiting to be received by receipient
/// The receipient is the agentId where the entry is linked from,
/// hence only the receipient knows it has pending mail.
#[hdk_entry(id = "pending_mail")]
#[derive(Clone, Debug, PartialEq)]
pub struct PendingMail {
    pub mail: Mail,
    pub outmail_eh: EntryHash,
}

impl PendingMail {
    pub fn new(mail: Mail, outmail_eh: EntryHash) -> Self {
        Self {
            mail,
            outmail_eh,
        }
    }


// TODO Encryption
//    /// Create PendingMail from Mail and destination AgentId
//    /// This will encrypt the Mail with the destination's key
//    pub fn create(mail: Mail, _to: AgentId) -> Self {
//        // Serialize
//        let serialized = serde_json::to_string(mail).unwrap();
//
//        // Encrypt
//        let encrypted = serialized;
//        // TODO should be:
//        // const encrypted = hdk::encrypt(mail, to);
//
//        // Create
//        PendingMail::new(mail, encrypted)
//    }
//
//    pub fn decrypt(self, _from: AgentId) -> Result<Mail, ()> {
//        // decrypt
//        let maybe_decrypted = Ok(self.outmail_eh);
//        // TODO should be:
//        // const maybe_decrypted = hdk::decrypt(self.encrypted_mail, from);
//        // if maybe_decrypted.is_err() {
//        //     return Err();
//        // }
//        // deserialize
//        let maybe_mail: Result<Mail> = serde_json::from_str(&decrypted.unwrap());
//        maybe_mail
//    }

}