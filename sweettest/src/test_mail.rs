use holochain::test_utils::consistency_10s;

use snapmail::{
   mail::*,
};

use holo_hash::*;

use crate::setup::*;

///
pub async fn test_mail_dm() {
   // Setup
   let (conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   // Send
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![agents[1].clone()],
      cc: vec![],
      bcc: vec![],
      manifest_address_list: vec![],
   };
   let mail_output: SendMailOutput = conductors[0].call(&cells[0].zome("snapmail"), "send_mail", mail).await;
   // Check if received
   let all_arrived: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("snapmail"), "get_all_arrived_mail", ()).await;
   //println!("all_arrived: {:?}", all_arrived);
   assert_eq!(1, all_arrived.len());

   let received_mail: GetMailOutput = conductors[1].call(&cells[1].zome("snapmail"), "get_mail", all_arrived[0].clone()).await;
   //println!("received_mail: {:?}", received_mail);

   assert!(received_mail.0.is_some());
   let rec_mail = received_mail.0.unwrap();
   assert!(rec_mail.is_ok());
   assert_eq!("blablabla", rec_mail.unwrap().mail.payload);

   let maybe_received: HasMailBeenReceivedOutput = conductors[0].call(&cells[0].zome("snapmail"), "has_mail_been_received", mail_output.outmail.clone()).await;
   println!("maybe_received: {:?}", maybe_received);
   assert!(maybe_received.is_err());

   let _ack_eh: EntryHash = conductors[1].call(&cells[1].zome("snapmail"), "acknowledge_mail", all_arrived[0].clone()).await;

   consistency_10s(&cells).await;

   let maybe_received: HasMailBeenReceivedOutput = conductors[0].call(&cells[0].zome("snapmail"), "has_mail_been_received", mail_output.outmail.clone()).await;
   println!("maybe_received2: {:?}", maybe_received);
   assert!(maybe_received.is_ok());

   let has_acked: bool = conductors[1].call(&cells[1].zome("snapmail"), "has_ack_been_received", all_arrived[0].clone()).await;
   println!("has_acked: {:?}", has_acked);
   assert!(has_acked);
}

/// TODO: shutdown doesn't work
pub async fn test_mail_pending() {
   // Setup
   let (mut conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   conductors[1].shutdown().await;

   consistency_10s(&cells).await;

   // Send
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![agents[1].clone()],
      cc: vec![],
      bcc: vec![],
      manifest_address_list: vec![],
   };
   let mail_output: SendMailOutput = conductors[0].call(&cells[0].zome("snapmail"), "send_mail", mail).await;
   println!("mail_output: {:?}", mail_output);
   assert_eq!(1, mail_output.to_pendings.len());

   consistency_10s(&cells).await;
   conductors[1].startup().await;

   consistency_10s(&cells).await;

   let received_mail: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("snapmail"), "check_incoming_mail", ()).await;
   //println!("received_mail: {:?}", received_mail);
   assert_eq!(received_mail.len(), 1);
}