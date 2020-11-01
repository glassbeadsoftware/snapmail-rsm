use hdk3::prelude::*;

/*
use hdk::{
    entry_definition::ValidatingEntryType,
};
*/

use crate::{entry_kind, link_kind};

/// Entry for an Acknowledgement Receipt of a Mail authored by this agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct OutAck {
}

pub fn outack_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::OutAck,
        description: "Entry for an Acknowledgement Receipt of a Mail authored by this agent",
        sharing: Sharing::Public, // should be private
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<OutAck>| {
            Ok(())
        },
        links: [
            to!(
                entry_kind::PendingAck,
                link_type: link_kind::Pending,
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

impl OutAck {
    pub fn new() -> Self {
        Self {
        }
    }
}