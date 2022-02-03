use hdk::prelude::*;
use hdk::prelude::link::Link;

use crate::{
    link_kind::*,
    mail::entries::OutMail,
    utils::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HasMailBeenFullyAcknowledgedOutput(Result<(), Vec<AgentPubKey>>);


/// Zome function
/// Check if agent received receipts from all receipients of one of its OutMail.
/// If false, returns list of agents who's receipt is missing.
#[hdk_extern]
#[snapmail_api]
pub fn has_mail_been_fully_acknowledged(outmail_hh: HeaderHash) -> ExternResult<HasMailBeenFullyAcknowledgedOutput> {
    /// Get OutMail
    let (outmail_eh, outmail) = get_typed_from_hh::<OutMail>(outmail_hh.clone())?;
    /// Merge all recipients lists into one
    let all_recipients: Vec<AgentPubKey> = [outmail.mail.to, outmail.mail.cc, outmail.bcc].concat();
    debug!("all_recipients: {:?} ({})", all_recipients, outmail_hh);
    /// Get all ``receipt`` links
    // FIXME: have tag filtering working when calling get_links
    // let links_result: Vec<Link> = get_links(outmail_eh, LinkKind::Receipt.as_tag_opt())?.into_inner();
    let links_result: Vec<Link> = get_links(outmail_eh, None)?;
    debug!("links_result: {:?}", links_result);
    /// Make list of Receipt authors
    let mut receipt_authors: Vec<AgentPubKey> = Vec::new();
    for receipt_link in links_result {
        let maybe_hash = LinkKind::Receipt.unconcat_hash(&receipt_link.tag);
        if let Err(_err) = maybe_hash {
            continue;
        }
        trace!("maybe_hash suffix = {:?}", maybe_hash);
        receipt_authors.push(maybe_hash.unwrap());
    }
    debug!("receipt_authors: {:?}", receipt_authors);
    /// Diff lists
    let diff: Vec<AgentPubKey>  = all_recipients.into_iter()
                                                .filter(|recipient| !receipt_authors.contains(recipient))
                                                .collect();
    debug!("diff: {:?}", diff);
    /// Done
    let result = if diff.len() > 0 { Err(diff) } else { Ok(()) };
    Ok(HasMailBeenFullyAcknowledgedOutput(result))
}
