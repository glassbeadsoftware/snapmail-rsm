use hdk::prelude::*;

use crate::{
    link_kind::*,
    dm_protocol::{DirectMessageProtocol, AckMessage},
    mail::{
        receive::*,
        entries::{
            InMail, PendingAck, OutAck,
        },
    },
    send_dm,
    utils::*,
};


/// Zome function
/// DEBUG
#[hdk_extern]
pub fn create_outack(_:()) -> ExternResult<HeaderHash> {
    let outack = OutAck::new();
    debug!("create_outack() called");
    let outack_hh = create_entry(&outack)?;
    debug!("create_outack() done");
    Ok(outack_hh)
}

/// Zome function
/// Return EntryHash of newly created OutAck
#[hdk_extern]
#[cfg_attr(not(target_arch = "wasm32"), snapmail_api)]
pub fn acknowledge_mail(inmail_hh: HeaderHash) -> ExternResult<EntryHash> {
    /// Make sure its an InMail ...
    let (inmail_eh, inmail) = get_typed_from_hh::<InMail>(inmail_hh.clone())?;
    /// ... has not already been acknowledged
    let res = get_links(inmail_eh.clone(), LinkKind::Acknowledgment.as_tag_opt())?.into_inner();
    if res.len() > 0 {
        return error("Mail has already been acknowledged");
    }
    debug!("Not acknowledged yet");

    /// Write OutAck
    let outack = OutAck::new();
    let outack_hh = create_entry(&outack)?;
    let outack_eh = hh_to_eh(outack_hh)?;
    debug!("Creating ack link...");
    let _ = create_link(inmail_eh, outack_eh.clone(), LinkKind::Acknowledgment.as_tag())?;

    /// Shortcut to self
    let me = agent_info()?.agent_latest_pubkey;
    if inmail.from.clone() == me {
        debug!("send ack to Self");
        let msg = AckMessage {
            outmail_eh: inmail.outmail_eh.clone(),
        };
        let res = receive_dm_ack(me, msg);
        assert!(res == DirectMessageProtocol::Success("Ack received".to_string()));
        return Ok(outack_eh);
    }

    /// Try Direct sharing of Acknowledgment
    debug!("Sending ack via DM ...");
    let res = send_dm_ack(&inmail.outmail_eh, &inmail.from);
    if res.is_ok() {
        debug!("Acknowledgment shared !");
        return Ok(outack_eh);
    }
    /// Otherwise share Acknowledgement via DHT
    let err = res.err().unwrap();
    debug!("Direct sharing of Acknowledgment failed: {}", err);
    let _ = acknowledge_mail_pending(&outack_eh, &inmail.outmail_eh, &inmail.from)?;

    /// Done
    Ok(outack_eh)
}

/// Try sending directly to other Agent if Online
fn send_dm_ack(outmail_eh: &EntryHash, from: &AgentPubKey) -> ExternResult<()> {
    debug!("acknowledge_mail_direct() START");
    /// Create DM
    let msg = AckMessage { outmail_eh: outmail_eh.clone() };
    /// Send DM
    let response = send_dm(from.clone(), DirectMessageProtocol::Ack(msg))?;
    /// Check Response
    debug!("Received response for Ack: {:?}", response);
    match response {
        DirectMessageProtocol::Success(_) => Ok(()),
        _ => error("ACK by DM Failed"),
    }
}

/// Create & Commit PendingAck
/// Return HeaderHash of newly created PendingAck
fn acknowledge_mail_pending(
    outack_eh: &EntryHash,
    outmail_eh: &EntryHash,
    original_sender: &AgentPubKey,
) -> ExternResult<HeaderHash> {
    /// Commit PendingAck
    let pending_ack = PendingAck::new(outmail_eh.clone());
    let pending_ack_hh = create_entry(&pending_ack)?;
    /// Create links between PendingAck and Outack & recepient inbox
    let pending_ack_eh = hash_entry(&pending_ack)?;
    let tag = LinkKind::AckInbox.concat_hash(original_sender);
    let _ = create_link(outack_eh.clone(), pending_ack_eh.clone(), LinkKind::Pending.as_tag())?;
    let _ = create_link(EntryHash::from(original_sender.clone()), pending_ack_eh, tag)?;
    debug!("pending_ack_hh: {:?} (for {})", pending_ack_hh, original_sender);
    /// Done
    Ok(pending_ack_hh)
}
