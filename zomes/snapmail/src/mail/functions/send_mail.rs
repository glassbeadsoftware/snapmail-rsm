use hdk::prelude::*;

use std::collections::HashMap;

use crate::{
    utils::*,
    send_dm,
    mail::entries::{PendingMail, ReceipientKind, Mail, OutMail},
    dm_protocol::{
        MailMessage, DirectMessageProtocol,
    },
    mail::receive::*,
    LinkKind,
    file::{FileManifest, FileChunk, get_manifest},
};

#[allow(non_camel_case_types)]
pub enum SendSuccessKind {
    OK_SELF,
    OK_DIRECT,
    OK_PENDING(HeaderHash),
}

/// Struct holding all result data from a send request
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMailOutput {
    pub outmail: HeaderHash,
    pub to_pendings: HashMap<String, HeaderHash>,
    pub cc_pendings: HashMap<String, HeaderHash>,
    pub bcc_pendings: HashMap<String, HeaderHash>,
}

impl SendMailOutput {
    pub fn new(outmail_hh: HeaderHash) -> Self {
        Self {
            outmail: outmail_hh,
            to_pendings: HashMap::new(),
            cc_pendings: HashMap::new(),
            bcc_pendings: HashMap::new(),
        }
    }

    pub fn add_pending(&mut self, kind: ReceipientKind, agent_id: &AgentPubKey, hh: HeaderHash) {
        let agent_str = format!("{}", agent_id);
        match kind {
            ReceipientKind::TO => self.to_pendings.insert(agent_str, hh),
            ReceipientKind::CC => self.cc_pendings.insert(agent_str, hh),
            ReceipientKind::BCC => self.bcc_pendings.insert(agent_str, hh),
        };
    }
}

///
fn send_manifest_by_dm(
    destination: &AgentPubKey,
    manifest: &FileManifest,
) -> ExternResult<()> {
    debug!("send_manifest_by_dm(): {:?}", destination);
    /// Send DM
    let response = send_dm(
        destination.clone(),
        DirectMessageProtocol::FileManifest(manifest.clone()),
    );
    debug!("send_manifest result = {:?}", response);
    /// Check Response
    if let Err(e) = response {
        return error(&format!("send_dm() of manifest failed: {}", e));
    }
    /// Return manifest's entry address on receiver's source chain
    match response.unwrap() {
        DirectMessageProtocol::Success(_manifest_address) => Ok(()),
        _ => error("hdk::send() of manifest failed".into())
    }
}

///
fn send_chunk_by_dm(destination: &AgentPubKey, chunk_eh: &EntryHash) -> ExternResult<()> {
    debug!("send_chunk_by_dm(): {}", chunk_eh);
    let maybe_el = get(chunk_eh.clone(), GetOptions::content())?;
        //.expect("No reason for get_entry() to crash");
    debug!("maybe_entry =  {:?}", maybe_el);
    if maybe_el.is_none() {
        return error("No chunk found at given address".into());
    }
    let chunk = get_typed_from_el::<FileChunk>(maybe_el.unwrap())?;
    /// Send DM
    let response = send_dm(
        destination.clone(),
        DirectMessageProtocol::Chunk(chunk),
    );
    debug!("send_chunk result = {:?}", response);
    /// Check Response
    if let Err(e) = response {
        return error(&format!("hdk::send() of chunk failed: {}", e));
    }
    match response.unwrap() {
        DirectMessageProtocol::Success(_) => Ok(()),
        _ => error("hdk::send() of chunk failed".into()),
    }
}

///
fn send_attachment_by_dm(destination: &AgentPubKey, manifest: &FileManifest) -> ExternResult<()> {
    /// Send each chunk first
    for chunk_eh in &manifest.chunks {
        send_chunk_by_dm(destination, chunk_eh)?;
    }
    /// Send FileManifest
    send_manifest_by_dm(destination, manifest)?;
    /// Done
    Ok(())
}


// TODO: use post-commit callback to send the mail via DM

/// Attempt sending Mail and attachments via Direct Messaging
fn send_mail_by_dm(
    outmail_eh: &EntryHash,
    mail: &Mail,
    destination: &AgentPubKey,
    manifest_list: &Vec<FileManifest>,
) -> ExternResult<()> {
    /// -- Send Attachments
    debug!("Send Attachments");
    /// For each attachment, send all the chunks
    for manifest in manifest_list {
        let result = send_attachment_by_dm(destination, manifest);
        if let Err(e) = result {
            let err_msg = format!("Send attachment failed -> Err: {}", e);
            debug!(?err_msg);
            return error(&err_msg);
        }
    }
    /// --  Send Mail
    debug!("send_mail_by_dm() to {}", destination);
    /// Create DM
    let msg = MailMessage {
        outmail_eh: outmail_eh.clone(),
        mail: mail.clone(),
    };
    /// Send DM
    let response_dm = send_dm(destination.clone(), DirectMessageProtocol::Mail(msg))?;
    debug!("send_mail_to() response_dm = {:?}", response_dm);
    /// Check Response
    if let DirectMessageProtocol::Success(_) = response_dm {
        return Ok(());
    }
    return error(&format!("send_dm() failed: {:?}", response_dm));
}

///
fn send_mail_to(
    outmail_eh: &EntryHash,
    mail: &Mail,
    destination: &AgentPubKey,
    manifest_list: &Vec<FileManifest>,
) -> ExternResult<SendSuccessKind> {
    debug!("send_mail_to() START - {}", destination);
    /// Shortcut to self
    let me = agent_info()?.agent_latest_pubkey;
    if destination.clone() == me {
        debug!("send_mail_to() Self");
        let msg = MailMessage {
            outmail_eh: outmail_eh.clone(),
            mail: mail.clone(),
        };
        let res = receive_dm_mail(me, msg);
        assert!(res == DirectMessageProtocol::Success("Mail received".to_string()));
        return Ok(SendSuccessKind::OK_SELF);
    }
    /// Try sending directly to other Agent if Online
    let result = send_mail_by_dm(outmail_eh, mail, destination, manifest_list);
    if result.is_ok() {
        return Ok(SendSuccessKind::OK_DIRECT);
    } else {
        let err = result.err().unwrap();
        debug!("send_mail_by_dm() failed: {:?}", err);
    }
    /// DM failed, send to DHT instead by creating a PendingMail
    /// Commit PendingMail
    let pending_mail = PendingMail::new(mail.clone(), outmail_eh.clone());
    let pending_mail_eh = hash_entry(&pending_mail)?;
    let maybe_pending_mail_hh = create_entry(&pending_mail);
    if let Err(err) = maybe_pending_mail_hh.clone() {
        debug!("PendingMail create_entry() failed = {:?}", err);
        return Err(maybe_pending_mail_hh.err().unwrap());
    };
    let pending_mail_hh = maybe_pending_mail_hh.unwrap();
    debug!("pending_mail_hh = {}", pending_mail_hh);
    /// Commit Pendings Link
    let tag = LinkKind::Pendings.concat_hash(destination);
    debug!("pendings tag = {:?}", tag);
    let maybe_link1_hh = create_link(outmail_eh.clone(), pending_mail_eh.clone(), tag);
    if let Err(err) = maybe_link1_hh.clone() {
        debug!("link1 failed = {:?}", err);
        return Err(maybe_link1_hh.err().unwrap());
    };
    let link1_hh = maybe_link1_hh.unwrap();
    debug!("link1_hh = {}", link1_hh);
    /// Commit MailInbox Link
    let tag = LinkKind::MailInbox.concat_hash(&me);
    let maybe_link2_hh = create_link(EntryHash::from(destination.clone()), pending_mail_eh, tag);
    if let Err(err) = maybe_link2_hh.clone() {
        debug!("link2 failed = {:?}", err);
        return Err(maybe_link2_hh.err().unwrap());
    };
    let link2_hh = maybe_link2_hh.unwrap();
    debug!("link2_hh = {}", link2_hh);
    /// Done
    Ok(SendSuccessKind::OK_PENDING(pending_mail_hh))
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMailInput {
    pub subject: String,
    pub payload: String,
    pub to: Vec<AgentPubKey>,
    pub cc: Vec<AgentPubKey>,
    pub bcc: Vec<AgentPubKey>,
    pub manifest_address_list: Vec<HeaderHash>,
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient.
/// if receipient not online, creates a PendingMail on the DHT.
#[hdk_extern]
#[snapmail_api]
pub fn send_mail(input: SendMailInput) -> ExternResult<SendMailOutput> {
    debug!("Sending mail: {}", input.subject);
    /// Get file manifests from addresses
    let mut file_manifest_list = Vec::new();
    let mut file_manifest_pair_list = Vec::new();
    for manifest_hh in input.manifest_address_list.clone() {
        let manifest_eh = hh_to_eh(manifest_hh.clone())?;
        let manifest = get_manifest(manifest_hh.clone().into())?;
        file_manifest_list.push(manifest.clone());
        file_manifest_pair_list.push((manifest_eh, manifest))
    }
    /// Create and commit OutMail
    let outmail = OutMail::create(
        input.subject,
        input.payload,
        input.to.clone(),
        input.cc.clone(),
        input.bcc.clone(),
        file_manifest_pair_list.clone(),
    );
    let outmail_hh = create_entry(&outmail)?;
    let outmail_eh = hash_entry(&outmail)?;
    debug!("OutMail created: {:?}", outmail_hh);
    /// Send to each recepient
    let mut total_result = SendMailOutput::new(outmail_hh.clone());
    /// to
    for agent in input.to {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::TO, &agent, pending_hh);
        }
    }
    /// cc
    for agent in input.cc {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::CC, &agent, pending_hh);
        }
    }
    /// bcc
    for agent in input.bcc {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::BCC, &agent, pending_hh);
        }
    }
    /// Done
    debug!("send's total_result: {:?}", total_result);
    Ok(total_result)
}
