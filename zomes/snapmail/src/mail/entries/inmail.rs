use hdk::prelude::*;

use crate::{
    MailMessage,
    mail::entries::{Mail, PendingMail},
    pub_enc_key::*,
    utils::*,
};

/// Entry representing a received mail.
#[hdk_entry(id = "inmail", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct InMail {
    pub mail: Mail,
    pub date_received: u64,
    pub outmail_eh: EntryHash,
    pub from: AgentPubKey,
    pub from_signature: Signature,
}

impl InMail {
    pub fn new(
        mail: Mail,
        from: AgentPubKey,
        date_received: u64,
        outmail_eh: EntryHash,
        from_signature: Signature,
    ) -> Self {
        Self {
            mail,
            from,
            date_received,
            outmail_eh,
            from_signature,
        }
    }


    pub fn from_direct(from: AgentPubKey, dm: MailMessage) -> Self {
        let received_date = crate::snapmail_now();
        Self::new(dm.mail, from.clone(), received_date, dm.outmail_eh, dm.mail_signature)
    }


    pub fn try_from_pending(pending: PendingMail, from: AgentPubKey) -> ExternResult<Option<Self>> {
        let received_date = crate::snapmail_now();
        /// Get my key
        let my_agent_key = agent_info()?.agent_latest_pubkey;
        debug!("try_from_pending my_agent_key: {}", my_agent_key);
        let recipient_key = get_enc_key(my_agent_key.clone())?;
        debug!("try_from_pending recipient_key: {:?}", recipient_key);
        /// Get sender's key
        let sender_key = get_enc_key(from.clone())?;
        debug!("try_from_pending sender_key: {:?}", sender_key);
        /// Decrypt
        let maybe_mail = pending.attempt_decrypt(sender_key, recipient_key);
        debug!("try_from_pending maybe_mail: {:?}", maybe_mail);
        /// Into InMail
        let inmail = match maybe_mail {
            None => return Ok(None),
            Some(mail) => {
                Self::new(mail,
                          from.clone(),
                          received_date,
                          pending.outmail_eh,
                          pending.from_signature.clone())
            },
        };
        /// Check signature
        let maybe_verified = verify_signature(from, pending.from_signature, inmail.mail.clone());
        match maybe_verified {
            Err(err) => {
                let response_str = "Verifying PendingMail failed";
                debug!("{}: {}", response_str, err);
                return error(response_str);
            }
            Ok(false) => {
                let response_str = "Failed verifying PendingMail signature";
                debug!("{}", response_str);
                return error(response_str);
            }
            Ok(true) => debug!("Valid PendingMail signature"),
        }
        /// Done
        Ok(Some(inmail))
    }

}
