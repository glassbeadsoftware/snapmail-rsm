use hdk::prelude::*;

use

// From your crate
pub trait PendingMailExt {
   fn create(mail: Mail, outmail_eh: EntryHash, sender: X25519PubKey, recipient: X25519PubKey) -> Self;
   fn from_mail(mail: Mail, outmail_eh: EntryHash, to: AgentPubKey) -> ExternResult<Self>;
   fn attempt_decrypt(&self, sender: X25519PubKey, recipient: X25519PubKey) -> Option<Mail>;
   fn try_into_inmail(&self, from: AgentPubKey) -> ExternResult<Option<InMail>>;
}


///
impl PendingMailExt for PendingMail {

   /// Create PendingMail from Mail and recipient's public encryption key
   /// This will encrypt the Mail with the recipient's key
   fn create(mail: Mail, outmail_eh: EntryHash, sender: X25519PubKey, recipient: X25519PubKey) -> Self {
      /// Serialize
      let serialized = bincode::serialize(&mail).unwrap();
      let data: XSalsa20Poly1305Data = serialized.into();
      /// Encrypt
      let encrypted = x_25519_x_salsa20_poly1305_encrypt(sender, recipient, data)
         .expect("Encryption should work");
      trace!("Encrypted: {:?}", encrypted.clone());
      let signature = sign_mail(&mail).expect("Should be able to sign with my key");
      // let me = agent_info().expect("Should have agent info").agent_latest_pubkey;
      // let signature = sign(me, mail).expect("Should be able to sign with my key");
      trace!("with:\n -    sender = {:?}\n - recipient = {:?}", sender.clone(), recipient.clone());
      /// Done
      PendingMail::new(encrypted, outmail_eh, signature)
   }


   /// Create PendingMail from Mail and recipient's public encryption key
   /// This will encrypt the Mail with the recipient's key
   /// called from post_commit()
   fn from_mail(mail: Mail, outmail_eh: EntryHash, to: AgentPubKey) -> ExternResult<Self> {
      /// Get my key
      let me = agent_info()?.agent_latest_pubkey;
      debug!("get_enc_key() for sender {:?}", me);
      let maybe_sender_key = call_remote(
         me.clone(),
         zome_info()?.name,
         "get_enc_key".to_string().into(),
         None,
         me.clone(),
      )?;
      debug!("get_enc_key() for sender result: {:?}", maybe_sender_key);
      let sender_key = match maybe_sender_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(sender) failed")
      };

      /// Get recipient's key
      debug!("get_enc_key() for recipient {:?}", to);
      let maybe_recipient_key = call_remote(
         me.clone(),
         zome_info()?.name,
         "get_enc_key".to_string().into(),
         None,
         to.clone(),
      )?;
      debug!("get_enc_key() for recipient result: {:?}", maybe_recipient_key);
      let recipient_key = match maybe_recipient_key {
         ZomeCallResponse::Ok(output) => output.decode()?,
         _ => return error("Self call to get_enc_key(recipient) failed")
      };
      /// Create
      debug!("pending_mail: recipient_key = {:?}", recipient_key);
      Ok(Self::create(mail, outmail_eh, sender_key, recipient_key))
   }

   /// Attempt to decrypt pendingMail with provided keys
   fn attempt_decrypt(&self, sender: X25519PubKey, recipient: X25519PubKey) -> Option<Mail> {
      trace!("attempt_decrypt of: {:?}", self.encrypted_mail.clone());
      trace!("with:\n -    sender = {:?}\n - recipient = {:?}", sender.clone(), recipient.clone());
      /// Decrypt
      let maybe_decrypted = x_25519_x_salsa20_poly1305_decrypt(sender, recipient, self.encrypted_mail.clone())
         .expect("Decryption should work");
      trace!("attempt_decrypt maybe_decrypted = {:?}", maybe_decrypted);
      let decrypted = match maybe_decrypted {
         Some(data) => data,
         None => return None,
      };
      /// Deserialize
      let mail: Mail = bincode::deserialize(decrypted.as_ref())
         .expect("Deserialization should work");
      /// Done
      Some(mail)
   }




   fn try_into_inmail(&self, from: AgentPubKey) -> ExternResult<Option<InMail>> {
      let received_date = zome_utils::now();
      /// Get my key
      let my_agent_key = agent_info()?.agent_latest_pubkey;
      debug!("try_from_pending my_agent_key: {}", my_agent_key);
      let recipient_key = get_enc_key(my_agent_key.clone())?;
      debug!("try_from_pending recipient_key: {:?}", recipient_key);
      /// Get sender's key
      let sender_key = get_enc_key(from.clone())?;
      debug!("try_from_pending sender_key: {:?}", sender_key);
      /// Decrypt
      let maybe_mail = self.attempt_decrypt(sender_key, recipient_key);
      debug!("try_from_pending maybe_mail: {:?}", maybe_mail);
      /// Into InMail
      let inmail = match maybe_mail {
         None => return Ok(None),
         Some(mail) => {
            InMail::new(mail,
                        from.clone(),
                        received_date,
                        self.outmail_eh,
                        self.from_signature.clone())
         },
      };
      /// Check signature
      let maybe_verified = verify_signature(from, self.from_signature, inmail.mail.clone());
      match maybe_verified {
         Err(err) => {
            let response_str = "Verifying PendingMail failed";
            debug!("{}: {}", response_str, err);
            return error(response_str);
         }
         Ok(false) => {
            let response_str = "Failed verifying PendingMail signature";
            debug!("{}", response_str);
            return error(response_str);
         }
         Ok(true) => debug!("Valid PendingMail signature"),
      }
      /// Done
      Ok(Some(inmail))
   }
}
