use hdk3::prelude::*;

use crate::{
    send_dm,
    ZomeBool,
    dm_protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
#[hdk_extern]
pub fn ping_agent(destination: AgentPubKey) -> ExternResult<ZomeBool> {
    /// Send ping DM
    debug!("ping_agent: {:?}", destination);
    let response_dm = send_dm(destination, DirectMessageProtocol::Ping)?;
    debug!("ping response = {:?}", response_dm);
    /// Check Response
    if let DirectMessageProtocol::Success(_) = response_dm {
        return Ok(ZomeBool(true));
    }
    Ok(ZomeBool(false))
}
