use hdk3::prelude::*;

use crate::{
    entry_kind,
    link_kind::*,
    protocol::{DirectMessageProtocol, AckMessage},
    mail::{
        utils::*,
        entries::{
            InMail, PendingAck, OutAck,
        },
    },
    utils::*,
    send_dm,
};


/// Zome function
/// DEBUG
#[hdk_extern]
pub fn create_outack(_:()) -> ExternResult<HeaderHash> {
    let outack = OutAck::new();
    debug!("create_outack() called").ok();
    let outack_hh = create_entry(&outack)?;
    debug!("create_outack() done").ok();
    Ok(outack_hh)
}

/// Zome function
/// Return address of newly created OutAck
#[hdk_extern]
pub fn acknowledge_mail(inmail_hh: HeaderHash) -> ExternResult<EntryHash> {
    ///  1. Make sure its an InMail
    let (inmail_eh, inmail) = get_typed_entry::<InMail>(inmail_hh.clone())?;
    ///  2. Make sure it has not already been acknowledged
    let res = get_links(inmail_eh.clone(), LinkKind::Acknowledgment.as_tag_opt())?.into_inner();
    if res.len() > 0 {
        return error("Mail has already been acknowledged");
    }
    debug!("No Acknowledgment yet").ok();
    /// 3. Write OutAck
    let outack = OutAck::new();
    let outack_hh = create_entry(&outack)?;
    let outack_eh = hh_to_eh(outack_hh)?;
    debug!("Creating ack link...").ok();
    let _ = create_link(inmail_eh, outack_eh.clone(), LinkKind::Acknowledgment.as_tag())?;
    /// 4. Try Direct sharing of Acknowledgment
    let res = acknowledge_mail_direct(&inmail.outmail_address, &inmail.from);
    if res.is_ok() {
        debug!("Acknowledgment shared !").ok();
        return Ok(outack_eh);
    }
    let err = res.err().unwrap();
    debug!("Direct sharing of Acknowledgment failed: {}", err).ok();
    /// 5. Otherwise share Acknowledgement via DHT
    // FIXME
    //let _ = acknowledge_mail_pending(&outack_address, &inmail.outmail_address, &inmail.from)?;
    Ok(outack_eh)
}

/// Try sending directly to other Agent if Online
fn acknowledge_mail_direct(outmail_hh: &HeaderHash, from: &AgentPubKey) -> ExternResult<()> {
    debug!("acknowledge_mail_direct() START").ok();
    /// Create DM
    let msg = AckMessage {
        outmail_address: outmail_hh.clone(),
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
    debug!("Received response for Ack: {:?}", response).ok();
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
//
// /// Create & Commit PendingAck
// /// Return address of newly created PendingAck
// /// Return PendingAck's address
// fn acknowledge_mail_pending(outack_hh: &HeaderHash, outmail_hh: &HeaderHash, from: &AgentPubKey) -> ExternResult<EntryHash> {
//     // Get Handle address first
//     let maybe_handle_address = crate::handle::get_handle_entry(from);
//     if let None = maybe_handle_address {
//         return Err(ZomeApiError::Internal("No handle has been set for ack receiving agent".to_string()));
//     }
//     let handle_address = maybe_handle_address.unwrap().0;
//     // Commit PendingAck
//     let pending_ack = PendingAck::new(outmail_hh.clone());
//     //let pending_ack_entry = Entry::App(entry_kind::PendingAck.into(), pending_ack.into());
//     let pending_ack_address = create_entry!(&pending_ack)?;
//     let _ = create_link!(&outack_address, &pending_ack_address, link_tag(link_kind::Pending))?;
//     let _ = create_link!(&handle_address, &pending_ack_address, link_tag(link_kind::AckInbox + &*hdk::AGENT_ADDRESS.to_string()))?;
//     debug!(format!("pending_ack_address: {:?} (for {} ; from: {})", pending_ack_address, handle_address, from)).ok();
//     Ok(pending_ack_address)
// }
