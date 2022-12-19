use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::handle::utils::*;

/// Zome Function
/// get an agent's latest handle
#[hdk_extern]
//#[snapmail_api]
pub fn get_handle(agent_id: AgentPubKey) -> ExternResult<String> {
    let maybe_current_handle = get_handle_element(agent_id);
    let str = match maybe_current_handle {
        None => "<noname>".to_string(),
        Some((handle, _ah)) => handle.name,
    };
    Ok(str)
}

