use hdk::prelude::*;

use crate::{
    mail::entries::PendingMail,
    utils::*,
    signal_protocol::*,
    file::dm::{request_chunk_by_dm, request_manifest_by_dm},
    link_kind::*,
    mail::entries::InMail,
    file::{FileManifest},
};


/// Zome Function
/// Check for PendingMails and convert to InMails
/// Return list of new InMail addresses created after checking for PendingMails
#[hdk_extern]
#[snapmail_api]
pub fn check_mail_inbox(_:()) -> ExternResult<Vec<HeaderHash>> {
    /// Lookup `mail_inbox` links on my agentId
    let me = agent_info()?.agent_latest_pubkey;
    let my_agent_eh = EntryHash::from(me.clone());
    let links_result = get_links(
        my_agent_eh.clone(),
        //None,
        LinkKind::MailInbox.as_tag_opt(),
        )?;
    debug!("incoming_mail links_result: {:?} (for {})", links_result, &my_agent_eh);
    /// Check each MailInbox link
    let mut new_inmails = Vec::new();
    for inbox_link in &links_result {
        let pending_mail_eh = inbox_link.target.clone();
        let maybe_el = get(pending_mail_eh.clone(), GetOptions::latest())?;
        if maybe_el.is_none() {
            warn!("Header not found for pending mail entry");
            continue;
        }
        let pending_hh = maybe_el.unwrap().header_address().clone();
        /// Get entry on the DHT
        let maybe_pending_mail = get_typed_and_author::<PendingMail>(&pending_mail_eh);
        if let Err(err) = maybe_pending_mail {
            warn!("Getting PendingMail from DHT failed: {}", err);
            continue;
        }
        let (author, pending) = maybe_pending_mail.unwrap();
        /// Convert and Commit as InMail
        let inmail = InMail::try_from_pending(pending, author)?.unwrap();
        let maybe_inmail_hh = create_entry(&inmail);
        if maybe_inmail_hh.is_err() {
            error!("Failed committing InMail");
            continue;
        }
        //debug!("inmail_hh: {}", maybe_inmail_hh.clone().unwrap());
        new_inmails.push(maybe_inmail_hh.unwrap());
        /// Remove inbox link
        let res = delete_link(inbox_link.create_link_hash.clone());
        if let Err(err) = res {
            error!("Remove ``mail_inbox`` link failed:");
            error!(?err);
            continue;
        }
        // /// Remove Pendings link
        // let res = delete_pendings_link(&inmail.outmail_eh, &me);
        // if let Err(err) = res {
        //     error!("Remove ``pendings`` link failed:");
        //     error!(?err);
        //     continue;
        // }
        //debug!("delete_link res: {:?}", res);
        /// Delete PendingMail entry
        let res = delete_entry(pending_hh.clone());
        if let Err(err) = res.clone() {
            error!("Delete PendingMail failed: {:?}", err);
            //continue; // TODO: figure out why delete entry fails
        }
        //debug!("delete_entry res: {:?}", res);

        debug!("incoming_mail attachments: {}", inmail.clone().mail.attachments.len());
        /// Retrieve and write FileManifest for each attachment
        let mut manifest_list: Vec<FileManifest> = Vec::new();
        for attachment_info in inmail.clone().mail.attachments {
            /// Retrieve
            debug!("Retrieving manifest: {}", attachment_info.manifest_eh);
            let maybe_maybe_manifest = request_manifest_by_dm(
                inmail.clone().from,
                attachment_info.manifest_eh,
            );
            if let Err(_err) = maybe_maybe_manifest {
                break;
            }
            let maybe_manifest = maybe_maybe_manifest.unwrap();
            if let None = maybe_manifest {
                break;
            }
            let manifest = maybe_manifest.unwrap();
            /// Write
            let maybe_file_address = create_entry(&manifest);
            if let Err(err) = maybe_file_address {
                let response_str = "Failed committing FileManifest";
                error!("{}: {}", response_str, err);
                break;
            }
            /// Add to list
            manifest_list.push(manifest);
        }
        /// Retrieve and write each FileChunk for each attachment
        for manifest in manifest_list {
            for chunk_eh in manifest.clone().chunks {
                /// Retrieve
                let maybe_maybe_chunk = request_chunk_by_dm(inmail.clone().from, chunk_eh);
                if let Err(_err) = maybe_maybe_chunk {
                    break;
                }
                let maybe_chunk = maybe_maybe_chunk.unwrap();
                if let None = maybe_chunk {
                    break;
                }
                let chunk = maybe_chunk.unwrap();
                /// Write
                let maybe_address = create_entry(&chunk);
                if let Err(err) = maybe_address {
                    let response_str = "Failed committing FileChunk";
                    error!("{}: {}", response_str, err);
                    break;
                }
            }
            // Emit Signal
            let signal = SignalProtocol::ReceivedFile(manifest);
            let res = emit_signal(&signal);
            if let Err(err) = res {
                error!("Emit signal failed: {}", err);
            }
        }
    }
    debug!("incoming_mail new_inmails.len() = {} (for {})", new_inmails.len(), &my_agent_eh);
    Ok(new_inmails)
}


// /// Delete Pendings links from outmail to `to` agent
// fn delete_pendings_link(outmail_eh: &EntryHash, to: &AgentPubKey) -> ExternResult<HeaderHash> {
//     let pendings_links_result = get_links(
//         outmail_eh.clone(),
//         //None,
//         Some(LinkKind::Pendings.concat_hash(to)),
//     )?;
//     debug!("pendings_links_result: {:?}", pendings_links_result);
//     if pendings_links_result.len() != 1 {
//         return error("Pendings link not found");
//     }
//     let res = delete_link(pendings_links_result[0].create_link_hash.clone());
//     res
// }