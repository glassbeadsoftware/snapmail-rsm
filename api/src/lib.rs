///

pub mod api_error;

#[macro_use]
extern crate lazy_static;

use holochain_zome_types::*;
use holochain::conductor::ConductorHandle;
use holochain::core::workflow::ZomeCallResult;
use holochain_conductor_api::*;

pub use crate::api_error::*;

pub const ZOME_NAME: &str = "snapmail";


lazy_static! {
   pub static ref DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(9);
}


///
pub async fn call_zome(conductor: ConductorHandle, fn_name: &str, payload: ExternIO) -> ZomeCallResult {
   let cell_ids = conductor.list_cell_ids().await.expect("list_cell_ids() should work");
   println!("Cell IDs : {:?}", cell_ids);
   assert!(!cell_ids.is_empty());
   let cell_id = cell_ids[0].clone();
   let provenance = cell_ids[0].agent_pubkey().to_owned();

   let result: ZomeCallResult = conductor.call_zome(ZomeCall {
      cap: None,
      cell_id,
      zome_name: ZOME_NAME.into(),
      fn_name: fn_name.into(),
      provenance,
      payload,
   }).await.unwrap();
   println!("  ZomeCall result = {:?}", result);
   result
}

/// Macro for calling call_zome()
#[macro_export]
macro_rules! snapmail {
    ($handle:tt, $name:expr, $ret:ty, $payload:tt) => ({
      let payload = ExternIO::encode($payload).unwrap();
      let result: SnapmailApiResult<$ret> = tokio_helper::block_on(async {
         let result = call_zome($handle, std::stringify!($name), payload).await?;
         println!(" call_zome result = {:?}", result);
         match result {
            ZomeCallResponse::Ok(io) => {
               println!("         macro io = {:?}", io);
               let maybe_ret: $ret = io.decode().unwrap();
               println!("  macro maybe_ret = {:?}", maybe_ret);
               Ok(maybe_ret)
            },
            ZomeCallResponse::Unauthorized(_, _, _, _) => Err(SnapmailApiError::Unauthorized),
            ZomeCallResponse::NetworkError(err) => Err(SnapmailApiError::NetworkError(err)),
         }
      }, *DEFAULT_TIMEOUT).map_err(|_e| SnapmailApiError::Timeout)?;
      println!("     macro result = {:?}", result);
      result
    })
}


///
pub fn snapmail_api_get_my_handle(conductor: ConductorHandle, _ : ()) -> SnapmailApiResult<String> {
   snapmail!(conductor, get_my_handle, String, ())
}