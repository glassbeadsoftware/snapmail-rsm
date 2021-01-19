use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
    holochain_json_api::json::JsonString,
};
use crate::{
    AgentAddress,
    mail::entries::InMail,
};
use crate::{
    file::dm::request_manifest_by_dm,
    signal_protocol::*,
};

/// Zome Function
/// Get InMail or OutMail struct in local source chain at address
pub fn get_missing_attachments(from: AgentAddress, inmail_address: Address) -> ZomeApiResult<u32> {
    let inmail = hdk::utils::get_as_type::<InMail>(inmail_address.clone())?;
    let mut missing = 0;
    for attachment_info in inmail.mail.attachments {
        let manifest_address = attachment_info.manifest_address;
        let manifest_str = format!("Manifest {}", manifest_address);
        let maybe_entry = hdk::get_entry(&manifest_address)?;
        let mut manifest = None;
        // Request manifest if missing
        if let None = maybe_entry {
            // Request manifest
            let maybe_maybe_manifest = request_manifest_by_dm(from.clone(), manifest_address.clone());
            // Notify failure
            if let Err(err) = maybe_maybe_manifest {
                let response_str = format!("{} request failed", manifest_str);
                hdk::debug(format!("{}: {}", response_str, err)).ok();
                missing += 1;
                continue;
            }
            let maybe_manifest = maybe_maybe_manifest.unwrap();
            if let None = maybe_manifest {
                hdk::debug(format!("{} unknown from source agent", manifest_str)).ok();
                missing += 1;
                continue;
            }
            manifest = Some(maybe_manifest.unwrap());
        }

        // Request chunks
        let maybe_missings = crate::file::get_missing_chunks(from.clone(), manifest_address);
        if let Err(err) = maybe_missings {
            let response_str = format!("{} requesting chunks failed", manifest_str);
            hdk::debug(format!("{}: {}", response_str, err)).ok();
            missing += 1;
            continue;
        }
        let missing_chunks_count = maybe_missings.unwrap();
        if missing_chunks_count > 0 {
            missing += 1;
            continue;
        }
        // Emit Signal
        let signal = SignalProtocol::ReceivedFile(manifest.unwrap());
        let signal_json = serde_json::to_string(&signal).expect("Should stringify");
        let res = hdk::emit_signal("received_file", JsonString::from_json(&signal_json));
        if let Err(err) = res {
            hdk::debug(format!("Emit signal failed: {}", err)).ok();
        }
    }
    Ok(missing)
}
