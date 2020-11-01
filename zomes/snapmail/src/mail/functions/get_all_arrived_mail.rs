//use hdk::prelude::*;

use hdk::{
    error::ZomeApiResult,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{link_kind, entry_kind};

// use hdk::{
//     holochain_persistence_api::{
//         hash::HashString,
//     },
// };

/// Zome Function
/// Return list of all InMails that this agent did not acknowledge.
#[hdk_extern]
pub fn get_all_arrived_mail() -> ExternResult<Vec<Address>> {
    // 1. Get all InMails with query
    let result = query!(entry_kind::InMail.into())?;
    debug!(format!("get_all_arrived_mail: {:?}", result)).ok();

    // DEBUG - Output dummy values instead
    // let mut unreads = Vec::new();
    // let dummy: HashString = HashString::from("QmYgC6qzGYDZyfp5xcyMH58cnBRde29Ent4JshSk629Qz6");
    // for _ in  0..2000 {
    //     unreads.push(dummy.clone());
    // }

    // For each InMail
    let mut unreads = Vec::new();
    for inmail_address in &result {
        //   2. Get Acknowledgment private link
        let res = hdk::get_links_count(
            inmail_address,
            LinkMatch::Exactly(link_kind::Acknowledgment),
            LinkMatch::Any,
        )?;
        //      b. if true continue
        if res.count > 0 {
            continue;
        }
        //   3. Add to result list
        unreads.push(inmail_address.clone());
    }
    Ok(unreads)
}