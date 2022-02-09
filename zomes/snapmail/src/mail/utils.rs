use hdk::prelude::*;

use crate::{
    link_kind::*,
    entry_kind::*,
    mail::entries::*,
    utils::*,
};


/// Get State of InMail
pub(crate) fn get_inmail_state(inmail_hh: &HeaderHash) -> ExternResult<InMailState> {
    /// Get inMail Details
    let maybe_details = get_details(inmail_hh.clone(), GetOptions::latest())?;
    if maybe_details.is_none() {
        return error("No InMail at given address");
    }
    let el_details = match maybe_details.unwrap() {
        Details::Element(details) => details,
        Details::Entry(_) => unreachable!("in get_outmail_state()"),
    };
    /// Check if deleted
    if el_details.deletes.len() > 0 {
        return Ok(InMailState::Deleted);
    }

    /// Get OutAck
    let outacks = get_outacks(Some(inmail_hh.to_owned()))?;
    if outacks.len() < 1 {
        return Ok(InMailState::Arrived);
    }
    let outack = outacks[0].to_owned();
    let outack_eh = hash_entry(outack)?;

    /// Get PendingAck
    let links_result = get_links(outack_eh, LinkKind::Pending.as_tag_opt())?;
    /// If link found, it means Ack has not been received
    if links_result.len() > 0 {
        return Ok(InMailState::Acknowledged);
    }
    Ok(InMailState::AckReceived)
}


/// Return address of created InAck
pub(crate) fn create_inack(outmail_eh: EntryHash, from: &AgentPubKey, ack_signature: Signature) -> ExternResult<HeaderHash> {
    debug!("Create inAck for: {} ({})", outmail_eh, from);
    let inack = InAck::new(outmail_eh.clone(), from.clone(), ack_signature);
    let inack_hh = create_entry(&inack)?;
    //let inack_eh = hash_entry(&inack)?;
    /// Done
    Ok(inack_hh)
}

///
pub(crate) fn get_outacks(maybe_inmail_filter: Option<HeaderHash>) -> ExternResult<Vec<OutAck>> {
     /// Get all OutAck entries
    let outacks_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .header_type(HeaderType::Create)
       .entry_type(EntryKind::OutAck.as_type());
    let maybe_outacks = query(outacks_query_args);
    if let Err(err) = maybe_outacks {
        error!("get_outacks() query failed: {:?}", err);
        return Err(err);
    }
    let mut res = Vec::new();
    for outack_el in maybe_outacks.unwrap() {
        let outack = get_typed_from_el::<OutAck>(outack_el)?;
        res.push(outack)
    }
    /// Filter for this InMail
    if let Some(inmail_hh) = maybe_inmail_filter {
        /// Make sure its an InMail
        let (inmail_eh, _inmail) = get_typed_from_hh::<InMail>(inmail_hh)?;
        res.retain(|outack| outack.inmail_eh == inmail_eh)
    }
    /// Done
    Ok(res)
}

// pub(crate) fn has_been_acknowledged(inmail_hh: HeaderHash) -> ExternResult<bool> {
//     let list = get_outacks(inmail_hh)?;
//     Ok(list.len() > 0)
// }


///
pub(crate) fn get_inacks(maybe_outmail_filter: Option<HeaderHash>) -> ExternResult<Vec<InAck>> {
    /// Get all InAck entries
    let outacks_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .header_type(HeaderType::Create)
       .entry_type(EntryKind::InAck.as_type());
    let maybe_inacks = query(outacks_query_args);
    if let Err(err) = maybe_inacks {
        error!("get_inacks() query failed: {:?}", err);
        return Err(err);
    }
    let mut res = Vec::new();
    for inack_el in maybe_inacks.unwrap() {
        let inack = get_typed_from_el::<InAck>(inack_el)?;
        res.push(inack)
    }
    /// Filter for this OutMail
    if let Some(outmail_hh) = maybe_outmail_filter {
        /// Make sure its an InMail
        let (outmail_eh, _) = get_typed_from_hh::<InMail>(outmail_hh)?;
        res.retain(|outack| outack.outmail_eh == outmail_eh)
    }
    /// Done
    Ok(res)
}


///
pub(crate) fn get_confirmations(package_eh: EntryHash) -> ExternResult<Vec<DeliveryConfirmation>> {
    /// Get all InAck entries
    let query_args = ChainQueryFilter::default()
       .include_entries(true)
       .header_type(HeaderType::Create)
       .entry_type(EntryKind::DeliveryConfirmation.as_type());
    let elements = query(query_args)?;
    let mut confirmations = Vec::new();
    /// Filter for this package
    for el in elements {
        let confirmation = get_typed_from_el::<DeliveryConfirmation>(el)?;
        if confirmation.package_eh == package_eh {
            confirmations.push(confirmation)
        }
    }
    /// Done
    Ok(confirmations)
}