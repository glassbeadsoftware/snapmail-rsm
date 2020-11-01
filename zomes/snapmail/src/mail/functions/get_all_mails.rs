use hdk3::prelude::*;

use chrono::DateTime;
use std::convert::TryFrom;

/*
use hdk::{
    error::ZomeApiResult,
    holochain_core_types::entry::{Entry},
};
use holochain_wasm_utils::{
    //holochain_core_types::link::LinkMatch,
    api_serialization::query::QueryArgsNames::QueryList,
};
*/

use crate::{
    // link_kind,
    entry_kind,
    mail::entries::{*, self},
    mail::utils::{get_inmail_state, get_outmail_state},
};

/// Zome Function
/// Return list of all InMails and OutMails in the local source chain
#[hdk_extern]
pub fn get_all_mails() -> ExternResult<Vec<MailItem>> {
    // 1. Get all mails with query
    let query_names = QueryList([entry_kind::InMail.to_owned(), entry_kind::OutMail.to_owned()].to_vec());
    let query_args = QueryArgsOptions {
        start: 0,
        limit: 0,
        headers: true,
        entries: true,
    };
    let maybe_query_result = hdk::query_result(query_names, query_args);
    if let Err(err) = maybe_query_result {
        debug!(format!(" get_all_mails query_result failed: {:?}", err)).ok();
        return Err(err);
    }
    let query_result = maybe_query_result.unwrap();
    debug!(format!(" get_all_mails query_result: {:?}", query_result)).ok();
    let mail_list = match query_result {
        QueryResult::HeadersWithEntries(list) => list,
        _ => panic!("Should be HeadersWithEntries"),
    };

    // For each mail
    let mut item_list = Vec::new();
    for (header, entry) in &mail_list {
        debug!(format!(" mail_list header =  {:?}", header)).ok();
        let date: i64 = DateTime::from(header.timestamp().clone()).timestamp_millis();
        let item = match entry {
            Entry::App(_, entry_value) => {
                if let Ok(inmail) = entries::InMail::try_from(entry_value.clone()) {
                    let maybe_state = get_inmail_state(header.entry_address());
                    if let Err(_err) = maybe_state {
                        // deleted entry?
                        continue;
                    }
                    let state = MailState::In(maybe_state.unwrap());
                    let item = MailItem {
                        address: header.entry_address().clone(),
                        author: inmail.from,
                        mail: inmail.mail,
                        state,
                        bcc: Vec::new(),
                        date,
                    };
                    item
                } else {
                    let outmail = entries::OutMail::try_from(entry_value).expect("Could not convert entry to requested type");
                    let maybe_state = get_outmail_state(header.entry_address());
                    if let Err(_err) = maybe_state {
                        // deleted entry?
                        continue;
                    }
                    let state = MailState::Out(maybe_state.unwrap());
                    let item = MailItem {
                        address: header.entry_address().clone(),
                        author: (*hdk::AGENT_ADDRESS).clone(),
                        mail: outmail.mail,
                        state,
                        bcc: outmail.bcc.clone(),
                        date,
                    };
                    item
                }
            },
            _ => {
                debug!("Should be a mail Entry").ok();
                panic!("Should be a mail Entry")
            },
        };
        // Add item to list
        item_list.push(item.clone());
    }
    debug!(format!(" get_all_mails size = {}", item_list.len())).ok();
    Ok(item_list)
}