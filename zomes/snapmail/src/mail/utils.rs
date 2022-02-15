use hdk::prelude::*;

use crate::{
    link_kind::*,
    entry_kind::*,
    mail::entries::*,
    utils::*,
};


/// Get State of InMail
pub(crate) fn get_inmail_state(inmail_hh: HeaderHash) -> ExternResult<InMailState> {
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
    let inmail: InMail = get_typed_from_el(el_details.element.clone())?;
    /// Get OutAck
    let outacks = get_outacks(Some(inmail_hh.to_owned()))?;
    if outacks.len() < 1 {
        return Ok(InMailState::Unacknowledged);
    }
    /// Determine OutAck delivery state
    let outack = outacks[0].to_owned();
    let outack_eh = hash_entry(outack)?;
    let confirmation_created = try_confirming_pending_ack_has_been_received(outack_eh.clone(), &inmail.from)?;
    let outack_state = if confirmation_created {
        DeliveryState::Delivered
    } else {
         get_delivery_state(outack_eh, &inmail.from)?
    };
    /// Map to inmail state
    let inmail_state = match outack_state {
        DeliveryState::Unsent => InMailState::AckUnsent,
        DeliveryState::Pending => InMailState::AckPending,
        DeliveryState::Delivered => InMailState::AckDelivered,
    };
    Ok(inmail_state)
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
    //debug!("get_outacks() maybe_outacks: {:?}", maybe_outacks.as_ref().unwrap());
    let mut res = Vec::new();
    for outack_el in maybe_outacks.unwrap() {
        let outack = get_typed_from_el::<OutAck>(outack_el)?;
        res.push(outack)
    }
    //debug!("get_outacks() res.len(): {}", res.len());
    if res.len() == 0 {
        return Ok(Vec::new());
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
    //debug!("get_inacks() maybe_inacks: {}", maybe_inacks.as_ref().unwrap().len());
    let mut res = Vec::new();
    for inack_el in maybe_inacks.unwrap() {
        let inack = get_typed_from_el::<InAck>(inack_el)?;
        res.push(inack)
    }
    //debug!("get_inacks() res.len(): {}", res.len());
    if res.len() == 0 {
        return Ok(Vec::new());
    }
    /// Filter for this OutMail
    if let Some(outmail_hh) = maybe_outmail_filter {
        /// Make sure its an OutMail
        let (outmail_eh, _) = get_typed_from_hh::<OutMail>(outmail_hh)?;
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
    //debug!("get_confirmations() elements.len(): {}", elements.len());
    if elements.len() == 0 {
        return Ok(Vec::new());
    }
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


/// If no confirmation, and there is a pending/s link but no inbox link, create a DeliveryConfirmation
/// Return true if a DeliveryConfirmation has been created
pub(crate) fn try_confirming_pending_mail_has_been_received(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<bool> {
    debug!("try_confirming_pending_mail_has_been_received() - START");
    /// Check confirmations
    let confirmations = get_confirmations(package_eh.clone())?;
    if !confirmations.is_empty() {
        return Ok(false);
    }
    let mut pending_found = false;
    /// If a pending link and and inbox link match, still waiting for confirmation
    let pendings_links = get_links(package_eh.clone(), Some(LinkKind::Pendings.as_tag()))?;
    let inbox_links = get_links(recipient.to_owned().into(), LinkKind::MailInbox.as_tag_opt())?;
    let inbox_targets: Vec<EntryHash> = inbox_links.iter().map(|x|x.target.clone()).collect();
    for pendings_link in pendings_links.iter() {
        let res = LinkKind::Pendings.unconcat_hash(&pendings_link.tag);
        if let Ok(agent) = res {
            // inbox link found ; check if tag is recipient
            if &agent == recipient {
                pending_found = true;
                if inbox_targets.contains(&pendings_link.target) {
                    return Ok(false);
                }
            }
        }
    }
    /// Create confirmation if conditions are met
    if pending_found {
        debug!("try_confirming_pending_mail_has_been_received() - CREATING CONFIRMATION");
        let confirmation = DeliveryConfirmation::new(package_eh.clone(), recipient.clone());
        let _ = create_entry(confirmation)?;
        return Ok(true);
    }
    /// Done
    Ok(false)
}



/// If no confirmation, and there is a pending/s link but no inbox link, create a DeliveryConfirmation
/// Return true if a DeliveryConfirmation has been created
pub(crate) fn try_confirming_pending_ack_has_been_received(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<bool> {
    debug!("try_confirming_pending_ack_has_been_received() - START");
    /// Check confirmations
    let confirmations = get_confirmations(package_eh.clone())?;
    if !confirmations.is_empty() {
        return Ok(false);
    }
    /// If a pending link and and inbox link match, still waiting for confirmation
    let pending_links = get_links(package_eh.clone(), Some(LinkKind::Pending.as_tag()))?;
    for pending_link in pending_links.iter() {
        if pending_link.tag != LinkKind::Pending.as_tag() {
            continue;
        }
        /// Check for inbox link: If no link, it means it has been deleted by recipient
        let links = get_links(recipient.to_owned().into(), LinkKind::AckInbox.as_tag_opt())?;
        for link in links.iter() {
            let res = LinkKind::AckInbox.unconcat_hash(&link.tag);
            if let Ok(agent) = res {
                // inbox link found ; check if tag is recipient
                if &agent == recipient {
                    return Ok(false);
                }
            }
        }
        /// Create confirmation since Pending found but not inbox link
        debug!("try_confirming_pending_ack_has_been_received() - CREATING CONFIRMATION");
        let confirmation = DeliveryConfirmation::new(package_eh.clone(), recipient.clone());
        let _ = create_entry(confirmation)?;
        return Ok(true);
    }
    Ok(false)
}

///
pub fn get_delivery_state(package_eh: EntryHash, recipient: &AgentPubKey) -> ExternResult<DeliveryState> {
    /// Look for a DeliveryConfirmation entry
    let confirmations = get_confirmations(package_eh.clone())?;
    let confirmed_recipients: Vec<AgentPubKey> = confirmations.iter().map(|x| x.recipient.clone()).collect();
    if confirmed_recipients.contains(recipient) {
        return Ok(DeliveryState::Delivered)
    }
    /// Look for a Pending/s link
    let links = get_links(package_eh.clone(), Some(LinkKind::Pending.as_tag()))?;
    for link in links {
        /// OutAck
        if link.tag == LinkKind::Pending.as_tag() {
            return Ok(DeliveryState::Pending)
        }
        /// OutMail
        let maybe_pendings = LinkKind::Pendings.unconcat_hash(&link.tag);
        if let Ok(agent) = maybe_pendings {
            if &agent == recipient {
                return Ok(DeliveryState::Pending)
            }
        }
    }
    /// None found
    Ok(DeliveryState::Unsent)
}