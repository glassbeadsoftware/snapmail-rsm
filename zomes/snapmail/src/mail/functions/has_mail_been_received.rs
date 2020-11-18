use hdk3::prelude::*;
use hdk3::prelude::link::Link;
use holo_hash::hash_type;
use holo_hash::HoloHash;

use crate::{
    link_kind,
    mail::entries::OutMail,
    utils::*,
    link_tag,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct HasMailBeenReceivedOutput(Result<(), Vec<AgentPubKey>>);


/// Zome function
/// Check if agent received receipts from all receipients of one of its OutMail.
/// If false, returns list of agents who's receipt is missing.
#[hdk_extern]
pub fn has_mail_been_received(outmail_address: HeaderHash) -> ExternResult<HasMailBeenReceivedOutput> {
    /// 1. get OutMail
    let (entry_address, outmail) = get_typed_entry::<OutMail>(outmail_address.clone())?;
    /// 2. Merge all recepients lists into one
    let all_recepients: Vec<AgentPubKey> = [outmail.mail.to, outmail.mail.cc, outmail.bcc].concat();
    debug!(format!("all_recepients: {:?} ({})", all_recepients, outmail_address)).ok();
    /// 3. get all ``receipt`` links
    let links_result: Vec<Link> = get_links!(entry_address, link_tag(link_kind::Receipt))?.into_inner();
    debug!(format!("links_result: {:?}", links_result)).ok();
    /// 4. Make list of Receipt authors
    let receipt_authors: Vec<AgentPubKey> = links_result.iter().map(|link| {
        let raw_data = link.tag.as_ref().clone();
        HoloHash::from_raw_36_and_type(raw_data, hash_type::Agent)
    })
    .collect();
    debug!(format!("receipt_authors: {:?}", receipt_authors)).ok();
    /// 5. Diff lists
    let diff: Vec<AgentPubKey>  = all_recepients.into_iter()
        .filter(|recepient| !receipt_authors.contains(recepient))
        .collect();
    debug!(format!("diff: {:?}", diff)).ok();
    /// Done
    let result = if diff.len() > 0 { Err(diff) } else { Ok(()) };
    Ok(HasMailBeenReceivedOutput(result))
}
