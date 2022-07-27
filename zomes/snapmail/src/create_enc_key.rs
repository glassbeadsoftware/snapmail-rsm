use hdk::prelude::*;

/// Create public encryption key and broadcast it
pub fn create_enc_key() -> ExternResult<()> {
   let new_key = PubEncKey::new();
   let key_eh = hash_entry(&new_key)?;
   let key_hh = create_entry(new_key)?;
   let my_agent_address = agent_info()?.agent_latest_pubkey;
   debug !("key_hh = {:?}", key_hh);
   let _ = create_link(
      my_agent_address,
      key_eh.clone(),
      HdkLinkType::Any,
      LinkKind::EncKey.as_tag(),
   )?;
   debug!("**** EncKey linked to agent!");
   Ok(())
}
