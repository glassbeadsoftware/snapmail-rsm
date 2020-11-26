use hdk3::prelude::*;

use crate::{
    MailMessage,
    mail::entries::{Mail, PendingMail},
};

/// Entry representing a received mail. It is private.
#[hdk_entry(id = "inmail")]
#[derive(Debug, Clone, PartialEq)]
pub struct InMail {
    pub mail: Mail,
    pub from: AgentPubKey,
    pub date_received: u64,
    pub outmail_eh: EntryHash,
}

impl InMail {
    pub fn new(
        mail: Mail,
        from: AgentPubKey,
        date_received: u64,
        outmail_eh: EntryHash,
    ) -> Self {
        Self {
            mail,
            from,
            date_received,
            outmail_eh,
        }
    }

    pub fn from_direct(from: AgentPubKey, dm: MailMessage) -> Self {
        let received_date = crate::snapmail_now();
        Self::new(dm.mail, from.clone(), received_date, dm.outmail_eh)
    }

    pub fn from_pending(pending: PendingMail, from: AgentPubKey) -> Self {
//        let maybe_mail = pending.decrypt(from);
//        if maybe_mail.is_err() {
//            return ZomeApiError();
//        }
        let received_date = crate::snapmail_now();
        Self::new(pending.mail, from.clone(), received_date, pending.outmail_eh)
    }
}
