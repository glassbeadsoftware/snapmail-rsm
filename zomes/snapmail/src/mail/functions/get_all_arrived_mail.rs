use hdk3::prelude::*;
use hdk3::prelude::query::ChainQueryFilter;

use crate::{
    ZomeHhVec,
    link_kind::*, entry_kind::*,
};

/// Zome Function
/// Return list of all InMails that this agent did not acknowledge.
#[hdk_extern]
pub fn get_all_arrived_mail(_: ()) -> ExternResult<ZomeHhVec> {
    /// Get all InMails with query
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(EntryKind::InMail.as_type());
    let maybe_inmail_result = query(inmail_query_args);
    if let Err(err) = maybe_inmail_result {
        debug!("get_all_mails() inmail_result failed: {:?}", err);
        //return Err(hdk3::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let inmails: Vec<Element> = maybe_inmail_result.unwrap().0;
    debug!(" get_all_arrived_mail() inmails: {:?}", inmails);

    // DEBUG - Output dummy values instead
    // let mut unreads = Vec::new();
    // let dummy: HashString = HashString::from("QmYgC6qzGYDZyfp5xcyMH58cnBRde29Ent4JshSk629Qz6");
    // for _ in  0..2000 {
    //     unreads.push(dummy.clone());
    // }

    /// For each InMail
    let mut unreads = Vec::new();
    for inmail in inmails {
        /// Get InMail's EntryHash
        let inmail_hh = inmail.header_hashed().as_hash().to_owned();
        let inmail_header = inmail.header();
        let inmail_eh = inmail_header.entry_hash().expect("Should have an Entry");
        /// Get Acknowledgment private link
        let links_result = get_links(
            inmail_eh.clone(),
            LinkKind::Acknowledgment.as_tag_opt(),
        )?.into_inner();
        /// If link found, it means Ack has not been received
        if links_result.len() > 0 {
            continue;
        }
        /// Add to result list
        unreads.push(inmail_hh.clone());
    }
    /// Done
    Ok(ZomeHhVec(unreads))
}