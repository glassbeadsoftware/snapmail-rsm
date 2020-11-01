use hdk3::prelude::*;

/*
use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
*/

use crate::{entry_kind, link_kind};

use super::Mail;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a mail on the DHT waiting to be received by receipient
/// The receipient is the agentId where the entry is linked from,
/// hence only the receipient knows it has pending mail.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PendingMail {
    pub mail: Mail,
    pub outmail_address: Address,
}

pub fn pending_mail_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::PendingMail,
        description: "Entry for a mail held in the DHT waiting to be received by its receipient",
        sharing: Sharing::Public, // should be encrypted
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<PendingMail>| {
            // FIXME
            Ok(())
        },
        links: [
            from!(
                entry_kind::Handle,
                link_type: link_kind::MailInbox,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME
                    Ok(())
                }
            )
        ]
    )
}

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

impl PendingMail {
    pub fn new(mail: Mail, outmail_address: Address) -> Self {
        Self {
            mail,
            outmail_address,
        }
    }


// FIXME Encryption
//    /// Create PendingMail from Mail and destination AgentId
//    /// This will encrypt the Mail with the destination's key
//    pub fn create(mail: Mail, _to: AgentId) -> Self {
//        // Serialize
//        let serialized = serde_json::to_string(mail).unwrap();
//
//        // Encrypt
//        let encrypted = serialized;
//        // FIXME should be:
//        // const encrypted = hdk::encrypt(mail, to);
//
//        // Create
//        PendingMail::new(mail, encrypted)
//    }
//
//    pub fn decrypt(self, _from: AgentId) -> Result<Mail, ()> {
//        // decrypt
//        let maybe_decrypted = Ok(self.outmail_address);
//        // FIXME should be:
//        // const maybe_decrypted = hdk::decrypt(self.encrypted_mail, from);
//        // if maybe_decrypted.is_err() {
//        //     return Err();
//        // }
//        // deserialize
//        let maybe_mail: Result<Mail> = serde_json::from_str(&decrypted.unwrap());
//        maybe_mail
//    }
}