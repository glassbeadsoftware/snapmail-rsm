use hdk::prelude::*;

use crate::{
   dm::*,
   path_kind,
   pub_enc_key::*,
};

/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** init() callback called!");
   /// Set Global Anchors
   Path::from(path_kind::Directory).ensure()?;
   /// Set access for receive/send function
   let mut functions: GrantedFunctions = BTreeSet::new();
   functions.insert((zome_info()?.zome_name, REMOTE_ENDPOINT.into()));
   create_cap_grant(
      CapGrantEntry {
         tag: "".into(),
         // empty access converts to unrestricted
         access: ().into(),
         functions,
      }
   )?;
   /// Create public encryption key and broadcast it
   PubEncKey::create_and_share()?;
   /// Done
   Ok(InitCallbackResult::Pass)
}
