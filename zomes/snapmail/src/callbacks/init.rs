use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::{
   dm::*,
   path_kind,
   //pub_enc_key::*,
   create_enc_key,
   link_kind::*,
};

#[hdk_extern]
fn init_caps(_: ()) -> ExternResult<()> {
   let mut functions: GrantedFunctions = BTreeSet::new();
   functions.insert((zome_info()?.name, REMOTE_ENDPOINT.into()));
   //functions.insert((zome_info()?.name, "get_enc_key".into()));
   create_cap_grant(
      CapGrantEntry {
         tag: "".into(),
         access: ().into(), // empty access converts to unrestricted
         functions,
      }
   )?;
   Ok(())
}



/// Zome Callback
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
   debug!("*** init() callback START");
   /// Set Global Anchors
   //let typed_path = path.clone().into_typed(ScopedLinkType::try_from(LinkTypes::Tree)?);
   let path = Path::from(path_kind::Directory).typed(LinkKind::Members)?;
   path.ensure()?;
   /// Setup initial capabilities
   init_caps(())?;
   /// Create public encryption key and broadcast it
   create_enc_key()?;
   //PubEncKey::create_and_share()?;
   /// Done
   debug!("*** init() callback DONE");
   Ok(InitCallbackResult::Pass)
}
