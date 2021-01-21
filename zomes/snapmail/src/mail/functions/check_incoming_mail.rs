use hdk3::prelude::*;

use crate::{
    mail::entries::PendingMail,
    ZomeHhVec,
    utils::*,
    //signal_protocol::*,
    //file::dm::{request_chunk_by_dm, request_manifest_by_dm},
    link_kind::*,
    mail::{self, entries::InMail},
    //file::{FileManifest},
};

/// Zome Function
/// Check for PendingMails and convert to InMails
/// Return list of new InMail addresses created after checking for PendingMails
#[hdk_extern]
pub fn check_incoming_mail(_:()) -> ExternResult<ZomeHhVec> {
    /// Lookup `mail_inbox` links on my agentId
    let my_agent_eh = EntryHash::from(agent_info()?.agent_latest_pubkey);
    let links_result = get_links(
        my_agent_eh.clone(),
        //None,
        LinkKind::MailInbox.as_tag_opt(),
        )?.into_inner();
    debug!("incoming_mail links_result: {:?} (for {})", links_result, &my_agent_eh);
    /// Check each MailInbox link
    let mut new_inmails = Vec::new();
    for link in &links_result {
        let pending_mail_eh = link.target.clone();
        let maybe_hh = get_latest_entry_from_eh::<PendingMail>(pending_mail_eh.clone())?;
        if maybe_hh.is_none() {
            debug!("Header not found for pending mail entry");
            continue;
        }
        let pending_hh = maybe_hh.unwrap().1;
        //debug!("pending_mail_eh: {}", pending_mail_eh);
        /// Get entry on the DHT
        let maybe_pending_mail = mail::get_pending_mail(&pending_mail_eh);
        if let Err(err) = maybe_pending_mail {
            debug!("Getting PendingMail from DHT failed: {}", err);
            continue;
        }
        let (author, pending) = maybe_pending_mail.unwrap();
        /// Convert and Commit as InMail
        let inmail = InMail::from_pending(pending, author);
        let maybe_inmail_hh = create_entry(&inmail);
        if maybe_inmail_hh.is_err() {
            debug!("Failed committing InMail");
            continue;
        }
        //debug!("inmail_hh: {}", maybe_inmail_hh.clone().unwrap());
        new_inmails.push(maybe_inmail_hh.unwrap());
        /// Remove link from this agent address
        let res = delete_link(link.create_link_hash.clone());
        if let Err(err) = res {
            debug!("Remove ``mail_inbox`` link failed:");
            debug!(err);
            continue;
        }
        //debug!("delete_link res: {:?}", res);
        /// Delete PendingMail entry
        let res = delete_entry(pending_hh);
        if let Err(err) = res {
            debug!("Delete PendingMail failed: {:?}", err);
            //continue; // TODO: figure out why delete entry fails
        }
        //debug!("delete_entry res: {:?}", res);

        // debug!("incoming_mail attachments: {}", inmail.clone().mail.attachments.len());
        // //  5. Retrieve and write FileManifest for each attachment
        // let mut manifest_list: Vec<FileManifest> = Vec::new();
        // for attachment_info in inmail.clone().mail.attachments {
        //     let manifest_address = attachment_info.manifest_address;
        //     // Retrieve
        //     debug!("Retrieving manifest: {}", manifest_address);
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
        //         debug!("{}: {}", response_str, err);
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
        //             debug!("{}: {}", response_str, err);
        //             break;
        //         }
        //     }
        //     // FIXME: Emit Signal
        //     // // Emit Signal
        //     // let signal = SignalProtocol::ReceivedFile(manifest);
        //     // let signal_json = serde_json::to_string(&signal).expect("Should stringify");
        //     // let res = hdk::emit_signal("received_file", JsonString::from_json(&signal_json));
        //     // if let Err(err) = res {
        //     //     debug!("Emit signal failed: {}", err);
        //     // }
        // }
    }
    debug!("incoming_mail new_inmails.len() = {} (for {})", new_inmails.len(), &my_agent_eh);
    Ok(ZomeHhVec(new_inmails))
}
