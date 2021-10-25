use hdk::prelude::*;
// use hdk::prelude::element::ElementEntry;
//
// use crate::{
//    handle::*,
//    chunk::*,
//    mail::entries::*,
//    entry_kind::*,
//    utils::*,
// };

/// Zome Callback
#[hdk_extern]
fn post_commit(_: Vec<SignedHeaderHashed>) -> ExternResult<PostCommitCallbackResult> {
   //debug!("post_commit() called: {:?}", info);
   debug!("post_commit() called");
   Ok(PostCommitCallbackResult::Success)
}
