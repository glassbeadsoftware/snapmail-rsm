use hdk::prelude::*;
use crate::{
   dm::*,
   path_kind,
};

/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** init() callback called!");
   /// Set Global Anchors
   Path::from(path_kind::Directory).ensure()?;
   /// Set access for receive/send
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
   /// Done
   Ok(InitCallbackResult::Pass)
}
