use hdk3::prelude::*;

/*
use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing,
    },
};
*/

use crate::{
    AgentAddress, link_kind, entry_kind,
    mail::entries::Mail,
    file::FileManifest,
};
use crate::mail::entries::AttachmentInfo;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing an authored mail. It is private.
#[hdk_entry(id = "outmail")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct OutMail {
    pub mail: Mail,
    pub bcc: Vec<AgentAddress>,
}

/// Entry definition
/*
pub fn outmail_def() -> ValidatingEntryType {
    entry!(
        name: entry_kind::OutMail,
        description: "Entry for a mail authored by this agent",
        sharing: Sharing::Public, // should be private
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<OutMail>| {
            // FIXME: Check no duplicate recepient?
            // FIXME: Check no duplicate attachment?
            Ok(())
        },
        links: [
            to!(
                entry_kind::InAck,
                link_type: link_kind::Receipt,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check if receipt for this author already received?
                    Ok(())
                }
            ),
            to!(
                entry_kind::PendingMail,
                link_type: link_kind::Pendings,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    // FIXME: Check that outmail_address within PendingMail corresponds
                    // FIXME: Check PendingMail is authored by same agant
                    Ok(())
                }
            )
        ]
    )
}
*/

//-------------------------------------------------------------------------------------------------
// Implementation
//-------------------------------------------------------------------------------------------------

///
impl OutMail {
    pub fn new(mail: Mail, bcc: Vec<AgentAddress>) -> Self {
        Self {
            mail, bcc,
        }
    }

    pub fn create(
        subject: String,
        payload: String,
        to: Vec<AgentAddress>,
        cc: Vec<AgentAddress>,
        bcc: Vec<AgentAddress>,
        file_manifest_list: Vec<(Address, FileManifest)>,
    ) -> Self {
        assert_ne!(0, to.len() + cc.len() + bcc.len());
        // TODO: remove duplicate receipients

        let attachments: Vec<AttachmentInfo> = file_manifest_list
            .iter().map(|(address, manifest)| AttachmentInfo::from_manifest(manifest.clone(), address.clone()))
            .collect();

        let date_sent = crate::snapmail_now();
        let mail = Mail { date_sent, subject, payload, to, cc, attachments };
        OutMail::new(mail, bcc)
    }
}