use hdk::prelude::*;
use hdk::prelude::query::ChainQueryFilter;

use crate::{
    // link_kind,
    entry_kind::*,
    mail::entries::*,
    mail::utils::{get_inmail_state, get_outmail_state},
    utils::*,
};


/// Zome Function
/// Return list of all InMails and OutMails in the local source chain
#[hdk_extern]
#[snapmail_api]
pub fn get_all_mails(_: ()) -> ExternResult<Vec<MailItem>> {
    /// Get all Create InMail headers with query
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .header_type(HeaderType::Create)
       .entry_type(EntryKind::InMail.as_type());
    let maybe_inmails = query(inmail_query_args);
    if let Err(err) = maybe_inmails {
        error!("get_all_mails() query failed: {:?}", err);
        return Err(err);
    }
    let created_inmails: Vec<Element> = maybe_inmails.unwrap();
    debug!(" get_all_mails() create inmails count = {}", created_inmails.len());
    //debug!(" get_all_mails() inmails: {:?}", inmails);
    /// Get all Create OutMail headers with query
    let outmail_query_args = ChainQueryFilter::default()
       .include_entries(true)
       .header_type(HeaderType::Create)
       .entry_type(EntryKind::OutMail.as_type());
    let maybe_outmails = query(outmail_query_args);
    if let Err(err) = maybe_outmails {
        error!("get_all_mails() outmail_result failed: {:?}", err);
        return Err(err);
    }
    let created_outmails: Vec<Element> = maybe_outmails.unwrap();
    debug!(" get_all_mails() outmails count = {}", created_outmails.len());
    /// Change all mails into MailItems
    let mut item_list = Vec::new();
    let my_agent_address = agent_info()?.agent_latest_pubkey;
    /// Change all OutMail into a MailItem
    for outmail_element in created_outmails {
        let outmail_hh = outmail_element.header_hashed().as_hash().to_owned();
        let date: i64 = outmail_element.header().timestamp().as_seconds_and_nanos().0;
        let maybe_state = get_outmail_state(&outmail_hh);
        if let Err(_err) = maybe_state {
            continue;
        }
        debug!(" outmail_element = {:?}", outmail_element);
        let outmail: OutMail = get_typed_from_el(outmail_element)?;
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
    for inmail_element in created_inmails {
        let inmail_hh = inmail_element.header_hashed().as_hash().to_owned();
        let date: i64 = inmail_element.header().timestamp().as_seconds_and_nanos().0;
        let maybe_state = get_inmail_state(&inmail_hh);
        if let Err(_err) = maybe_state {
            continue;
        }
        let state = MailState::In(maybe_state.unwrap());
        let inmail: InMail = get_typed_from_el(inmail_element)?;
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
    Ok(item_list)
}