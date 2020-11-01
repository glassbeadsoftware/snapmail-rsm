// use hdk::prelude::*;

use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
    },
    holochain_json_api::json::JsonString,
};
use holochain_wasm_utils::{
    holochain_core_types::link::LinkMatch,
};
use crate::{
    signal_protocol::*,
    file::dm::{request_chunk_by_dm, request_manifest_by_dm},
    link_kind, entry_kind, mail::{self, entries::InMail}, file::{FileManifest}};

/// Zome Function
/// Return list of new InMail addresses created after checking MailInbox links
#[hdk_extern]
pub fn check_incoming_mail() -> ExternResult<Vec<Address>> {
    let maybe_my_handle_address = crate::handle::get_my_handle_entry();
    if let None = maybe_my_handle_address {
        return Err(ZomeApiError::Internal("This agent does not have a Handle set up".to_string()));
    }
    let my_handle_address = maybe_my_handle_address.unwrap().0;
    // Lookup `mail_inbox` links on my agentId
    let links_result = hdk::get_links(
        // &*hdk::AGENT_ADDRESS,
        &my_handle_address,
        LinkMatch::Exactly(link_kind::MailInbox),
        LinkMatch::Any,
    )?;
    debug!(format!("incoming_mail links_result: {:?} (for {})", links_result, &my_handle_address)).ok();
    // For each MailInbox link
    let mut new_inmails = Vec::new();
    for pending_address in &links_result.addresses() {
        //  1. Get entry on the DHT
        debug!(format!("pending mail address: {}", pending_address)).ok();
        let maybe_pending_mail = mail::get_pending_mail(pending_address);
        if let Err(err) = maybe_pending_mail {
            debug!(format!("Getting PendingMail from DHT failed: {}", err)).ok();
            continue;
        }
        let (author, pending) = maybe_pending_mail.unwrap();
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        let inmail_entry = Entry::App(entry_kind::InMail.into(), inmail.clone().into());
        let maybe_inmail_address = hdk::commit_entry(&inmail_entry);
        if maybe_inmail_address.is_err() {
            debug!("Failed committing InMail").ok();
            continue;
        }
        new_inmails.push(maybe_inmail_address.unwrap());
        //  3. Remove link from this agentId
        let res = hdk::remove_link(
            //*hdk::AGENT_ADDRESS,
            &my_handle_address,
            &pending_address,
            link_kind::MailInbox,
            "",
        );
        if let Err(err) = res {
            debug!("Remove ``mail_inbox`` link failed:").ok();
            debug!(err).ok();
            continue;
        }
        //  4. Delete PendingMail entry
        let res = hdk::remove_entry(pending_address);
        if let Err(err) = res {
            debug!(format!("Delete PendingMail failed: {:?}", err)).ok();
            //continue; // TODO: figure out why delete entry fails
        }
        debug!(format!("incoming_mail attachments: {}", inmail.clone().mail.attachments.len())).ok();
        //  5. Retrieve and write FileManifest for each attachment
        let mut manifest_list: Vec<FileManifest> = Vec::new();
        for attachment_info in inmail.clone().mail.attachments {
            let manifest_address = attachment_info.manifest_address;
            // Retrieve
            debug!(format!("Retrieving manifest: {}", manifest_address)).ok();
            let maybe_maybe_manifest = request_manifest_by_dm(inmail.clone().from, manifest_address);
            if let Err(_err) = maybe_maybe_manifest {
                break;
            }
            let maybe_manifest = maybe_maybe_manifest.unwrap();
            if let None = maybe_manifest {
                break;
            }
            let manifest = maybe_manifest.unwrap();
            // Write
            let file_entry = Entry::App(entry_kind::FileManifest.into(), manifest.clone().into());
            let maybe_file_address = hdk::commit_entry(&file_entry);
            if let Err(err) = maybe_file_address {
                let response_str = "Failed committing FileManifest";
                debug!(format!("{}: {}", response_str, err)).ok();
                break;
            }
            // Add to list
            manifest_list.push(manifest);
        }
        //  6. Retrieve and write each FileChunk for each attachment
        for manifest in manifest_list {
            for chunk_address in manifest.clone().chunks {
                // Retrieve
                let maybe_maybe_chunk = request_chunk_by_dm(inmail.clone().from, chunk_address);
                if let Err(_err) = maybe_maybe_chunk {
                    break;
                }
                let maybe_chunk = maybe_maybe_chunk.unwrap();
                if let None = maybe_chunk {
                    break;
                }
                let chunk = maybe_chunk.unwrap();
                // Write
                let file_entry = Entry::App(entry_kind::FileChunk.into(), chunk.into());
                let maybe_address = hdk::commit_entry(&file_entry);
                if let Err(err) = maybe_address {
                    let response_str = "Failed committing FileChunk";
                    debug!(format!("{}: {}", response_str, err)).ok();
                    break;
                }
            }
            // Emit Signal
            let signal = SignalProtocol::ReceivedFile(manifest);
            let signal_json = serde_json::to_string(&signal).expect("Should stringify");
            let res = hdk::emit_signal("received_file", JsonString::from_json(&signal_json));
            if let Err(err) = res {
                debug!(format!("Emit signal failed: {}", err)).ok();
            }
        }
    }
    Ok(new_inmails)
}
