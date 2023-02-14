use tokio::time::{sleep, Duration};

//use holochain::sweettest::*;
use holo_hash::*;

use sweettest_utils::*;

use snapmail::{
   handle::*,
   mail::*,
   //pub_enc_key::*,
};

use snapmail_model::*;
use crate::setup::*;
use crate::DNA_FILEPATH;


///
pub async fn test_encryption() {
   // Setup
   let (conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   // let (conductor0, alex, cell0) = setup_1_conductor().await;
   // let (conductor1, billy, cell1) = setup_1_conductor().await;
   // let (conductor2, _camille, cell2) = setup_1_conductor().await;
   //
   // let cells = vec![&cell0, &cell1, &cell2];

   let _: ActionHash = conductors[0].call(&cells[0].zome("snapmail"), "set_handle", ALEX_NICK).await;
   let _: ActionHash = conductors[1].call(&cells[1].zome("snapmail"), "set_handle", BILLY_NICK).await;
   let _: ActionHash = conductors[2].call(&cells[2].zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   print_chain(&conductors[0], &cells[0]).await;

   //println!("Waiting for consistency...");
   //holochain::test_utils::consistency_10s(cells.as_slice()).await;
   //println!("consistency done!");

   let mut length = 0;
   for _ in 0..10u32 {
      let handle_list: Vec<HandleItem> = conductors[0].call(&cells[0].zome("snapmail"), "get_all_handles", ()).await;
      length = handle_list.len();
      println!("handle_list: {:?}", handle_list);
      if length == 3 {
         break;
      }
      print_peers(&conductors[0], &cells[0]).await;
      tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
   }
   assert_eq!(3, length);

   // Test
   let _output: () = conductors[0].call(&cells[0].zome("snapmail"), "test_encryption", agents[1].clone()).await;
}


///
pub async fn test_mail_self() {
   /// Setup
   let (conductor0, alex, cell0) = setup_1_conductor(DNA_FILEPATH).await;
   /// Send
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![alex.clone()],
      cc: vec![],
      bcc: vec![],
      reply_of: None,
      manifest_address_list: vec![],
   };
   let outmail_ah: ActionHash = conductor0.call(&cell0.zome("snapmail"), "send_mail", mail).await;

   sleep(Duration::from_millis(500)).await;
   print_chain(&conductor0,&cell0).await;

   /// Should NOT be considered 'acknowledged'
   let outmail_state: OutMailState = conductor0.call(&cell0.zome("snapmail"), "get_outmail_state", outmail_ah.clone()).await;
   println!("outmail_state: {:?}", outmail_state);
   assert!(outmail_state == OutMailState::AllReceived);

   sleep(Duration::from_millis(500)).await;
   print_chain(&conductor0,&cell0).await;

   /// Check if acknowledged
   let mut unacknowledged_inmails: Vec<ActionHash> = Vec::new();
   for _ in 0..10u32 {
      unacknowledged_inmails = conductor0.call(&cell0.zome("snapmail"), "get_all_unacknowledged_inmails", ()).await;
      if unacknowledged_inmails.len() > 0 {
         break;
      }
      sleep(Duration::from_millis(100)).await;
   }
   println!("unacknowledged_inmails: {:?}", unacknowledged_inmails);
   assert_eq!(1, unacknowledged_inmails.len());

   sleep(Duration::from_millis(500)).await;
   print_chain(&conductor0,  &cell0).await;

   /// Get mail
   let received_mail: GetMailOutput = conductor0.call(&cell0.zome("snapmail"), "get_mail", unacknowledged_inmails[0].clone()).await;
   println!("received_mail: {:?}", received_mail);
   assert!(received_mail.0.is_some());
   let rec_mail = received_mail.0.unwrap();
   assert!(rec_mail.is_ok());
   assert_eq!("blablabla", rec_mail.unwrap().mail.payload);
   /// Ack mail
   let ack_eh: EntryHash = conductor0.call(&cell0.zome("snapmail"), "acknowledge_mail", unacknowledged_inmails[0].clone()).await;
   println!("ack_eh: {:?}", ack_eh);

   sleep(Duration::from_millis(500)).await;
   print_chain(&conductor0, &cell0).await;

   /// Check Ack
   let has_acked: bool = conductor0.call(&cell0.zome("snapmail"), "has_ack_been_delivered", unacknowledged_inmails[0].clone()).await;
   println!("has_acked: {:?}", has_acked);
   assert!(has_acked);
   /// Should be considered 'acknowledged'
   let outmail_state: OutMailState = conductor0.call(&cell0.zome("snapmail"), "get_outmail_state", outmail_ah.clone()).await;
   println!("outmail_state: {:?}", outmail_state);
   assert!(outmail_state == OutMailState::AllAcknowledged);

   sleep(Duration::from_millis(500)).await;
}


///
pub async fn test_mail_dm() {
   // Setup
   let (conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   // A sends to B
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![agents[1].clone()],
      cc: vec![],
      bcc: vec![],
      reply_of: None,
      manifest_address_list: vec![],
   };
   let outmail_ah: ActionHash = conductors[0].call(&cells[0].zome("snapmail"), "send_mail", mail).await;

   //sleep(Duration::from_millis(500)).await;
   //print_chain(&conductors[0], &agents[0], &cells[0]).await;

   /// B checks if arrived
   let unacknowledged_inmails: Vec<ActionHash> = try_zome_call(&conductors[1], &cells[1], "snapmail","get_all_unacknowledged_inmails", (),
                 |unacknowledged_inmails: &Vec<ActionHash>| {unacknowledged_inmails.len() == 1})
      .await
      .expect("Should have an unacknowledged inmail");


   let received_mail: GetMailOutput = conductors[1].call(&cells[1].zome("snapmail"), "get_mail", unacknowledged_inmails[0].clone()).await;
   println!("received_mail: {:?}", received_mail);
   assert!(received_mail.0.is_some());
   let rec_mail = received_mail.0.unwrap();
   assert!(rec_mail.is_ok());
   assert_eq!("blablabla", rec_mail.unwrap().mail.payload);

   //print_chain(&conductors[1], &agents[1], &cells[1]).await;

   /// A acks msg
   let outmail_state: OutMailState = conductors[0].call(&cells[0].zome("snapmail"), "get_outmail_state", outmail_ah.clone()).await;
   println!("outmail_state: {:?}", outmail_state);
   assert!(outmail_state == OutMailState::AllReceived);
   let ack_eh: EntryHash = conductors[1].call(&cells[1].zome("snapmail"), "acknowledge_mail", unacknowledged_inmails[0].clone()).await;
   println!("ack_eh: {:?}", ack_eh);

   sleep(Duration::from_millis(500)).await;
   //print_chain(&conductors[1], &agents[1], &cells[1]).await;

   // /// A checks if msg has been acknowledged
   // println!("*** Calling has_mail_been_fully_acknowledged()");
   // try_zome_call(&conductors[0], cells[0], "has_mail_been_fully_acknowledged", outmail_ah.clone(),
   //               |maybe_received: &HasMailBeenFullyAcknowledgedOutput| {maybe_received.is_ok()})
   //    .await
   //    .expect("Should have received ack");

   /// B checks if ack has been received
   let has_acked: bool = conductors[1].call(&cells[1].zome("snapmail"), "has_ack_been_delivered", unacknowledged_inmails[0].clone()).await;
   println!("has_acked: {:?}", has_acked);
   assert!(has_acked);

   // let outmail_state: OutMailState = conductors[0].call(&cells[0].zome("snapmail"), "get_outmail_state", outmail_ah.clone()).await;
   // println!("outmail_state: {:?}", outmail_state);
   // assert!(outmail_state == OutMailState::AllAcknowledged);

   sleep(Duration::from_millis(500)).await;
}


/// WARNING: shutdown doesn't work
pub async fn test_mail_pending() {
   /// Setup
   let (mut conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   // /// Setup
   // let (mut conductor0, alex, cell0) = setup_1_conductor().await;
   // /// Setup Billy
   // let billy;
   // {
   //    let (mut conductor1, billy_temp, cell1) = setup_1_conductor().await;
   //    let _: ActionHash = conductor1.call(&cell1.zome("snapmail"), "set_handle", BILLY_NICK).await;
   //    billy = billy_temp.clone();
   //    conductor1.shutdown().await;
   // }
   // /// Setup Camille
   // let (mut conductor2, camille, cell2) = setup_1_conductor().await;
   // //let mut conductors = vec![&mut conductor1, &mut conductor2, &mut conductor3];
   // let _agents = vec![&alex, &billy, &camille];
   // //let cells = vec![&cell0, &cell1, &cell2];
   //
   // let _: ActionHash = conductor0.call(&cell0.zome("snapmail"), "set_handle", ALEX_NICK).await;
   //
   // let _: ActionHash = conductor2.call(&cell2.zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   // consistency_10s(cells.as_slice()).await;
   //println!("consistency done!");


   /// B goes offline
   conductors[1].shutdown().await;

   // let enc_key: holochain_zome_types::X25519PubKey = conductors[1].call(&cells[1].zome("snapmail"), "get_my_enc_key", ()).await;

   //consistency_10s(&cells).await;

   //println!("agents: {:?}", agents);

   //println!("\n\n\n SETUP DONE\n\n");


   /// A sends to B
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![agents[1].clone()], // agents,
      cc: vec![],
      bcc: vec![],
      reply_of: None,
      manifest_address_list: vec![],
   };
   let outmail_ah: ActionHash = conductors[0].call(
      &cells[0].zome("snapmail"),
      "send_mail",
      mail,
   ).await;
   println!("outmail_ah: {:?}", outmail_ah);

   sleep(Duration::from_millis(20 * 1000)).await;

   /// Check status: Should be 'Pending'
   /// B checks inbox
   try_zome_call(&conductors[0], &cells[0], "snapmail","get_outmail_state", outmail_ah.clone(),
                 |mail_state: &OutMailState| {mail_state == &OutMailState::AllSent })
      .await
      .expect("Should have AllSent state");


   print_chain(&conductors[0],  &cells[0]).await;

   /// B goes online
   conductors[1].startup().await;

   print_chain(&conductors[1], &cells[1]).await;
   sleep(Duration::from_millis(30 * 1000)).await;
   print_chain(&conductors[1],  &cells[1]).await;

   /// B checks inbox
   try_zome_call(&conductors[1], &cells[1], "snapmail","check_mail_inbox", (), |res:&Vec<ActionHash>| {res.len() > 0})
      .await
      .expect("Should have one mail");
   let mail_ahs = try_zome_call(&conductors[1], &cells[1], "snapmail","get_all_unacknowledged_inmails", (), |res:&Vec<ActionHash>| {res.len() > 0})
      .await
      .expect("Should have one mail");

   /// B acknowledges mail
   let outack_eh: EntryHash = conductors[1].call(
      &cells[1].zome("snapmail"),
      "acknowledge_mail",
      mail_ahs[0].clone(),
   ).await;
   println!("outack_eh: {:?}", outack_eh);


   /// A checks ack inbox
   let outmails_ehs = try_zome_call(&conductors[0], &cells[0], "snapmail","check_ack_inbox", (), |res:&Vec<EntryHash>| {res.len() > 0})
      .await
      .expect("Should have one ack");
   println!("outmails_ehs: {:?}", outmails_ehs);
   try_zome_call(&conductors[0], &cells[0], "snapmail","get_outmail_state", outmail_ah.clone(),
                 |mail_state: &OutMailState| {mail_state == &OutMailState::AllAcknowledged })
      .await
      .expect("Should have FullyAcknowledged state");

   print_chain(&conductors[0],  &cells[0]).await;
}
