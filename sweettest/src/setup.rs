
use holochain::conductor::*;
use holochain::sweettest::*;
use holochain::test_utils::consistency_10s;
use holochain_state::source_chain::*;
use holochain_zome_types::*;
use holo_hash::*;
use holochain_p2p::*;
use colored::*;
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

   /// QuicMdns Config
   // let mut network = SweetNetwork::local_quic();
   // network.network_type = kitsune_p2p::NetworkType::QuicMdns;
   // let mut config = holochain::conductor::config::ConductorConfig::default();
   // config.network = Some(network);
   // let mut conductor = SweetConductor::from_config(config).await;

   /// Standard config
   let mut conductor = SweetConductor::from_standard_config().await;

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


fn print_element(element: &SourceChainJsonElement) -> String {
   let mut str = format!("({}) ", element.header_address);

   // if (element.header.header_type() == HeaderType::CreateLink) {
   //    str += &format!(" '{:?}'", element.header.tag());
   // }

   str += &format!("{:?30}", element.header.header_type());

   match &element.header {
      Header::CreateLink(create_link) => {
         let s = std::str::from_utf8(&create_link.tag.0).unwrap();
         str += &format!(" '{}'", s);
      },
      Header::Create(create_entry) => {
            let mut s = String::new();
            match &create_entry.entry_type {
            EntryType::App(app_entry_type) => {
               s += " AppEntry ";
               s += &format!("{}", app_entry_type.id().0);
            },
            _ => {
               s += &format!("{:?}", create_entry.entry_type);
            }
         };
         str += &s.green().to_string();
      },
      Header::Update(update_entry) => {
         let mut s = String::new();
         match &update_entry.entry_type {
            EntryType::App(app_entry_type) => {
               s += " AppEntry ";
               s += &format!("{}", app_entry_type.id().0).green();
            },
            _ => {
               s += &format!("{:?}", update_entry.entry_type);
            }
         };
         str += &s.yellow().to_string();
      }
      _ => {},
   }

   //       else {
   //    if (element.header.entry_type) {
   //       if (typeof element.header.entry_type === 'object') {
   //          str += ' - AppEntry ; id = ' + element.header.entry_type.App.id
   //       } else {
   //          str += ' - ' + element.header.entry_type
   //       }
   //    }
   // }

   if element.header.is_genesis() {
      str = str.blue().to_string();
   }
   str
}

pub fn print_peers(conductor: &SweetConductor, cell: &SweetCell) {
   let cell_id = cell.cell_id();
   let space = cell_id.dna_hash().to_kitsune();
   let env = conductor.get_p2p_env(space);
   let peer_dump = p2p_agent_store::dump_state(
      env.into(),
      Some(cell_id.clone()),
   ).expect("p2p_store should not fail");
   println!(" *** peer_dump: {:?}",peer_dump.peers);
}


pub async fn print_chain(conductor: &SweetConductor, agent: &AgentPubKey, cell: &SweetCell) {
   let cell_id = cell.cell_id();
   let vault = conductor.get_cell_env_readonly(cell_id).unwrap();

   let space = cell_id.dna_hash().to_kitsune();

   let env = conductor.get_p2p_env(space);
   let peer_dump = p2p_agent_store::dump_state(
      env.into(),
      Some(cell_id.clone()),
   ).expect("p2p_store should not fail");


   // let p2p_env = conductor
   //    .p2p_env
   //    .lock()
   //    .get(&space)
   //    .cloned()
   //    .expect("invalid cell space");
   // let peer_dump = p2p_agent_store::dump_state(p2p_env.into(), Some(cell_id.clone()))?;

   println!(" *** peer_dump: {:?}",peer_dump.peers);

   let json_dump = dump_state(vault.clone().into(), agent.clone()).await.unwrap();
   //let json = serde_json::to_string_pretty(&json_dump).unwrap();

   println!(" ====== SOURCE-CHAIN STATE DUMP START ===== {}", json_dump.elements.len());
   //println!("source_chain_dump({}) of {:?}", json_dump.elements.len(), agent);

   let mut count = 0;
   for element in &json_dump.elements {
      let str = print_element(&element);
      println!(" {:02}. {}", count, str);
      count += 1;
   }

   println!(" ====== SOURCE-CHAIN STATE DUMP END  ===== {}", json_dump.elements.len());
}