use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::{
    send_dm,
    dm_protocol::DirectMessageProtocol,
};

/// Zome function
/// Return true if agent is online
#[hdk_extern]
//#[snapmail_api]
pub fn ping_agent(destination: AgentPubKey) -> ExternResult<bool> {
    /// Send ping DM
    debug!("ping_agent: {:?}", destination);
    let response_dm = send_dm(destination, DirectMessageProtocol::Ping)?;
    debug!("ping response = {:?}", response_dm);
    /// Check Response
    if let DirectMessageProtocol::Success(_) = response_dm {
        return Ok(true);
    }
    Ok(false)
}
