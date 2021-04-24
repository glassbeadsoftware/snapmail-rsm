///

pub mod api_error;

#[macro_use]
extern crate lazy_static;

use crate::api_error::*;
use snapmail::handle::*;
use snapmail::mail::*;

use holochain_types::app::*;
use holochain_zome_types::*;
use holochain::conductor::error::*;
use holochain::conductor::ConductorHandle;
use holo_hash::*;
use holochain::core::workflow::ZomeCallResult;
use holochain_conductor_api::*;


pub const ZOME_NAME: &str = "snapmail";


lazy_static! {
   pub static ref DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(9);
}


///
async fn call_zome(conductor: ConductorHandle, fn_name: &str, payload: ExternIO) -> ZomeCallResult {
   let cell_ids = conductor.list_cell_ids().await.expect("list_cell_ids() should work");
   println!("Cell IDs : {:?}", cell_ids);
   assert!(!cell_ids.is_empty());
   let cell_id = cell_ids[0].clone();
   let provenance = cell_ids[0].agent_pubkey().to_owned();

   let result= conductor.call_zome(ZomeCall {
      cap: None,
      cell_id,
      zome_name: ZOME_NAME.into(),
      fn_name: fn_name.into(),
      provenance,
      payload,
   }).await.unwrap();
   println!("ZomeCall result: {:?}", result);
   result
}

/// Macro for calling call_zome()
macro_rules! snapmail {
    ($handle:tt, $name:expr, $ret:ty, $payload:tt) => ({
      let payload = ExternIO::encode($payload).unwrap();
      let result = tokio_helper::block_on(async {
         let result = call_zome($handle, std::stringify!($name), payload).await?;
         match result {
            ZomeCallResponse::Ok(io) => {
            let hash: $ret = io.decode()?;
               Ok(hash)
            },
            ZomeCallResponse::Unauthorized(_, _, _, _) => Err(SnapmailApiError::Unauthorized),
            ZomeCallResponse::NetworkError(err) => Err(SnapmailApiError::NetworkError(err)),
         }
      }, *DEFAULT_TIMEOUT).map_err(|_e| SnapmailApiError::Timeout)?;
      result
    })
}

///
pub fn snapmail_get_my_handle(conductor: ConductorHandle) -> SnapmailApiResult<String> {
   snapmail!(conductor, get_my_handle, String, ())
}

///
pub fn snapmail_set_handle(conductor: ConductorHandle, handle: String) -> SnapmailApiResult<HeaderHash> {
   snapmail!(conductor, set_handle, HeaderHash, handle)
}

// ///
// pub fn snapmail_get_all_handles(conductor: ConductorHandle, handle: String) -> SnapmailResult<HeaderHash> {
//    snapmail!(conductor, get_all_handles, HeaderHash, handle)
// }
//
// ///
// pub fn snapmail_ping_agent(conductor: ConductorHandle, handle: String) -> SnapmailResult<HeaderHash> {
//    snapmail!(conductor, ping_agent, HeaderHash, handle)
// }