use hdk::{
    error::ZomeApiResult,
    holochain_core_types::time::Timeout,
};

use crate::{
    AgentAddress,
    protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
#[hdk_extern]
pub fn ping_agent(destination: AgentHash) -> ExternResult<ZomeBool> {
    // 1. Send DM
    debug!(format!("ping_agent: {:?}", destination)).ok();
    let response: ZomeCallResponse = call_remote!(
        input.agent_pubkey,
        zome_info!()?.zome_name,
        "receive".to_string().into(),
        None,
        DirectMessageProtocol::Ping
    )?;
    hdk::debug(format!("ping response = {:?}", response)).ok();
    // 2. Check Response
    match response {
        ZomeCallResponse::Ok(guest_output) => {
            debug!(format!("guest_output: {:?}", guest_output)).ok();
            //let hash: HeaderHash = guest_output.into_inner().try_into()?;
            //debug!(format!("hash_output: {:?}", hash)).ok();
            Ok(true)
        },
        ZomeCallResponse::Unauthorized => Err(HdkError::Wasm(WasmError::Zome(
            "{\"code\": \"000\", \"message\": \"[Unauthorized] receive\"}".to_owned()))),
    }
}
