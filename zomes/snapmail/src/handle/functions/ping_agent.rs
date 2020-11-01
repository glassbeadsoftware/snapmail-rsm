use hdk3::prelude::*;

// use hdk::{
//     error::ZomeApiResult,
//     holochain_core_types::time::Timeout,
// };

use crate::{
    ZomeBool,
    protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
#[hdk_extern]
pub fn ping_agent(destination: AgentPubKey) -> ExternResult<ZomeBool> {
    /// 1. Send ping DM
    debug!(format!("ping_agent: {:?}", destination)).ok();
    let dm_sb: SerializedBytes = DirectMessageProtocol::Ping.try_into().unwrap();
    let response: ZomeCallResponse = call_remote!(
        destination,
        zome_info!()?.zome_name,
        "receive".to_string().into(),
        None,
        dm_sb
    )?;
    debug!(format!("ping response = {:?}", response)).ok();
    /// 2. Check Response
    match response {
        ZomeCallResponse::Ok(guest_output) => {
            debug!(format!("guest_output: {:?}", guest_output)).ok();
            //let hash: HeaderHash = guest_output.into_inner().try_into()?;
            //debug!(format!("hash_output: {:?}", hash)).ok();
            Ok(ZomeBool(true))
        },
        ZomeCallResponse::Unauthorized => Err(HdkError::Wasm(WasmError::Zome(
            "[Unauthorized] receive() call()".to_owned()))),
    }
}
