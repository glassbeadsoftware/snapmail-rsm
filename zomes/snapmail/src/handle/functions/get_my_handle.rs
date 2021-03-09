use hdk::prelude::*;

use crate::{
    ZomeString,
    handle::{
        functions::get_handle_element,
        utils::get_handle_string,
    },
};

/// Zome Function
/// Return this agent's latest handle string
#[hdk_extern]
pub fn get_my_handle(_: ()) -> ExternResult<ZomeString> {
    let maybe_current_handle_entry = get_my_handle_element();
    return get_handle_string(maybe_current_handle_entry);
}

/// Return Element holding the agent's handle entry
pub(crate) fn get_my_handle_element() -> Option<Element> {
    /// Get my agent address
    let agent_info = agent_info().ok()?;
    /// Get handle on that agent address
    return get_handle_element(agent_info.agent_latest_pubkey);
}
