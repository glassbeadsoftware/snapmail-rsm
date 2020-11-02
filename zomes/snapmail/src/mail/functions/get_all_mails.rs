use hdk3::prelude::*;
use hdk3::prelude::query::ChainQueryFilter;

use chrono::DateTime;
use std::convert::TryFrom;


use crate::{
    // link_kind,
    entry_kind,
    def_to_type,
    mail::entries::{*, self},
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
       .entry_type(def_to_type(entry_kind::InMail));
    let maybe_inmail_result = query!(inmail_query_args);
    if let Err(err) = maybe_inmail_result {
        debug!(format!("get_all_mails() inmail_result failed: {:?}", err)).ok();
        return Err(hdk3::error::HdkError::SerializedBytes(err));
        //return Err(err);
    }
    let inmails: Vec<Element> = maybe_inmail_result.unwrap().0;
    debug!(format!(" get_all_mails inmails: {:?}", inmails)).ok();
    ///
    let outmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(def_to_type(entry_kind::OutMail));
    let maybe_outmail_result = query!(outmail_query_args);
    if let Err(err) = maybe_outmail_result {
        debug!(format!("get_all_mails() outmail_result failed: {:?}", err)).ok();
        return Err(hdk3::error::HdkError::SerializedBytes(err));
        //return Err(err);
    }
    let outmails: Vec<Element> = maybe_outmail_result.unwrap().0;
    debug!(format!(" get_all_mails outmails: {:?}", outmails)).ok();
    //let all_mails = inmails.concat(outmails);

    // ///
    // let mail_list = match query_result {
    //     QueryResult::HeadersWithEntries(list) => list,
    //     _ => panic!("Should be HeadersWithEntries"),
    // };

    /// 2. Change all mails into MailItems
    let mut item_list = Vec::new();
    let my_agent_address = agent_info!()?.agent_latest_pubkey;

    /// Change all InMail into a MailItem
    for element in outmails {
        let header_address = element.header_hashed().as_hash().to_owned();
        let header = element.header();
        let entry_address = header.entry_hash().expect("Should have an Entry");
        let date: i64 = header.timestamp().0;
        let maybe_state = get_outmail_state(entry_address);
        if let Err(_err) = maybe_state {
            // deleted entry?
            continue;
        }
        let outmail: OutMail = try_from_element(element)?;
        let state = MailState::Out(maybe_state.unwrap());
        let item = MailItem {
            address: header_address.clone(),
            author: my_agent_address.clone(),
            mail: outmail.mail,
            state,
            bcc: outmail.bcc.clone(),
            date,
        };
        item_list.push(item.clone());
    }

    /// Change all InMail into a MailItem
    for element in inmails {
        let header_address = element.header_hashed().as_hash().to_owned();
        let header = element.header();
        let entry_address = header.entry_hash().expect("Should have an Entry");
        let date: i64 = header.timestamp().0;
        let maybe_state = get_inmail_state(&entry_address);
        if let Err(_err) = maybe_state {
            // deleted entry?
            continue;
        }
        let state = MailState::In(maybe_state.unwrap());
        let inmail: InMail = try_from_element(element)?;
        let item = MailItem {
            address: header_address.clone(),
            author: inmail.from,
            mail: inmail.mail,
            state,
            bcc: Vec::new(),
            date,
        };
        item_list.push(item.clone());
    }

    /// Done
    debug!(format!(" get_all_mails() size = {}", item_list.len())).ok();
    Ok(ZomeMailItemVec(item_list))
}