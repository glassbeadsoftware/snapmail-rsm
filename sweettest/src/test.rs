
use holochain::sweettest::*;
use holochain::conductor::{
   ConductorHandle,
};
use maplit::hashset;
use snapmail::{
    handle::*,
 };
use holo_hash::*;


pub async fn test() {
   //observability::test_run().ok();

   let dna = SweetDnaFile::from_bundle(std::path::Path::new("./snapmail.dna"))
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
         .list_active_apps_for_cell_id(cell.cell_id())
         .await
         .unwrap()
   };

   // - Ensure that the first CellId is associated with both apps,
   //   and the other two are only associated with one app each.
   assert_eq!(
      list_apps(conductor.clone(), cell1.clone()).await,
      hashset!["app1".to_string(), "app2".to_string()]
   );

   // let handle_address = snapmail_set_handle(conductor, name.to_string());

   let name = "alex";
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   //println!("handle: {:?}", handle);
   assert_eq!("<noname>", handle);

   let handle_address1: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   //println!("handle: {:?}", handle);
   assert_eq!(name, handle);

   let name = "bobby";
   let handle_address2: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;
   //println!("handle: {:?}", handle);
   assert_eq!(name, handle);

   //let dump = conductor.dump_cell_state(cell1.cell_id()).await;
   //println!("dump: {:?}", dump);
   // let real_cell = conductor.cell_by_id(cell_id).unwrap();
   // let arc = real_cell.env();
   // let source_chain = SourceChainBuf::new(arc.clone().into()).unwrap();
   // let source_chain_dump = source_chain.dump_state().await.unwrap();
   // println!("source_chain_dump: {:?}", source_chain_dump.elements);

   let handle_list: Vec<HandleItem> = conductor.call(&cell1.zome("snapmail"), "get_all_handles", ()).await;
   println!("handle_list: {:?}", handle_list);


   let name = "camille";
   let handle_address3: HeaderHash = conductor.call(&cell1.zome("snapmail"), "set_handle", name.to_string()).await;
   let handle: String = conductor.call(&cell1.zome("snapmail"), "get_my_handle", ()).await;


   println!("handle_address v1: {:?}", handle_address1);
   println!("handle_address v2: {:?}", handle_address2);
   println!("handle_address v3: {:?}", handle_address3);

   assert_eq!(name, handle);
}
