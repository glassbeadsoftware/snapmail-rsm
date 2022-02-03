use hdk::prelude::*;

use super::Mail;
use crate::mail::entries::sign_mail;
use crate::pub_enc_key::*;

/// Entry representing a mail on the DHT waiting to be received by recipient.
/// The recipient is the agentId where the entry is linked from.
/// The mail is encrypted with the recipient's public encryption key.
///
#[hdk_entry(id = "pending_mail")]
#[derive(Clone, PartialEq)]
pub struct PendingMail {
    pub encrypted_mail: XSalsa20Poly1305EncryptedData,
    pub outmail_eh: EntryHash,
    pub from_signature: Signature,
}

impl PendingMail {
   pub fn new(
      encrypted_mail: XSalsa20Poly1305EncryptedData,
      outmail_eh: EntryHash,
      from_signature: Signature,
   ) -> Self {
      Self {
         encrypted_mail,
         outmail_eh,
         from_signature,
      }
   }


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
   pub fn from_mail(mail: Mail, outmail_eh: EntryHash, to: AgentPubKey) -> ExternResult<Self> {
      /// Get my key
      let my_agent_key = agent_info()?.agent_latest_pubkey;
      debug!("get_enc_key() for sender {:?}", my_agent_key);
      let sender_key = get_enc_key(my_agent_key)?;
      /// Get recipient's key
      debug!("get_enc_key() for recipient {:?}", to);
      let recipient_key = get_enc_key(to)?;
      /// Create
      debug!("pending_mail: recipient_key = {:?}", recipient_key);
      Ok(Self::create(mail, outmail_eh, sender_key, recipient_key))
   }


   /// Attempt to decrypt pendingMail with provided keys
   pub fn attempt_decrypt(&self, sender: X25519PubKey, recipient: X25519PubKey) -> Option<Mail> {
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
}