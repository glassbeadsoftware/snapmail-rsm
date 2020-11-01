use hdk3::prelude::*;

/*
use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        time::Timeout,
    },
};
*/

use std::collections::HashMap;

use crate::{
    link_kind, entry_kind,
    mail::entries::{PendingMail, ReceipientKind, Mail, OutMail},
    protocol::{
        MailMessage, DirectMessageProtocol,
    },
    file::{FileManifest, FileChunk},
};
use crate::file::get_manifest;


#[allow(non_camel_case_types)]
pub enum SendSuccessKind {
    OK_DIRECT,
    OK_PENDING(Address),
}

/// Struct holding all result data from a send request
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct SendTotalResult {
    outmail: Address,
    to_pendings: HashMap<AgentAddress, Address>,
    cc_pendings: HashMap<AgentAddress, Address>,
    bcc_pendings: HashMap<AgentAddress, Address>,
}

impl SendTotalResult {
    pub fn new(outmail_address: Address) -> Self {
        Self {
            outmail: outmail_address,
            to_pendings: HashMap::new(),
            cc_pendings: HashMap::new(),
            bcc_pendings: HashMap::new(),
        }
    }

    pub fn add_pending(&mut self, kind: ReceipientKind, agentId: &AgentAddress, address: Address) {
        match kind {
            ReceipientKind::TO => self.to_pendings.insert(agentId.clone(), address),
            ReceipientKind::CC => self.cc_pendings.insert(agentId.clone(), address),
            ReceipientKind::BCC => self.bcc_pendings.insert(agentId.clone(), address),
        };
    }
}

fn send_manifest_by_dm(
    destination: &AgentAddress,
    sender_manifest: &FileManifest,
    chunk_address_list: Vec<Address>,
) -> ZomeApiResult<Address> {
    debug!(format!("send_manifest_by_dm(): {:?}", destination)).ok();

    // Create receiver manifest
    let mut receiver_manifest = sender_manifest.clone();
    receiver_manifest.chunks = chunk_address_list;
    //   Create DM
    let payload = serde_json::to_string(&DirectMessageProtocol::FileManifest(receiver_manifest)).unwrap();
    //   Send DM
    let result = hdk::send(
        destination.clone(),
        payload,
        Timeout::new(crate::DIRECT_SEND_TIMEOUT_MS),
    );
    debug!(format!("send_manifest result = {:?}", result)).ok();
    //   Check Response
    if let Err(_e) = result {
        return Err(ZomeApiError::Internal("hdk::send() of manifest failed".into()))
    }
    let response = result.unwrap();
    debug!(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(_e) = maybe_msg {
        return Err(ZomeApiError::Internal("hdk::send() of manifest failed 2".into()))
    }
    // Return manifest's entry address on receiver's source chain
    match maybe_msg.unwrap() {
        DirectMessageProtocol::Success(manifest_address) => Ok(manifest_address.into()),
        _ => Err(ZomeApiError::Internal("hdk::send() of manifest failed 3".into())),
    }
}

fn send_chunk_by_dm(destination: &AgentAddress, chunk_address: &Address) -> ZomeApiResult<Address> {
    debug!(format!("send_chunk_by_dm(): {}", chunk_address)).ok();
    let maybe_entry = hdk::get_entry(&chunk_address)?;
        //.expect("No reason for get_entry() to crash");
    debug!(format!("maybe_entry =  {:?}", maybe_entry)).ok();
    if maybe_entry.is_none() {
        return Err(ZomeApiError::Internal("No chunk found at given address".into()))
    }
    let chunk = crate::into_typed::<FileChunk>(maybe_entry.unwrap())?;

    //   Create DM
    let payload = serde_json::to_string(&DirectMessageProtocol::Chunk(chunk)).unwrap();
    //   Send DM
    let result = hdk::send(
        destination.clone(),
        payload,
        Timeout::new(crate::DIRECT_SEND_CHUNK_TIMEOUT_MS),
    );
    debug!(format!("send_chunk result = {:?}", result)).ok();
    //   Check Response
    if let Err(e) = result {
        return Err(ZomeApiError::Internal(format!("hdk::send() of chunk failed: {}", e)));
    }
    let response = result.unwrap();
    debug!(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(_e) = maybe_msg {
        return Err(ZomeApiError::Internal("hdk::send() of chunk failed 2".into()))
    }
    match maybe_msg.unwrap() {
        DirectMessageProtocol::Success(chunk_address) => Ok(chunk_address.into()),
        _ => Err(ZomeApiError::Internal("hdk::send() of chunk failed 3".into())),
    }
}

fn send_attachment_by_dm(destination: &AgentAddress, manifest: &FileManifest) -> ZomeApiResult<Address> {
    // Send each chunk and receive chunk entry address in return
    let mut chunk_address_list: Vec<Address> = Vec::new();
    for chunk_address in &manifest.chunks {
        let receiver_chunk_address = send_chunk_by_dm(destination, chunk_address)?;
        chunk_address_list.push(receiver_chunk_address);
    }
    // Create and Send FileManifest
    return send_manifest_by_dm(destination, manifest, chunk_address_list);
}

/// Attempt sending Mail and attachments via Direct Messaging
fn send_mail_by_dm(
    outmail_address: &Address,
    mail: &Mail,
    destination: &AgentAddress,
    manifest_list: &Vec<FileManifest>,
) -> ZomeApiResult<()> {

    // -- Send Attachments
    debug!("Send Attachments".to_string()).ok();
    // For each attachment, send all the chunks
    let mut manifest_address_list: Vec<Address> = Vec::new();
    for manifest in manifest_list {
        let maybe_manifest_address = send_attachment_by_dm(destination, manifest);
        if let Err(e) = maybe_manifest_address {
            let err_msg = format!("Send attachment failed -> Err: {}", e);
            debug!(err_msg.clone()).ok();
            return Err(ZomeApiError::Internal(err_msg));
        }
        manifest_address_list.push(maybe_manifest_address.unwrap());
    }

    // --  Send Mail
    debug!("Send Mail".to_string()).ok();
    // Create DM
    let msg = MailMessage {
        outmail_address: outmail_address.clone(),
        mail: mail.clone(),
        manifest_address_list,
    };
    let payload = serde_json::to_string(&DirectMessageProtocol::Mail(msg)).unwrap();
    //   Send DM
    let result = hdk::send(
        destination.clone(),
        payload,
        Timeout::new(crate::DIRECT_SEND_TIMEOUT_MS),
    );
    debug!(format!("send_mail_to() result = {:?}", result)).ok();
    //   Check Response
    if let Ok(response) = result.clone() {
        debug!(format!("Received response: {:?}", response)).ok();
        let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
        if let Ok(msg) = maybe_msg {
            if let DirectMessageProtocol::Success(_) = msg {
                return Ok(());
            }
        }
    };
    Err(ZomeApiError::Internal(format!("send() failed: {:?}", result)))
}

///
fn send_mail_to(
    outmail_address: &Address,
    mail: &Mail,
    destination: &AgentAddress,
    manifest_list: &Vec<FileManifest>,
) -> ZomeApiResult<SendSuccessKind> {

    debug!(format!("sending mail to... {}", destination)).ok();

    // 1. First try sending directly to other Agent if Online
    let result = send_mail_by_dm(outmail_address, mail, destination, manifest_list);
    if result.is_ok() {
        return Ok(SendSuccessKind::OK_DIRECT);
    } else {
        let err = result.err().unwrap();
        debug!(format!("send_mail_by_dm() failed: {:?}", err)).ok();
    }

    // -- Send to DHT -- //

    // 2. Direct Send failed, so send to DHT instead by creating a PendingMail
    // Get Handle address first
    debug!(format!("Sending mail by DM failed. Getting handle for... {}", destination)).ok();
    let maybe_destination_handle_address = crate::handle::get_handle_entry(destination);
    if let None = maybe_destination_handle_address {
        debug!("No handle has been set for receiving agent").ok();
        return Err(ZomeApiError::Internal("No handle has been set for receiving agent".to_string()));
    }
    let destination_handle_address = maybe_destination_handle_address.unwrap().0;
    debug!(format!("destination_handle_address: {}", destination_handle_address)).ok();

    //    a. Commit PendingMail
    let pending = PendingMail::new(mail.clone(), outmail_address.clone());
    let pending_entry = Entry::App(entry_kind::PendingMail.into(), pending.into());
    let pending_address_maybe = hdk::commit_entry(&pending_entry);
    if let Err(err) = pending_address_maybe.clone() {
        debug!(format!("pending_mail commit failed = {:?}", err)).ok();
        return Err(pending_address_maybe.err().unwrap());
    };
    let pending_address = pending_address_maybe.unwrap();
    debug!(format!("pending_mail pending_address = {}", pending_address)).ok();
    //    a. Commit Pendings Link
    let link1_address_maybe = hdk::link_entries(&outmail_address, &pending_address, link_kind::Pendings, &pending_address.to_string());
    if let Err(err) = link1_address_maybe.clone() {
        debug!(format!("link1 failed = {:?}", err)).ok();
        return Err(link1_address_maybe.err().unwrap());
    };
    let link1_address = link1_address_maybe.unwrap();
    debug!(format!("link1_address = {}", link1_address)).ok();
    //    a. Commit MailInbox Link
    let link2_address_maybe = hdk::link_entries(&destination_handle_address, &pending_address, link_kind::MailInbox, &*hdk::AGENT_ADDRESS.to_string());
    if let Err(err) = link2_address_maybe.clone() {
        debug!(format!("link2 failed = {:?}", err)).ok();
        return Err(link2_address_maybe.err().unwrap());
    };
    let link2_address = link2_address_maybe.unwrap();
    debug!(format!("link2_address = {}", link2_address)).ok();
    // Done
    Ok(SendSuccessKind::OK_PENDING(pending_address))
}

/// Zone Function
/// Send Mail: Creates OutMail, tries to send directly to each receipient.
/// if receipient not online, creates a PendingMail on the DHT.
#[hdk_extern]
pub fn send_mail(
    subject: String,
    payload: String,
    to: Vec<AgentAddress>,
    cc: Vec<AgentAddress>,
    bcc: Vec<AgentAddress>,
    manifest_address_list: Vec<Address>,
) -> ExternResult<SendTotalResult> {

    debug!(format!("Sending mail: {}", subject)).ok();

    // Get file manifests from addresses
    let mut file_manifest_list = Vec::new();
    let mut file_manifest_pair_list = Vec::new();
    for manifest_address in manifest_address_list.clone() {
        let manifest = get_manifest(manifest_address.clone())?;
        file_manifest_list.push(manifest.clone());
        file_manifest_pair_list.push((manifest_address.clone(), manifest))
    }

    // Create and commit OutMail
    let outmail = OutMail::create(
        subject,
        payload,
        to.clone(),
        cc.clone(),
        bcc.clone(),
        file_manifest_pair_list.clone(),
    );
    let outmail_entry = Entry::App(entry_kind::OutMail.into(), outmail.clone().into());
    let outmail_address = hdk::commit_entry(&outmail_entry)?;
    debug!(format!("OutMail created: {}", outmail_address)).ok();

    // Send to each recepient
    let mut total_result = SendTotalResult::new(outmail_address.clone());
    // to
    for agent in to {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::TO, &agent, pending_address);
        }
    }
    // cc
    for agent in cc {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::CC, &agent, pending_address);
        }
    }
    // bcc
    for agent in bcc {
        let res = send_mail_to(&outmail_address, &outmail.mail, &agent, &file_manifest_list);
        if let Ok(SendSuccessKind::OK_PENDING(pending_address)) = res {
            total_result.add_pending(ReceipientKind::BCC, &agent, pending_address);
        }
    }

    // Done
    debug!(format!("total_result: {:?}", total_result)).ok();
    Ok(total_result)
}