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

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PendingAck {
    pub outmail_address: Address,
}

pub fn pending_ack_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::PendingAck,
        description: "Entry for an Acknowledgement Receipt of a Mail to be stored on the DHT",
        sharing: Sharing::Public, // should be Encrypted
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<PendingAck>| {
            // FIXME
            Ok(())
        },
        links: [
            from!(
                entry_kind::Handle,
                link_type: link_kind::AckInbox,
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

impl PendingAck {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail_address,
        }
    }
}
