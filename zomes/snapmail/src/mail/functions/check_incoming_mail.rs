use hdk3::prelude::*;

use crate::{
    mail::entries::PendingMail,
    ZomeHeaderHashVec,
    utils::*,
    signal_protocol::*,
    //file::dm::{request_chunk_by_dm, request_manifest_by_dm},
    link_kind::*, entry_kind,
    mail::{self, entries::InMail},
    //file::{FileManifest},
};

/// Zome Function
/// Return list of new InMail addresses created after checking MailInbox links
#[hdk_extern]
pub fn check_incoming_mail(_:()) -> ExternResult<ZomeHeaderHashVec> {
    let maybe_element = crate::handle::get_my_handle_element();
    if let None = maybe_element {
        return error("This agent does not have a Handle set up");
    }
    let my_handle_element = maybe_element.unwrap();
    let my_handle_address = get_eh(&my_handle_element)?;
    /// Lookup `mail_inbox` links on my agentId
    let links_result = get_links(
        my_handle_address.clone(),
        Some(LinkKind::MailInbox.as_tag()),
        )?.into_inner();
    debug!("incoming_mail links_result: {:?} (for {})", links_result, &my_handle_address).ok();
    // For each MailInbox link
    let mut new_inmails = Vec::new();
    for link in &links_result {
        let pending_mail_eh = link.target.clone();
        let maybe_hh = get_latest_for_entry::<PendingMail>(pending_mail_eh.clone())?;
        if maybe_hh.is_none() {
            debug!("Header not found for pending mail entry").ok();
            continue;
        }
        let pending_hh = maybe_hh.unwrap().1;
        //  1. Get entry on the DHT
        debug!("pending mail address: {}", pending_mail_eh).ok();
        let maybe_pending_mail = mail::get_pending_mail(&pending_mail_eh);
        if let Err(err) = maybe_pending_mail {
            debug!("Getting PendingMail from DHT failed: {}", err).ok();
            continue;
        }
        let (author, pending) = maybe_pending_mail.unwrap();
        //  2. Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        //let inmail_entry = Entry::App(entry_kind::InMail.into(), inmail.clone().into());
        let maybe_inmail_address = create_entry(&inmail);
        if maybe_inmail_address.is_err() {
            debug!("Failed committing InMail").ok();
            continue;
        }
        new_inmails.push(maybe_inmail_address.unwrap());
        //  3. Remove link from this agentId
        let res = delete_link(link.create_link_hash.clone());
        if let Err(err) = res {
            debug!("Remove ``mail_inbox`` link failed:").ok();
            debug!(err).ok();
            continue;
        }
        //  4. Delete PendingMail entry
        let res = delete_entry(pending_hh);
        if let Err(err) = res {
            debug!("Delete PendingMail failed: {:?}", err).ok();
            //continue; // TODO: figure out why delete entry fails
        }
        // debug!("incoming_mail attachments: {}", inmail.clone().mail.attachments.len()).ok();
        // //  5. Retrieve and write FileManifest for each attachment
        // let mut manifest_list: Vec<FileManifest> = Vec::new();
        // for attachment_info in inmail.clone().mail.attachments {
        //     let manifest_address = attachment_info.manifest_address;
        //     // Retrieve
        //     debug!("Retrieving manifest: {}", manifest_address).ok();
        //     let maybe_maybe_manifest = request_manifest_by_dm(inmail.clone().from, manifest_address);
        //     if let Err(_err) = maybe_maybe_manifest {
        //         break;
        //     }
        //     let maybe_manifest = maybe_maybe_manifest.unwrap();
        //     if let None = maybe_manifest {
        //         break;
        //     }
        //     let manifest = maybe_manifest.unwrap();
        //     // Write
        //     let maybe_file_address = create_entry(&manifest);
        //     if let Err(err) = maybe_file_address {
        //         let response_str = "Failed committing FileManifest";
        //         debug!("{}: {}", response_str, err).ok();
        //         break;
        //     }
        //     // Add to list
        //     manifest_list.push(manifest);
        // }
        // //  6. Retrieve and write each FileChunk for each attachment
        // for manifest in manifest_list {
        //     for chunk_address in manifest.clone().chunks {
        //         // Retrieve
        //         let maybe_maybe_chunk = request_chunk_by_dm(inmail.clone().from, chunk_address);
        //         if let Err(_err) = maybe_maybe_chunk {
        //             break;
        //         }
        //         let maybe_chunk = maybe_maybe_chunk.unwrap();
        //         if let None = maybe_chunk {
        //             break;
        //         }
        //         let chunk = maybe_chunk.unwrap();
        //         // Write
        //         let maybe_address = create_entry(&chunk);
        //         if let Err(err) = maybe_address {
        //             let response_str = "Failed committing FileChunk";
        //             debug!("{}: {}", response_str, err).ok();
        //             break;
        //         }
        //     }
        //     // FIXME
        //     // // Emit Signal
        //     // let signal = SignalProtocol::ReceivedFile(manifest);
        //     // let signal_json = serde_json::to_string(&signal).expect("Should stringify");
        //     // let res = hdk::emit_signal("received_file", JsonString::from_json(&signal_json));
        //     // if let Err(err) = res {
        //     //     debug!("Emit signal failed: {}", err).ok();
        //     // }
        // }
    }
    Ok(ZomeHeaderHashVec(new_inmails))
}
