
use holochain::sweettest::*;
use holochain::test_utils::consistency_10s;
use holo_hash::*;
use futures::future;
use snapmail::{
   handle::*,
};

pub const DNA_FILEPATH: &str = "./snapmail.dna";
pub const ALEX_NICK: &str = "alex";
pub const BILLY_NICK: &str = "billy";
pub const CAMILLE_NICK: &str = "camille";

///
pub async fn setup_1_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   let mut network = SweetNetwork::local_quic();
   network.network_type = kitsune_p2p::NetworkType::QuicMdns;
   let mut config = holochain::conductor::config::ConductorConfig::default();
   config.network = Some(network);
   let mut conductor = SweetConductor::from_config(config).await;

   // let mut conductor = SweetConductor::from_standard_config().await;

   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app", alex.clone(), &[dna.clone()])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   (conductor, alex, cell1)
}

///
pub async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   // let mut network = SweetNetwork::env_var_proxy().unwrap_or_else(|| {
   //    println!("KIT_PROXY not set using local quic network");
   //    SweetNetwork::local_quic()
   // });
   // let mut network = SweetNetwork::local_quic();
   // network.network_type = kitsune_p2p::NetworkType::QuicMdns;
   // let mut config = holochain::conductor::config::ConductorConfig::default();
   // config.network = Some(network);
   // let mut conductors = SweetConductorBatch::from_config(n, config).await;

   let mut conductors = SweetConductorBatch::from_standard_config(n).await;

   let all_agents: Vec<AgentPubKey> =
      future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
   let apps = conductors
      .setup_app_for_zipped_agents("app", &all_agents, &[dna])
      .await
      .unwrap();
   conductors.exchange_peer_info().await;
   (conductors, all_agents, apps)
}

///
pub async fn setup_3_conductors() -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let (conductors, agents, apps) = setup_conductors(3).await;
   let cells = apps.cells_flattened();

   let _: HeaderHash = conductors[0].call(&cells[0].zome("snapmail"), "set_handle", ALEX_NICK).await;
   let _: HeaderHash = conductors[1].call(&cells[1].zome("snapmail"), "set_handle", BILLY_NICK).await;
   let _: HeaderHash = conductors[2].call(&cells[2].zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   consistency_10s(&cells).await;

   let handle_list: Vec<HandleItem> = conductors[0].call(&cells[0].zome("snapmail"), "get_all_handles", ()).await;
   assert_eq!(3, handle_list.len());


   (conductors, agents, apps)
}
