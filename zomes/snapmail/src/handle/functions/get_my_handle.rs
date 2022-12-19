use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::handle::functions::get_handle::*;

/// Zome Function
/// Return this agent's latest handle string
#[hdk_extern]
//#[snapmail_api]
pub fn get_my_handle(_: ()) -> ExternResult<String> {
    /// Get my agent address
    let latest_pubkey = agent_info()?.agent_latest_pubkey;
    /// Get handle on that agent address
    get_handle(latest_pubkey)
}
