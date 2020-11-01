use hdk3::prelude::*;
/*
use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
*/
use crate::{
    entry_kind, link_kind,
    AgentAddress, MailMessage,
    mail::entries::{Mail, PendingMail},
};

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a received mail. It is private.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct InMail {
    pub mail: Mail,
    pub from: AgentAddress,
    pub date_received: u64,
    pub outmail_address: Address,
}

pub fn inmail_def() -> ValidatingEntryType {
    entry!(
            name: entry_kind::InMail,
            description: "Entry for a received mail",
            sharing: Sharing::Public, // should be private
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<InMail>| {
                // FIXME
                Ok(())
            },
            links: [
                to!(
                    entry_kind::OutAck,
                    link_type: link_kind::Acknowledgment,
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

impl InMail {
    pub fn new(
        mail: Mail,
        from: AgentAddress,
        date_received: u64,
        outmail_address: Address,
    ) -> Self {
        Self {
            mail,
            from,
            date_received,
            outmail_address,
        }
    }

    pub fn from_direct(from: AgentAddress, dm: MailMessage) -> Self {
        let received_date = crate::snapmail_now();
        Self::new(dm.mail, from.clone(), received_date, dm.outmail_address)
    }

    pub fn from_pending(pending: PendingMail, from: AgentAddress) -> Self {
//        let maybe_mail = pending.decrypt(from);
//        if maybe_mail.is_err() {
//            return ZomeApiError();
//        }
        let received_date = crate::snapmail_now();
        Self::new(pending.mail, from.clone(), received_date, pending.outmail_address)
    }
}