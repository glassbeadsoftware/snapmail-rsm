
use holochain::sweettest::*;
use holochain::conductor::{
   ConductorHandle,
};
use maplit::hashset;
use snapmail::{
    handle::*,
    mail::*,
    file::*,
   CHUNK_MAX_SIZE,
 };
use holo_hash::*;

use crate::setup::*;
use crate::test_mail::*;

///
pub async fn test() {
   //test_list_apps().await;

   //test_pub_enc_key().await;

   // Handle
   test_handle().await;

   // Mail
   test_encryption().await;
   test_mail_dm().await;
   //test_mail_pending().await;

   // File
   std::env::set_var("WASM_LOG", "WARN");
   //test_file_dm().await;

}

///
pub async fn test_list_apps() {
   //observability::test_run().ok();

   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   // Install two apps on the Conductor:
   // Both share a CellId in common, and also include a distinct CellId each.
   let mut conductor = SweetConductor::from_standard_config().await;
   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app1", alex.clone(), &[dna.clone()])
      .await
      .unwrap();
   let _app2 = conductor
      .setup_app_for_agent("app2", alex.clone(), &[dna])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   let list_apps = |conductor: ConductorHandle, cell: SweetCell| async move {
      conductor
         .list_running_apps_for_required_cell_id(cell.cell_id())
         .await
         .unwrap()
   };

   // - Ensure that the first CellId is associated with both apps,
   //   and the other two are only associated with one app each.
   assert_eq!(
      list_apps(conductor.clone(), cell1.clone()).await,
      hashset!["app1".to_string(), "app2".to_string()]
   );
}

///
pub async fn test_pub_enc_key() {
   let (conductor, alex, cell1) = setup_1_conductor().await;

   println!("Calling get_my_enc_key()");
   let enc_key: holochain_zome_types::X25519PubKey = conductor.call(&cell1.zome("snapmail"), "get_my_enc_key", ()).await;
   println!("enc_key: {:?}", enc_key);
   //assert_eq!("<noname>", handle);

   print_chain(&conductor, &alex, &cell1).await;

   //let _ :() = conductor.call(&cell1.zome("snapmail"), "init_caps", ()).await;

   //let _enc_key: holochain_zome_types::X25519PubKey = conductor.call(&cell1.zome("snapmail"), "get_my_enc_key", ()).await;

   //let _handle_address1: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", "toto").await;


}

///
pub async fn test_handle() {
   let (conductor, alex, cell1) = setup_1_conductor().await;

   let name = "alex";
   println!("Calling get_my_handle()");
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   println!("handle: {:?}", handle);
   assert_eq!("<noname>", handle);

   let handle_address1: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;
   println!("handle_address1: {:?}", handle_address1);
   //tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   println!("handle: {:?}", handle);
   assert_eq!(name, handle);

   let name = "bobby";
   let _handle_address2: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   println!("handle: {:?}", handle);
   assert_eq!(name, handle);

   //print_chain(&conductor, &alex, &cell1).await;

   let handle_list: Vec<HandleItem> = conductor.call(&cell1.zome("snapmail"), "get_all_handles", ()).await;
   assert_eq!(1, handle_list.len());
   assert_eq!(name, handle_list[0].name);

   let name = "camille";
   let _handle_address3: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;

   let mut handle = String::new();
   for _ in 0..3u32 {
      handle = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
      println!("handle: {:?}", handle);
      if name == handle {
         break;
      }
   }
   assert_eq!(name, handle);

   for _ in 0..3u32 {
      let handle_list: Vec<HandleItem> = conductor.call(&cell1.zome("snapmail"), "get_all_handles", ()).await;
      assert_eq!(1, handle_list.len());
      handle = handle_list[0].name.clone();
      println!("handle_list: {:?}", handle_list);
      if name == handle {
         break;
      }
   }
   assert_eq!(name, handle);
}


pub async fn test_file_dm() {
   send_file_dm(CHUNK_MAX_SIZE * 3 - 2000).await;
}


fn split_file(full_data_string: &str) ->  (String, Vec<String>) {
   let hash = "fake_hash".to_string(); // holo_hash_encode(full_data_string.as_bytes());
   //console.log('file hash: ' + hash);
   let num_chunks = (full_data_string.len() as f32 / CHUNK_MAX_SIZE as f32).ceil() as usize;
   let mut chunks = Vec::with_capacity(num_chunks);

   let mut o = 0;
   for _i in 0..num_chunks {
      let sub: String = full_data_string.chars().skip(o).take(CHUNK_MAX_SIZE).collect();
      chunks.push(sub);
      o += CHUNK_MAX_SIZE;
   }
   // Done
   (hash, chunks)
}

///
pub async fn send_file_dm(size: usize) {
   // Setup
   let (conductors, agents, apps) = setup_3_conductors().await;
   let cells = apps.cells_flattened();

   // - Create fake file
   let data_string = "0123465789".repeat(size / 10);
   let (file_hash, file_chunks) = split_file(&data_string);

   // Write chunks
   let mut chunk_list = Vec::new();
   let mut count: usize = 0;
   for chunk in file_chunks {
      let file_chunk = FileChunk {
         data_hash: file_hash.clone(),
         chunk_index: count,
         chunk,
      };
      let chunk_eh: EntryHash = conductors[0].call(&cells[0].zome("snapmail"), "write_chunk", file_chunk).await;
      chunk_list.push(chunk_eh);
      count += 1;
   }
   chunk_list.reverse();

   // Write manifest
   let manifest_params = WriteManifestInput {
      data_hash: file_hash,
      filename: "fake.str".to_string(),
      filetype: "str".to_string(),
      orig_filesize: data_string.len(),
      chunks: chunk_list.clone(),
   };
   let manifest_address: HeaderHash = conductors[0].call(&cells[0].zome("snapmail"), "write_manifest", manifest_params).await;

   // Send
   let mail = SendMailInput {
      subject: "test-outmail".to_string(),
      payload: "blablabla".to_string(),
      to: vec![agents[1].clone()],
      cc: vec![],
      bcc: vec![],
      manifest_address_list: vec![manifest_address],
   };
   let _mail_output: SendMailOutput = conductors[0].call(&cells[0].zome("snapmail"), "send_mail", mail).await;

   // Check if received
   let all_arrived: Vec<HeaderHash> = conductors[1].call(&cells[1].zome("snapmail"), "get_all_arrived_mail", ()).await;
   //println!("all_arrived: {:?}", all_arrived);
   assert_eq!(1, all_arrived.len());

   let received_mail: GetMailOutput = conductors[1].call(&cells[1].zome("snapmail"), "get_mail", all_arrived[0].clone()).await;
   println!("received_mail: {:?}", received_mail);
   assert!(received_mail.0.is_some());
   let rec_mail = received_mail.0.unwrap();
   assert!(rec_mail.is_ok());
   let mail = rec_mail.unwrap().mail;
   assert_eq!(1, mail.attachments.len());

   let attachment = mail.attachments[0].clone();

   // Get chunk list via manifest
   let manifest: FileManifest = conductors[1].call(&cells[1].zome("snapmail"), "get_manifest", attachment.manifest_eh).await;
   println!("manifest: {:?}", manifest);

   // Get chunks
   let mut result_string = String::new();
   //for (var i = chunk_list.length - 1; i >= 0; --i) {
   for i in 0..chunk_list.len() {
      let result: String = conductors[1].call(&cells[1].zome("snapmail"), "get_chunk", chunk_list[i].clone()).await;
      println!("result_len: {:?}", result.len());
      result_string.push_str(&result);
   }
   assert_eq!(data_string.len(), result_string.len());
   assert_eq!(data_string, result_string);
}