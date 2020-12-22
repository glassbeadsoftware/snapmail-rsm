use hdk3::prelude::*;

use crate::{
    utils::*,
    //dm::*,
    entry_kind, signal_protocol::*,
    //file::{FileChunk, FileManifest},
    mail::{
        self,
        entries::{
            InMail,
            InMailState,
            MailItem, MailState,
        },
    },
    DirectMessageProtocol, MailMessage, AckMessage,
    ReceivedAck, snapmail_now,
};

///
pub fn receive_dm(from: AgentPubKey, dm: DirectMessageProtocol) -> DirectMessageProtocol {
    debug!("Received from: {}", from);
    // let maybe_msg: Result<DirectMessageProtocol, _> = msg_json.try_into();
    // if let Err(err) = maybe_msg {
    //     return format!("error: {}", err);
    // }
    match dm {
        // DirectMessageProtocol::Chunk(chunk) => {
        //     mail::receive_direct_chunk(from, chunk)
        // },
        // DirectMessageProtocol::FileManifest(manifest) => {
        //     mail::receive_direct_manifest(from, manifest)
        // },
        DirectMessageProtocol::Mail(mail) => {
            mail::receive_dm_mail(from, mail)
        },
        DirectMessageProtocol::Ack(ack) => {
            mail::receive_dm_ack(from, ack)
            //DirectMessageProtocol::Success(String::new())
        },
        // DirectMessageProtocol::RequestChunk(address) => {
        //     mail::receive_direct_request_chunk(from, address)
        // },
        // DirectMessageProtocol::RequestManifest(address) => {
        //     mail::receive_direct_request_manifest(from, address)
        // },
        DirectMessageProtocol::Ping => {
            DirectMessageProtocol::Success(String::new())
        },
        _ => {
             DirectMessageProtocol::Failure("Unexpected protocol".to_owned())
        },
    }
}

/*
/// Handle a RequestFileManifestMessage.
/// TODO: Emits `received_request_manifest` signal.
/// Returns FileManifest, UnknownEntry or Failure.
pub fn receive_direct_request_manifest(from: AgentAddress, manifest_address: Address) -> DirectMessageProtocol {
    debug!(format!("received request manifest from: {}", from));
    let maybe_maybe_entry = hdk::get_entry(&manifest_address);
    if let Err(err) = maybe_maybe_entry {
        let response_str = "Failed on get_entry()";
        debug!(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let maybe_entry = maybe_maybe_entry.unwrap();
    if let None = maybe_entry {
        return DirectMessageProtocol::UnknownEntry;
    }
    debug!(format!("Sending manifest: {}", manifest_address));
    let maybe_manifest = crate::into_typed::<FileManifest>(maybe_entry.unwrap());
    if let Err(_err) = maybe_manifest {
        let response_str = "Requested entry is not a FileManifest";
        debug!(format!("{}", response_str));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    // Return Success response
    return DirectMessageProtocol::FileManifest(maybe_manifest.unwrap());
}

/// Handle a FileManifestMessage.
/// Emits `received_manifest` signal.
/// Returns Success or Failure.
pub fn receive_direct_manifest(from: AgentAddress, manifest: FileManifest) -> DirectMessageProtocol {
    debug!(format!("received manifest from: {}", from));
    // FIXME: Check if already have file?
    // Commit FileManifest
    let manifest_entry = Entry::App(entry_kind::FileManifest.into(), manifest.into());
    let maybe_address = hdk::commit_entry(&manifest_entry);
    if let Err(err) = maybe_address {
        let response_str = "Failed committing FileManifest";
        debug!(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let manifest_address = maybe_address.unwrap();
    debug!(format!("received manifest_address: {}", manifest_address));
    // FIXME: emit signal
    // Return Success response
    return DirectMessageProtocol::Success(manifest_address.into());
}

/// Handle a RequestFileChunkMessage.
/// Emits `received_request_chunk` signal.
/// Returns FileChunk, UnknownEntry or Failure.
pub fn receive_direct_request_chunk(from: AgentAddress, chunk_address: Address) -> DirectMessageProtocol {
    debug!(format!("received request chunk from: {}", from));
    // FIXME: emit signal
    let maybe_maybe_entry = hdk::get_entry(&chunk_address);
    if let Err(err) = maybe_maybe_entry {
        let response_str = "Failed on get_entry()";
        debug!(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let maybe_entry = maybe_maybe_entry.unwrap();
    if let None = maybe_entry {
        return DirectMessageProtocol::UnknownEntry;
    }
    debug!(format!("Sending chunk: {}", chunk_address));
    let maybe_chunk = crate::into_typed::<FileChunk>(maybe_entry.unwrap());
    if let Err(_err) = maybe_chunk {
        let response_str = "Requested entry is not a FileChunk";
        debug!(format!("{}", response_str));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    // Return Success response
    return DirectMessageProtocol::Chunk(maybe_chunk.unwrap());
}

/// Handle a ChunkMessage.
/// Emits `received_chunk` signal.
/// Returns Success or Failure.
pub fn receive_direct_chunk(_from: AgentAddress, chunk: FileChunk) -> DirectMessageProtocol {
    // FIXME: Check if already have chunk?
    // Commit FileChunk
    let chunk_entry = Entry::App(entry_kind::FileChunk.into(), chunk.into());
    let maybe_address = hdk::commit_entry(&chunk_entry);
    if let Err(err) = maybe_address {
        let response_str = "Failed committing FileChunk";
        debug!(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let chunk_address = maybe_address.unwrap();
    debug!(format!("received chunk_address: {}",  chunk_address));
    // FIXME: emit signal
    // Return Success response
    return DirectMessageProtocol::Success(chunk_address.into());
}
*/

/// Handle a MailMessage.
/// Emits `received_mail` signal.
/// Returns Success or Failure.
pub fn receive_dm_mail(from: AgentPubKey, mail_msg: MailMessage) -> DirectMessageProtocol {
    /// Create InMail
    let inmail = InMail::from_direct(from.clone(), mail_msg.clone());
    /// Commit InMail
    let maybe_inmail_hh = create_entry(&inmail);
    if let Err(err) = maybe_inmail_hh {
        let response_str = "Failed committing InMail";
        debug!(format!("{}: {}", response_str, err));
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let inmail_hh =  maybe_inmail_hh.unwrap();
    debug!(format!("inmail_address: {:?}", inmail_hh));
    /// Emit signal
    let item = MailItem {
        address: inmail_hh,
        author: from.clone(),
        mail: mail_msg.mail.clone(),
        state: MailState::In(InMailState::Arrived),
        bcc: Vec::new(),
        date: snapmail_now() as i64, // FIXME
    };
    let res = emit_signal(&SignalProtocol::ReceivedMail(item));
    if let Err(err) = res {
        debug!("Emit signal failed: {}", err);
    }
    /// Return Success response
    return DirectMessageProtocol::Success("Mail received".to_string());
}

/// Handle a AckMessage.
/// Emits `received_ack` signal.
/// Returns Success or Failure.
pub fn receive_dm_ack(from: AgentPubKey, ack_msg: AckMessage) -> DirectMessageProtocol {
    debug!("receive_dm_ack() from: {:?} ; for {:?}", from, ack_msg.outmail_eh);

    /// Check if we have acked outmail
    let maybe_outmail = get_local_eh(ack_msg.outmail_eh.clone());
    if let Err(err) = maybe_outmail {
        let response_str = "Failed to find OutMail from Ack";
        debug!("{}: {}", response_str, err);
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    let outmail_hh = maybe_outmail.unwrap().header_address().clone();
    /// Create InAck
    let outmail_eh = ack_msg.outmail_eh.clone();
    debug!("outmail_eh = {:?}", outmail_eh);
    let res = mail::commit_inack(outmail_eh, &from);
    if let Err(err) = res {
        let response_str = "Failed committing InAck";
        debug!("{}: {}", response_str, err);
        return DirectMessageProtocol::Failure(response_str.to_string());
    }
    /// Emit Signal
    let signal = SignalProtocol::ReceivedAck(ReceivedAck {
        from: from.clone(),
        for_mail: outmail_hh,
    });
    let res = emit_signal(&signal);
    if let Err(err) = res {
        debug!(format!("Emit signal failed: {}", err));
    }
    /// Return Success response
    debug!("receive_direct_ack() success!");
    return DirectMessageProtocol::Success("Ack received".to_string());
}
