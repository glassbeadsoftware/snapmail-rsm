use holochain::test_utils::consistency_10s;

use snapmail::{
   handle::*,
   mail::*,
   pub_enc_key::*,
};

use holo_hash::*;

use crate::setup::*;

///
pub async fn test_encryption() {
   // Setup
   //let (conductors, agents, apps) = setup_3_conductors().await;
   //let cells = apps.cells_flattened();

   let (conductor0, alex, cell0) = setup_1_conductor().await;
   let (conductor1, billy, cell1) = setup_1_conductor().await;
   let (conductor2, camille, cell2) = setup_1_conductor().await;

   let cells = vec![&cell0, &cell1, &cell2];

   let _: HeaderHash = conductor0.call(&cells[0].zome("snapmail"), "set_handle", ALEX_NICK).await;
   let _: HeaderHash = conductor1.call(&cells[1].zome("snapmail"), "set_handle", BILLY_NICK).await;
   let _: HeaderHash = conductor2.call(&cells[2].zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   print_chain(&conductor0, &alex, &cells[0]).await;

   // println!("Waiting for consistency...");
   // consistency_10s(cells.as_slice()).await;
   // println!("consistency done!");

   let mut length = 0;
   for _ in 0..10u32 {
      let handle_list: Vec<HandleItem> = conductor0.call(&cell0.zome("snapmail"), "get_all_handles", ()).await;
      length = handle_list.len();
      println!("handle_list: {:?}", handle_list);
      if length == 3 {
         break;
      }
      print_peers(&conductor0, &cell0);
      tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
   }
   assert_eq!(3, length);

   // Test
   let _output: () = conductor0.call(&cell0.zome("snapmail"), "test_encryption", billy.clone()).await;
}

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
   // let (mut conductors, agents, apps) = setup_3_conductors().await;
   // let cells = apps.cells_flattened();

   let (mut conductor0, alex, cell0) = setup_1_conductor().await;

   let billy;
   {
      let (mut conductor1, billy_temp, cell1) = setup_1_conductor().await;
      let _: HeaderHash = conductor1.call(&cell1.zome("snapmail"), "set_handle", BILLY_NICK).await;
      billy = billy_temp.clone();
      conductor1.shutdown().await;
   }
   let (mut conductor2, camille, cell2) = setup_1_conductor().await;

   //let mut conductors = vec![&mut conductor1, &mut conductor2, &mut conductor3];
   let agents = vec![&alex, &billy, &camille];
   //let cells = vec![&cell1, &cell2, &cell3];

   let _: HeaderHash = conductor0.call(&cell0.zome("snapmail"), "set_handle", ALEX_NICK).await;

   let _: HeaderHash = conductor2.call(&cell2.zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   // consistency_10s(cells.as_slice()).await;
   //println!("consistency done!");

   //conductors[1].shutdown().await;

   //consistency_10s(cells.as_slice()).await;

   //conductors[1].shutdown().await;

   // let enc_key: holochain_zome_types::X25519PubKey = conductors[1].call(&cells[1].zome("snapmail"), "get_my_enc_key", ()).await;

   //consistency_10s(&cells).await;

   //println!("agents: {:?}", agents);

   // Send
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![billy.clone()], // agents,
      cc: vec![],
      bcc: vec![],
      manifest_address_list: vec![],
   };
   let mail_output: SendMailOutput = conductor0.call(&cell0.zome("snapmail"), "send_mail", mail).await;
   println!("mail_output: {:?}", mail_output);
   assert_eq!(1, mail_output.to_pendings.len());

   // consistency_10s(&cells).await;
   // conductors[1].startup().await;
   //
   // consistency_10s(&cells).await;
   //
   // let received_mail: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("snapmail"), "check_incoming_mail", ()).await;
   // //println!("received_mail: {:?}", received_mail);
   // assert_eq!(received_mail.len(), 1);
}