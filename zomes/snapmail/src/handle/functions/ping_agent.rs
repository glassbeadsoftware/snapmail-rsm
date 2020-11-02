use hdk3::prelude::*;

// use hdk::{
//     error::ExternResult,
//     holochain_core_types::time::Timeout,
// };

use crate::{
    send,
    ZomeBool,
    protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
#[hdk_extern]
pub fn ping_agent(destination: AgentPubKey) -> ExternResult<ZomeBool> {
    /// 1. Send ping DM
    debug!(format!("ping_agent: {:?}", destination)).ok();
    let response_dm = send(destination, DirectMessageProtocol::Ping)?;
    debug!(format!("ping response = {:?}", response_dm)).ok();
    /// 2. Check Response
    if let DirectMessageProtocol::Success(_) = response_dm {
        return Ok(ZomeBool(true));
    }
    Ok(ZomeBool(false))
}
