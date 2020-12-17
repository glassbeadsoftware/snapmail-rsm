use hdk3::prelude::*;

use crate::{
    link_kind::*,
    dm_protocol::{DirectMessageProtocol, AckMessage},
    mail::{
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
pub fn acknowledge_mail(inmail_hh: HeaderHash) -> ExternResult<EntryHash> {
    /// Make sure its an InMail ...
    let (inmail_eh, inmail) = get_typed_entry::<InMail>(inmail_hh.clone())?;
    /// ... has not already been acknowledged
    let res = get_links(inmail_eh.clone(), LinkKind::Acknowledgment.as_tag_opt())?.into_inner();
    if res.len() > 0 {
        return error("Mail has already been acknowledged");
    }
    debug!("Not acknowledged yet");

    /// Write OutAck
    // let outack_eh = inmail_eh.clone();
    //
    let outack = OutAck::new();
    let outack_hh = create_entry(&outack)?;
    let outack_eh = hh_to_eh(outack_hh)?;
    debug!("Creating ack link...");
    let _ = create_link(inmail_eh, outack_eh.clone(), LinkKind::Acknowledgment.as_tag())?;

    // debug!("ack link DONE ; sending ack via DM ...").ok();
    // /// Try Direct sharing of Acknowledgment
    // let res = send_dm_ack(&inmail.outmail_eh, &inmail.from);
    // if res.is_ok() {
    //     debug!("Acknowledgment shared !").ok();
    //     return Ok(outack_eh);
    // }
    // /// Otherwise share Acknowledgement via DHT
    // let err = res.err().unwrap();
    // debug!("Direct sharing of Acknowledgment failed: {}", err).ok();
    // let _ = acknowledge_mail_pending(&outack_eh, &inmail.outmail_eh, &inmail.from)?;

    /// Done
    Ok(outack_eh)
}

/// Try sending directly to other Agent if Online
fn send_dm_ack(outmail_eh: &EntryHash, from: &AgentPubKey) -> ExternResult<()> {
    debug!("acknowledge_mail_direct() START");
    /// Create DM
    let msg = AckMessage {
        outmail_eh: outmail_eh.clone(),
    };
    //let payload = serde_json::to_string(&DirectMessageProtocol::Ack(msg)).unwrap();
    //let payload: SerializedBytes = DirectMessageProtocol::Ack(msg).try_into().unwrap();
    /// Send DM
    let response = send_dm(from.clone(), DirectMessageProtocol::Ack(msg))?;
    // let result = call_remote!(
    //     from.clone(),
    //     zome_info!()?.zome_name,
    //     "receive".to_string().into(),
    //     None,
    //     payload,
    // );
    // if let Err(err) = result {
    //     return Err(err);
    // }
    // let response = result.unwrap();
    /// Check Response
    debug!("Received response for Ack: {:?}", response);
    // let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    // if let Err(err) = maybe_msg {
    //     debug!(format!("Received response -> Err: {}", err)).ok();
    //     return Err(HdkError::Wasm(WasmError::Zome(format!("{}", err))));
    // }
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
    // /// Get Handle address first
    // let maybe_element = crate::handle::get_handle_element(from);
    // if let None = maybe_element {
    //     return error("No handle has been set for ack receiving agent");
    // }
    // let handle_element = maybe_element.unwrap();
    // let handle_eh = get_eh(&handle_element)?;
    /// Commit PendingAck
    let pending_ack = PendingAck::new(outmail_eh.clone());
    let pending_ack_hh = create_entry(&pending_ack)?;
    /// Create links between PendingAck and Outack & recepient inbox
    let pending_ack_eh = hash_entry(&pending_ack)?;
    //let recepient = format!("{}", original_sender);
    let tag = LinkKind::AckInbox.concat_hash(original_sender);
    let _ = create_link(outack_eh.clone(), pending_ack_eh.clone(), LinkKind::Pending.as_tag())?;
    let _ = create_link(EntryHash::from(original_sender.clone()), pending_ack_eh, tag)?;
    debug!("pending_ack_hh: {:?} (for {})", pending_ack_hh, original_sender);
    /// Done
    Ok(pending_ack_hh)
}
