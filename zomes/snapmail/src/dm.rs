use hdk3::prelude::*;
use crate::{
   mail,
   dm_protocol::*,
   utils::*,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct DmPacket {
   pub from: AgentPubKey,
   pub dm: DirectMessageProtocol,
}

#[hdk_extern]
pub fn receive_dm(dm_packet: DmPacket) -> ExternResult<DirectMessageProtocol> {
   // let (from, dm): (AgentPubKey, DirectMessageProtocol) = dm_packet.into();
   debug!("*** receive_dm() called from {:?}", dm_packet.from);
   let response = mail::receive_dm(dm_packet.from, dm_packet.dm);
   debug!("*** receive_dm() response to send back: {:?}", response);
   Ok(response)
}

///
pub(crate) fn send_dm(destination: AgentPubKey, dm: DirectMessageProtocol) -> ExternResult<DirectMessageProtocol> {
   /// Pre-conditions: Don't call yourself
   let me = agent_info()?.agent_latest_pubkey;
   if destination == me {
      /// FOR DEBUGGING ONLY?
      return error("send_dm() aborted. Can't send to self.");
   }
   // FIXME: Check AgentPubKey is valid, i.e. exists in Directory
   /// Prepare payload
   let dm_packet = DmPacket { from: me, dm: dm.clone() };
   /// Call peer
   debug!("calling remote receive_dm() ; dm = {:?}", dm);
   let response = call_remote(
      destination,
      zome_info()?.zome_name,
      "receive_dm".to_string().into(),
      None,
      &dm_packet,
   );
   debug!("calling remote receive_dm() DONE ; dm = {:?}", dm);
   return response;
   // if let Err(err) = maybe_response {
   //    let fail_str = format!("Failed call_remote() during send_dm(): {:?}", err);
   //    debug!(fail_str).ok();
   //    return error(&fail_str);
   // }

   // Check and convert response to DirectMessageProtocol
   /*
   match maybe_response.unwrap() {
      ZomeCallResponse::Ok(output) => {
         debug!(format!("Received response from receive_dm() : {:?}", output).to_string()).ok();
         //let maybe_msg: Result<DirectMessageProtocol, _> = output.into_inner().try_into()?;
         // if maybe_msg.is_err() {
         //     return Err(HdkError::Wasm(WasmError::Zome("receive() response failed to deserialize.".to_owned())));
         // }
         // Ok(maybe_msg.unwrap())

         let msg: DirectMessageProtocol = output.into_inner().try_into()?;
         debug!(format!("msg_output: {:?} ; dm was: {:?}", msg, dm)).ok();
         Ok(msg)
      },
      ZomeCallResponse::Unauthorized => {
         error("[Unauthorized] call to receive_dm().")
      },
   }
   */
}
