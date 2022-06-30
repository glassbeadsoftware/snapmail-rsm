use hdk::prelude::*;
use zome_utils::*;

use crate::{
    send_dm,
    mail::entries::{PendingMail, Mail, OutMail, InMail, sign_mail},
    dm_protocol::{
        MailMessage, DirectMessageProtocol,
    },
    //mail::receive::*,
    LinkKind,
    file::{FileManifest, FileChunk, get_manifest},
};
use crate::mail::entries::DeliveryConfirmation;


#[allow(non_camel_case_types)]
pub enum SendSuccessKind {
    OK_SELF,
    OK_DIRECT,
    OK_PENDING,
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMailInput {
    pub subject: String,
    pub payload: String,
    pub reply_of: Option<HeaderHash>,
    pub to: Vec<AgentPubKey>,
    pub cc: Vec<AgentPubKey>,
    pub bcc: Vec<AgentPubKey>,
    pub manifest_address_list: Vec<HeaderHash>,
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


/// Attempt sending Mail and attachments via Direct Messaging
fn deliver_mail_by_dm(
    outmail_eh: &EntryHash,
    mail: &Mail,
    destination: &AgentPubKey,
    manifest_list: &Vec<FileManifest>,
    signature: &Signature,
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
    debug!("deliver_mail_by_dm() to {}", destination);
    /// Create DM
    let msg = MailMessage {
        outmail_eh: outmail_eh.clone(),
        mail: mail.clone(),
        mail_signature: signature.clone(),
    };
    /// Send DM
    let response_dm = send_dm(destination.clone(), DirectMessageProtocol::Mail(msg))?;
    debug!("deliver_mail_by_dm() response_dm = {:?}", response_dm);
    /// Check Response
    if let DirectMessageProtocol::Success(_) = response_dm {
        return Ok(());
    }
    return error(&format!("send_dm() failed: {:?}", response_dm));
}


#[hdk_extern]
fn commit_inmail(inmail: InMail) -> ExternResult<HeaderHash> {
    debug!("commit_inmail() START **********");
    create_entry(inmail)
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct CommitPendingMailInput {
    mail: PendingMail,
    outmail_eh: EntryHash,
    destination: AgentPubKey,
}


#[hdk_extern]
fn commit_pending_mail(input: CommitPendingMailInput) -> ExternResult<HeaderHash> {
    debug!("commit_pending_mail() START **********");
    let me = agent_info()?.agent_latest_pubkey;
    /// Commit Pending Mail
    let pending_mail_eh = hash_entry(&input.mail)?;
    let maybe_pending_mail_hh = create_entry(&input.mail);
    if let Err(err) = maybe_pending_mail_hh.clone() {
        debug!("PendingMail create_entry() failed = {:?}", err);
        return Err(maybe_pending_mail_hh.err().unwrap());
    };
    let pending_mail_hh = maybe_pending_mail_hh.unwrap();
    debug!("pending_mail_hh = {:?}", pending_mail_hh);
    /// Commit Pendings Link
    let tag = LinkKind::Pendings.concat_hash(&input.destination);
    debug!("pendings tag = {:?}", tag);
    let maybe_link1_hh = create_link(
        input.outmail_eh.clone(),
        pending_mail_eh.clone(),
        HdkLinkType::Any,
        tag);
    if let Err(err) = maybe_link1_hh.clone() {
        debug!("link1 failed = {:?}", err);
        return Err(maybe_link1_hh.err().unwrap());
    };
    let link1_hh = maybe_link1_hh.unwrap();
    debug!("link1_hh = {}", link1_hh);
    /// Commit MailInbox Link
    let tag = LinkKind::MailInbox.concat_hash(&me);
    let maybe_link2_hh = create_link(
        EntryHash::from(input.destination.clone()),
        pending_mail_eh,
        HdkLinkType::Any,
        tag);
    if let Err(err) = maybe_link2_hh.clone() {
        debug!("link2 failed = {:?}", err);
        return Err(maybe_link2_hh.err().unwrap());
    };
    let link2_hh = maybe_link2_hh.unwrap();
    debug!("link2_hh = {}", link2_hh);
    /// Done
    return Ok(pending_mail_hh)
}


///
pub(crate) fn deliver_mail(
    outmail_eh: &EntryHash,
    mail: &Mail,
    destination: &AgentPubKey,
    manifest_list: &Vec<FileManifest>,
    signature: &Signature,
) -> ExternResult<SendSuccessKind> {
    debug!("deliver_mail() START - {:?}", destination);
    /// Shortcut to self
    let me = agent_info()?.agent_latest_pubkey;
    if destination.clone() == me {
        debug!("deliver_mail() Self");
        let msg = MailMessage {
            outmail_eh: outmail_eh.clone(),
            mail: mail.clone(),
            mail_signature: signature.clone(),
        };
        let inmail = InMail::from_direct(me.clone(), msg);
        debug!("deliver_mail() REMOTE CALLING...");
        let res = call_remote(
            me,
            zome_info()?.name,
            "commit_inmail".to_string().into(),
            None,
            inmail,
        )?;
        debug!("commit_inmail() END : {:?}", res);
        assert!(matches!(res, ZomeCallResponse::Ok { .. }));
        return Ok(SendSuccessKind::OK_SELF);
    }
    /// Try sending directly to other Agent if Online
    let result = deliver_mail_by_dm(outmail_eh, mail, destination, manifest_list, signature);
    if result.is_ok() {
        return Ok(SendSuccessKind::OK_DIRECT);
    } else {
        let err = result.err().unwrap();
        debug!("send_mail_by_dm() failed: {:?}", err);
    }

    debug!("deliver_mail() - Creating pending_mail...");
    /// DM failed, send to DHT instead by creating a PendingMail
    /// Create and commit PendingMail with remote call to self
    let pending_mail = PendingMail::from_mail(
        mail.clone(),
        outmail_eh.clone(),
        destination.clone(),
    )?;
    let payload = CommitPendingMailInput {
        mail: pending_mail,
        outmail_eh: outmail_eh.clone(),
        destination: destination.clone(),
    };
    debug!("deliver_mail() - calling commit_pending_mail()");
    let response = call_remote(
        me,
        zome_info()?.name,
        "commit_pending_mail".to_string().into(),
        None,
        payload,
    )?;
    debug!("deliver_mail() - commit_pending_mail() response: {:?}", response);
    /// Done
    Ok(SendSuccessKind::OK_PENDING)
}


/// Zone Function
/// Send Mail: Creates and commits OutMail. Files must already be committed.
/// post_commit will try to send directly to each recipient.
#[hdk_extern]
#[snapmail_api]
pub fn send_mail(input: SendMailInput) -> ExternResult<HeaderHash> {
    debug!("Sending mail: {}", input.subject);
    /// Get file manifests from addresses
    let mut file_manifest_list = Vec::new();
    let mut file_manifest_pair_list = Vec::new();
    for manifest_hh in input.manifest_address_list.clone() {
        let manifest_eh = get_eh(manifest_hh.clone())?;
        let manifest = get_manifest(manifest_hh.clone().into())?;
        file_manifest_list.push(manifest.clone());
        file_manifest_pair_list.push((manifest_eh, manifest))
    }
    /// Create and commit OutMail
    let outmail = OutMail::create(
        input.subject,
        input.payload,
        input.reply_of,
        input.to.clone(),
        input.cc.clone(),
        input.bcc.clone(),
        file_manifest_pair_list.clone(),
    );
    let outmail_hh = create_entry(&outmail)?;
    debug!("OutMail created: {:?}", outmail_hh);
    Ok(outmail_hh)
}


/// Once OutMail committed, try to send directly to each recipient.
/// if recipient not online, creates a PendingMail on the DHT.
pub fn send_committed_mail(
    outmail_eh: &EntryHash,
    outmail: OutMail,
    whitelist: Option<Vec<AgentPubKey>>) -> ExternResult<()> {
    debug!("CALLED send_committed_mail() {:?}", outmail_eh);
    /// Get filtered recipients
    let recipients = match whitelist {
        None => outmail.recipients(),
        Some(list) => {
            outmail.recipients().iter()
               .filter(|x| list.contains(x))
               .cloned()
               .collect()
        }
    };
    /// Get all attachments manifests
    let mut file_manifest_list = Vec::new();
    for attachment in outmail.mail.attachments.clone() {
        let manifest = get_manifest(attachment.manifest_eh.into())?;
        file_manifest_list.push(manifest.clone());
    }
    /// Create signature
    let signature = sign_mail(&outmail.mail)?;
    /// Send to each recipient
    for agent in recipients {
        let res = deliver_mail(outmail_eh, &outmail.mail, &agent, &file_manifest_list, &signature);
        match res {
            // Create 'Sent' link when successfully sent via DM
            Ok(SendSuccessKind::OK_SELF | SendSuccessKind::OK_DIRECT) => {
                let confirmation = DeliveryConfirmation::new(outmail_eh.clone(), agent.clone());
                let response = call_remote(
                    agent_info()?.agent_latest_pubkey,
                    zome_info()?.name,
                    "commit_confirmation".to_string().into(),
                    None,
                    confirmation,
                )?; // Can't fallback if this fails. Must notify the error.
                debug!("commit_confirmation() response: {:?}", response);
            },
            Ok(_) => {},
            Err(e) => {
                debug!("send_mail_to() failed: {}", e);
            }
        }
    }
    /// Done
    Ok(())
}

//
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct CommitSentsLinkInput {
//     pub outmail_eh: EntryHash,
//     pub to: AgentPubKey,
// }
//
// /// Create & Commit 'Sent' link
// /// Return HeaderHash of newly created link
// #[hdk_extern]
// fn commit_sents_link(input: CommitSentsLinkInput) -> ExternResult<HeaderHash> {
//     debug!("commit_sents_link(): {:?} ", input);
//     let tag = LinkKind::Sents.concat_hash(&input.to);
//     let hh = create_link(input.outmail_eh.clone(), input.outmail_eh, HdkLinkType::Any, tag)?;
//     Ok(hh)
// }
