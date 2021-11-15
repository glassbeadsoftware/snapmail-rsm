use hdk::prelude::*;
use hdk::prelude::query::ChainQueryFilter;

use crate::{
   link_kind::*,
   entry_kind::*,
   mail::entries::*,
   utils::*,
   mail::get_outmail_state,
   mail::send_mail_to,
   file::get_manifest,
};


/// Zome Function
/// Re-send mail to OutMails for which we have missing acks
/// Return list of OutMails for which we requested acks
#[hdk_extern]
#[snapmail_api]
pub fn request_acks(_: ()) -> ExternResult<Vec<HeaderHash>> {
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
   /// Check for each OutMail
   let mut hhs = Vec::new();
   for outmail_element in created_outmails {
      let outmail_hh = outmail_element.header_hashed().as_hash().to_owned();
      //let date: i64 = outmail_element.header().timestamp().as_seconds_and_nanos().0;
      let maybe_state = get_outmail_state(outmail_hh.clone());
      if let Err(_err) = maybe_state {
         continue;
      }
      debug!(" outmail_element = {:?}", outmail_element);
      let outmail: OutMail = get_typed_from_el(outmail_element)?;
      let outmail_eh = hash_entry(outmail.clone())?;
      let receipient_count = outmail.bcc.len() + outmail.mail.to.len() + outmail.mail.cc.len();
      let pendings = get_links(outmail_eh.clone(), LinkKind::Pending.as_tag_opt())?;
      let receipts = get_links(outmail_eh.clone(), LinkKind::Receipt.as_tag_opt())?;
      if receipient_count == pendings.len() + receipts.len() {
         continue;
      }
      hhs.push(outmail_hh);
      /// Get file manifest
      let mut file_manifest_list = Vec::new();
      for attachment in outmail.mail.attachments.clone() {
         let manifest = get_manifest(attachment.manifest_eh.into())?;
         file_manifest_list.push(manifest.clone());
      }
      /// Look for missing acks
      let recipients = outmail.recipients();
      let mut pending_agents: Vec<AgentPubKey> = pendings.iter().map(|link| {
         LinkKind::Pendings.unconcat_hash(&link.tag).unwrap()
      }).collect();
      let receipt_agents: Vec<AgentPubKey> = receipts.iter().map(|link| {
         LinkKind::Receipt.unconcat_hash(&link.tag).unwrap()
      }).collect();
      pending_agents.extend_from_slice(&receipt_agents);
      /// Send mail to each missing ack/pending
      for recipient in recipients {
         if pending_agents.contains(&recipient) {
            continue;
         }
         let _res = send_mail_to(&outmail_eh, &outmail.mail, &recipient, &file_manifest_list);
      }
   }
   /// Done
   Ok(hhs)
}


