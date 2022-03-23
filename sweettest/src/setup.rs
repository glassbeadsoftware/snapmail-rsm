use holochain::sweettest::*;
use holo_hash::*;

use snapmail::handle::*;

use sweettest_utils::*;

use crate::DNA_FILEPATH;


///
pub async fn setup_3_conductors() -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
   let (conductors, agents, apps) = setup_conductors(DNA_FILEPATH, 3).await;
   let cells = apps.cells_flattened();

   let _: HeaderHash = conductors[0].call(&cells[0].zome("snapmail"), "set_handle", ALEX_NICK).await;
   let _: HeaderHash = conductors[1].call(&cells[1].zome("snapmail"), "set_handle", BILLY_NICK).await;
   let _: HeaderHash = conductors[2].call(&cells[2].zome("snapmail"), "set_handle", CAMILLE_NICK).await;

   let _ = try_zome_call(&conductors[0], cells[0], "snapmail", "get_all_handles", (),
                                                    |handle_list: &Vec<HandleItem>| handle_list.len() == 3).await;

   println!("\n\n\n AGENTS SETUP DONE\n\n");

   (conductors, agents, apps)
}
