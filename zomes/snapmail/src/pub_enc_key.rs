
use hdk::prelude::*;

use crate::{
  link_kind::*,
   utils::*,
};

/// Entry representing the Public Encryption Key of an Agent
#[hdk_entry(id = "PubEncKey", visibility = "public")]
#[derive(Clone, PartialEq)]
pub struct PubEncKey {
   pub value: X25519PubKey,
}

impl PubEncKey {
   pub fn new() -> Self {
      let value = create_x25519_keypair().expect("Create Keypair should work");
      Self {
         value,
      }
   }

   /// Create public encryption key and broadcast it
   pub fn create_and_share() -> ExternResult<()> {
      let new_key = PubEncKey::new();
      let key_eh = hash_entry(&new_key)?;
      let key_hh = create_entry(new_key)?;
      let my_agent_address = agent_info()?.agent_latest_pubkey;
      debug !("key_hh = {:?}", key_hh);
      let _ = create_link(
         EntryHash::from(my_agent_address),
         key_eh.clone(),
         LinkKind::EncKey.as_tag(),
      )?;
      debug!("**** EncKey linked to agent!");
      Ok(())
   }
}

#[hdk_extern]
#[snapmail_api]
pub fn get_enc_key(from: AgentPubKey) -> ExternResult<X25519PubKey> {
   /// Get All Handle links on agent ; should have only one
   let key_links = get_links(from.into(), LinkKind::EncKey.as_tag_opt())
      .expect("No reason for this to fail");
   assert!(key_links.len() <= 1);
   if key_links.len() == 0 {
      error!("No PubEncKey found for this agent");
      return error("No PubEncKey found for this agent");
   }
   /// Get the Entry from the link
   let key_eh = key_links[0].target.clone();
   let key_and_hash = get_latest_typed_from_eh::<PubEncKey>(key_eh.clone())
      .expect("No reason for get_entry to crash")
      .expect("Should have it");
   /// Done
   Ok(key_and_hash.0.value)
}


#[hdk_extern]
#[snapmail_api]
pub fn get_my_enc_key(_: ()) -> ExternResult<X25519PubKey> {
   /// Get my agent address
   let latest_pubkey = agent_info()?.agent_latest_pubkey;
   /// Get encryption key on that agent address
   get_enc_key(latest_pubkey)
}

#[hdk_extern]
fn test_encryption(to: AgentPubKey) -> ExternResult<()> {
   /// Get my key
   let my_agent_key = agent_info()?.agent_latest_pubkey;
   let sender = get_enc_key(my_agent_key)?;
   /// Get recipient's key
   let recipient = get_enc_key(to)?;
   /// Serialize
   let data: XSalsa20Poly1305Data = vec![1,2,3,74,4,85,48,7,87,89].into();
   /// Encrypt
   let encrypted = x_25519_x_salsa20_poly1305_encrypt(sender, recipient, data)
      .expect("Encryption should work");
   debug!("create decrypt of: {:?}\n With:", encrypted.clone());
   debug!("-    sender = {:?}", sender.clone());
   debug!("- recipient = {:?}", recipient.clone());
   /// Normal decrypt
   let maybe_decrypted = x_25519_x_salsa20_poly1305_decrypt(recipient, sender, encrypted.clone());
   debug!("  maybe_decrypted normal = {:?}", maybe_decrypted);
   /// Inverted keys
   let maybe_decrypted = x_25519_x_salsa20_poly1305_decrypt(sender, recipient, encrypted.clone());
   debug!("maybe_decrypted inverted = {:?}", maybe_decrypted);
   /// Done
   Ok(())
}

// -- VALIDATION -- //

///
pub fn validate_PubEncKey_entry(_: PubEncKey, _maybe_validation_package: Option<ValidationPackage>) -> ExternResult<ValidateCallbackResult> {
   trace!("*** validate_PubEncKey_entry() called!");
   Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_PubEncKey_delete(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
   Ok(ValidateCallbackResult::Invalid("Agent must always have a Handle".into()))
}
