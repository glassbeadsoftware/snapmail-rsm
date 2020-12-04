use hdk3::prelude::*;

use std::collections::HashMap;

use crate::{
    utils::*,
    send_dm,
    mail::entries::{PendingMail, ReceipientKind, Mail, OutMail},
    dm_protocol::{
        MailMessage, DirectMessageProtocol,
    },
    LinkKind,
    //file::{FileManifest, FileChunk, get_manifest},
};

#[allow(non_camel_case_types)]
pub enum SendSuccessKind {
    OK_DIRECT,
    OK_PENDING(HeaderHash),
}

/// Struct holding all result data from a send request
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct SendMailOutput {
    outmail: HeaderHash,
    to_pendings: HashMap<AgentPubKey, HeaderHash>,
    cc_pendings: HashMap<AgentPubKey, HeaderHash>,
    bcc_pendings: HashMap<AgentPubKey, HeaderHash>,
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
        match kind {
            ReceipientKind::TO => self.to_pendings.insert(agent_id.clone(), hh),
            ReceipientKind::CC => self.cc_pendings.insert(agent_id.clone(), hh),
            ReceipientKind::BCC => self.bcc_pendings.insert(agent_id.clone(), hh),
        };
    }
}

// FIXME
// fn send_manifest_by_dm(
//     destination: &AgentAddress,
//     sender_manifest: &FileManifest,
//     chunk_address_list: Vec<Address>,
// ) -> ExternResult<Address> {
//     debug!(format!("send_manifest_by_dm(): {:?}", destination)).ok();
//
//     // Create receiver manifest
//     let mut receiver_manifest = sender_manifest.clone();
//     receiver_manifest.chunks = chunk_address_list;
//     //   Create DM
//     let payload = serde_json::to_string(&DirectMessageProtocol::FileManifest(receiver_manifest)).unwrap();
//     //   Send DM
//     let result = hdk::send(
//         destination.clone(),
//         payload,
//         Timeout::new(crate::DIRECT_SEND_TIMEOUT_MS),
//     );
//     debug!(format!("send_manifest result = {:?}", result)).ok();
//     //   Check Response
//     if let Err(_e) = result {
//         return Err(ZomeApiError::Internal("hdk::send() of manifest failed".into()))
//     }
//     let response = result.unwrap();
//     debug!(format!("Received response: {:?}", response)).ok();
//     let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
//     if let Err(_e) = maybe_msg {
//         return Err(ZomeApiError::Internal("hdk::send() of manifest failed 2".into()))
//     }
//     // Return manifest's entry address on receiver's source chain
//     match maybe_msg.unwrap() {
//         DirectMessageProtocol::Success(manifest_address) => Ok(manifest_address.into()),
//         _ => Err(ZomeApiError::Internal("hdk::send() of manifest failed 3".into())),
//     }
// }

// FIXME
// fn send_chunk_by_dm(destination: &AgentAddress, chunk_address: &Address) -> ExternResult<Address> {
//     debug!(format!("send_chunk_by_dm(): {}", chunk_address)).ok();
//     let maybe_entry = hdk::get_entry(&chunk_address)?;
//         //.expect("No reason for get_entry() to crash");
//     debug!(format!("maybe_entry =  {:?}", maybe_entry)).ok();
//     if maybe_entry.is_none() {
//         return Err(ZomeApiError::Internal("No chunk found at given address".into()))
//     }
//     let chunk = crate::into_typed::<FileChunk>(maybe_entry.unwrap())?;
//
//     //   Create DM
//     let payload = serde_json::to_string(&DirectMessageProtocol::Chunk(chunk)).unwrap();
//     //   Send DM
//     let result = hdk::send(
//         destination.clone(),
//         payload,
//         Timeout::new(crate::DIRECT_SEND_CHUNK_TIMEOUT_MS),
//     );
//     debug!(format!("send_chunk result = {:?}", result)).ok();
//     //   Check Response
//     if let Err(e) = result {
//         return Err(ZomeApiError::Internal(format!("hdk::send() of chunk failed: {}", e)));
//     }
//     let response = result.unwrap();
//     debug!(format!("Received response: {:?}", response)).ok();
//     let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
//     if let Err(_e) = maybe_msg {
//         return Err(ZomeApiError::Internal("hdk::send() of chunk failed 2".into()))
//     }
//     match maybe_msg.unwrap() {
//         DirectMessageProtocol::Success(chunk_address) => Ok(chunk_address.into()),
//         _ => Err(ZomeApiError::Internal("hdk::send() of chunk failed 3".into())),
//     }
// }

// FIXME
// fn send_attachment_by_dm(destination: &AgentAddress, manifest: &FileManifest) -> ExternResult<Address> {
//     // Send each chunk and receive chunk entry address in return
//     let mut chunk_address_list: Vec<Address> = Vec::new();
//     for chunk_address in &manifest.chunks {
//         let receiver_chunk_address = send_chunk_by_dm(destination, chunk_address)?;
//         chunk_address_list.push(receiver_chunk_address);
//     }
//     // Create and Send FileManifest
//     return send_manifest_by_dm(destination, manifest, chunk_address_list);
// }


// FIXME use post-commit callback to send the mail via DM

/// Attempt sending Mail and attachments via Direct Messaging
fn send_mail_by_dm(
    outmail_eh: &EntryHash,
    mail: &Mail,
    destination: &AgentPubKey,
    //manifest_list: &Vec<FileManifest>,
) -> ExternResult<()> {

    /// -- Send Attachments
    // FIXME
    // debug!("Send Attachments".to_string()).ok();
    // // For each attachment, send all the chunks
    // let mut manifest_address_list: Vec<Address> = Vec::new();
    // for manifest in manifest_list {
    //     let maybe_manifest_address = send_attachment_by_dm(destination, manifest);
    //     if let Err(e) = maybe_manifest_address {
    //         let err_msg = format!("Send attachment failed -> Err: {}", e);
    //         debug!(err_msg.clone()).ok();
    //         return Err(ZomeApiError::Internal(err_msg));
    //     }
    //     manifest_address_list.push(maybe_manifest_address.unwrap());
    // }

    /// --  Send Mail
    debug!("send_mail_by_dm()").ok();
    /// Create DM
    let msg = MailMessage {
        outmail_eh: outmail_eh.clone(),
        mail: mail.clone(),
        //manifest_address_list,
    };
    //let payload = serde_json::to_string(&DirectMessageProtocol::Mail(msg)).unwrap();

    /// Send DM
    let response_dm = send_dm(destination.clone(), DirectMessageProtocol::Mail(msg))?;
    debug!("send_mail_to() response_dm = {:?}", response_dm).ok();

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
    //manifest_list: &Vec<FileManifest>,
) -> ExternResult<SendSuccessKind> {
    debug!("send_mail_to() START - {}", destination).ok();
    /// First try sending directly to other Agent if Online
    let result = send_mail_by_dm(outmail_eh, mail, destination/*, manifest_list*/);
    if result.is_ok() {
        return Ok(SendSuccessKind::OK_DIRECT);
    } else {
        let err = result.err().unwrap();
        debug!("send_mail_by_dm() failed: {:?}", err).ok();
    }

    // FIXME
    //return Ok(SendSuccessKind::OK_PENDING(outmail_eh.clone()));

    /// DM failed, send to DHT instead by creating a PendingMail

    // /// Get Handle address first
    // debug!("Sending mail by DM failed. Getting handle for... {}", destination).ok();
    // let maybe_destination_element = crate::handle::get_handle_element(destination);
    // if let None = maybe_destination_element {
    //     debug!("No handle has been set for receiving agent").ok();
    //     return error("No handle has been set for receiving agent");
    // }
    // let destination_element = maybe_destination_element.unwrap();
    // let my_handle_address = get_eh(&my_handle_element)?;
    // debug!("destination_element: {}", destination_element).ok();

    /// Commit PendingMail
    let pending_mail = PendingMail::new(mail.clone(), outmail_eh.clone());
    let pending_mail_eh = hash_entry(&pending_mail)?;
    let maybe_pending_mail_hh = create_entry(&pending_mail);
    if let Err(err) = maybe_pending_mail_hh.clone() {
        debug!("PendingMail create_entry() failed = {:?}", err).ok();
        return Err(maybe_pending_mail_hh.err().unwrap());
    };
    let pending_mail_hh = maybe_pending_mail_hh.unwrap();
    debug!("pending_mail_hh = {}", pending_mail_hh).ok();
    /// Commit Pendings Link
    //let recepient = format!("{}", original_sender);
    let tag = LinkKind::Pendings.concat_hash(&pending_mail_hh);
    let maybe_link1_hh = create_link(outmail_eh.clone(), pending_mail_eh.clone(), tag);
    if let Err(err) = maybe_link1_hh.clone() {
        debug!("link1 failed = {:?}", err).ok();
        return Err(maybe_link1_hh.err().unwrap());
    };
    let link1_hh = maybe_link1_hh.unwrap();
    debug!("link1_hh = {}", link1_hh).ok();
    /// Commit MailInbox Link
    //let from = format!("{}", agent_info()?.agent_latest_pubkey);
    let from = agent_info()?.agent_latest_pubkey;
    let tag = LinkKind::MailInbox.concat_hash(&from);
    let maybe_link2_hh = create_link(EntryHash::from(destination.clone()), pending_mail_eh, tag);
    if let Err(err) = maybe_link2_hh.clone() {
        debug!("link2 failed = {:?}", err).ok();
        return Err(maybe_link2_hh.err().unwrap());
    };
    let link2_hh = maybe_link2_hh.unwrap();
    debug!(format!("link2_hh = {}", link2_hh)).ok();
    /// Done
    Ok(SendSuccessKind::OK_PENDING(pending_mail_hh))
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct SendMailInput {
    pub subject: String,
    pub payload: String,
    pub to: Vec<AgentPubKey>,
    pub cc: Vec<AgentPubKey>,
    pub bcc: Vec<AgentPubKey>,
    //pub manifest_address_list: Vec<HeaderHash>,
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient.
/// if receipient not online, creates a PendingMail on the DHT.
#[hdk_extern]
pub fn send_mail(
    input: SendMailInput
) -> ExternResult<SendMailOutput> {

    debug!("Sending mail: {}", input.subject).ok();

    /// Get file manifests from addresses
    // // FIXME
    //let mut file_manifest_list = Vec::new();
    //let mut file_manifest_pair_list = Vec::new();
    // for manifest_address in manifest_address_list.clone() {
    //     let manifest = get_manifest(manifest_address.clone())?;
    //     file_manifest_list.push(manifest.clone());
    //     file_manifest_pair_list.push((manifest_address.clone(), manifest))
    // }

    /// Create and commit OutMail
    let outmail = OutMail::create(
        input.subject,
        input.payload,
        input.to.clone(),
        input.cc.clone(),
        input.bcc.clone(),
//        input.file_manifest_pair_list.clone(),
    );
    //let outmail_entry = Entry::App(entry_kind::OutMail.into(), outmail.clone().into());
    let outmail_hh = create_entry(&outmail)?;
    let outmail_eh = hash_entry(&outmail)?;
    debug!("OutMail created: {:?}", outmail_hh).ok();

    /// Send to each recepient
    let mut total_result = SendMailOutput::new(outmail_hh.clone());
    /// to
    for agent in input.to {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, /*&file_manifest_list*/);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::TO, &agent, pending_hh);
        }
    }
    /// cc
    for agent in input.cc {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, /*&file_manifest_list*/);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::CC, &agent, pending_hh);
        }
    }
    /// bcc
    for agent in input.bcc {
        let res = send_mail_to(&outmail_eh, &outmail.mail, &agent, /*&file_manifest_list*/);
        if let Ok(SendSuccessKind::OK_PENDING(pending_hh)) = res {
            total_result.add_pending(ReceipientKind::BCC, &agent, pending_hh);
        }
    }

    /// Done
    debug!("total_result: {:?}", total_result).ok();
    Ok(total_result)
}