use hdk3::prelude::*;
use hdk3::prelude::query::ChainQueryFilter;

use crate::{
    // link_kind,
    entry_kind::*,
    mail::entries::*,
    mail::utils::{get_inmail_state, get_outmail_state},
    utils::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeMailItemVec(Vec<MailItem>);

/// Zome Function
/// Return list of all InMails and OutMails in the local source chain
#[hdk_extern]
pub fn get_all_mails(_: ()) -> ExternResult<ZomeMailItemVec> {
    /// 1. Get all mails with query (InMail & OutMail)
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(EntryKind::InMail.as_type());
    let maybe_inmails = query(inmail_query_args);
    if let Err(err) = maybe_inmails {
        debug!("get_all_mails() query failed: {:?}", err);
        //return Err(hdk3::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let inmails: Vec<Element> = maybe_inmails.unwrap().0;
    debug!(" get_all_mails() inmails count = {}", inmails.len());
    //debug!(" get_all_mails() inmails: {:?}", inmails);

    ///
    let outmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(EntryKind::OutMail.as_type());
    let maybe_outmails = query(outmail_query_args);
    if let Err(err) = maybe_outmails {
        debug!("get_all_mails() outmail_result failed: {:?}", err);
        //return Err(hdk3::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let outmails: Vec<Element> = maybe_outmails.unwrap().0;
    debug!(" get_all_mails() outmails count = {}", outmails.len());
    //debug!(" get_all_mails outmails: {:?}", outmails);
    //let all_mails = inmails.concat(outmails);

    // ///
    // let mail_list = match query_result {
    //     QueryResult::HeadersWithEntries(list) => list,
    //     _ => panic!("Should be HeadersWithEntries"),
    // };

    /// 2. Change all mails into MailItems
    let mut item_list = Vec::new();
    let my_agent_address = agent_info()?.agent_latest_pubkey;

    /// Change all OutMail into a MailItem
    for outmail_element in outmails {
        let outmail_hh = outmail_element.header_hashed().as_hash().to_owned();
        /// Make sure element has not been deleted
        let maybe_element = get(outmail_hh.clone(), GetOptions::latest())?;
        if maybe_element.is_none() {
            continue;
        }
        ///
        let outmail_header = outmail_element.header();
        let outmail_eh = outmail_header.entry_hash().expect("Should have an Entry");
        let date: i64 = outmail_header.timestamp().0;
        let maybe_state = get_outmail_state(outmail_eh);
        if let Err(_err) = maybe_state {
            // deleted entry?
            continue;
        }
        let outmail: OutMail = try_from_element(outmail_element)?;
        let state = MailState::Out(maybe_state.unwrap());
        let item = MailItem {
            address: outmail_hh.clone(),
            author: my_agent_address.clone(),
            mail: outmail.mail,
            state,
            bcc: outmail.bcc.clone(),
            date,
        };
        item_list.push(item.clone());
    }

    debug!(" get_all_mails() final outmail count = {}", item_list.len());

    /// Change all InMail into a MailItem
    for inmail_element in inmails {
        let inmail_hh = inmail_element.header_hashed().as_hash().to_owned();
        /// Make sure element has not been deleted
        let maybe_element = get(inmail_hh.clone(), GetOptions::latest())?;
        if maybe_element.is_none() {
            continue;
        }
        ///
        let inmail_header = inmail_element.header();
        let inmail_eh = inmail_header.entry_hash().expect("Should have an Entry");
        let date: i64 = inmail_header.timestamp().0;
        let maybe_state = get_inmail_state(&inmail_eh);
        if let Err(_err) = maybe_state {
            // deleted entry?
            continue;
        }
        let state = MailState::In(maybe_state.unwrap());
        let inmail: InMail = try_from_element(inmail_element)?;
        let item = MailItem {
            address: inmail_hh.clone(),
            author: inmail.from,
            mail: inmail.mail,
            state,
            bcc: Vec::new(),
            date,
        };
        item_list.push(item.clone());
    }

    /// Done
    debug!(" get_all_mails() total count = {}", item_list.len());
    Ok(ZomeMailItemVec(item_list))
}