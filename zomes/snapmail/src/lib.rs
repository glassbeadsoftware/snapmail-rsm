#[macro_use] extern crate shrinkwraprs;

mod chunk;

use hdk3::prelude::*;
use test_wasm_common::*;
use chunk::*;

holochain_externs!();
holochain_wasmer_guest::host_externs!(__call_remote);

const POST_ID: &str = "post";
const POST_VALIDATIONS: u8 = 8;
#[derive(Default, SerializedBytes, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
struct Post(String);

entry_defs!(vec![Post::entry_def(), FileChunk::entry_def()]);

// returns the current agent info
fn _whoami(_: ()) -> Result<AgentInfo, WasmError> {
    Ok(agent_info!()?)
}

map_extern!(whoami, _whoami);

// map_extern!(commit_post, _commit_post);
//
//
// fn _commit_post() -> Result<holo_hash_core::EntryHash, WasmError> {
//     let post = Post("foo".into());
//     Ok(host_call!(
//         __commit_entry,
//         CommitEntryInput::new(((&post).into(), (&post).try_into()?))
//     )?)
// }

#[no_mangle]
/// always returns "foo" in a TestString
pub extern "C" fn foo(_: GuestPtr) -> GuestPtr {
    // this is whatever the dev wants we don't know
    let response = TestString::from(String::from("foo"));

    // imagine this is inside the hdk
    let response_sb: SerializedBytes = try_result!(response.try_into(), "failed to serialize TestString");
    ret!(GuestOutput::new(response_sb));
}


impl Post {
    pub fn entry_def() -> EntryDef {
        EntryDef {
            id: POST_ID.into(),
            visibility: EntryVisibility::Public,
            crdt_type: CrdtType,
            required_validations: POST_VALIDATIONS.into(),
        }
    }
}

impl From<&Post> for EntryDefId {
    fn from(_: &Post) -> Self {
        POST_ID.into()
    }
}

impl From<&Post> for EntryVisibility {
    fn from(_: &Post) -> Self {
        Self::Public
    }
}

impl From<&Post> for CrdtType {
    fn from(_: &Post) -> Self {
        Self
    }
}

impl From<&Post> for RequiredValidations {
    fn from(_: &Post) -> Self {
        POST_VALIDATIONS.into()
    }
}

impl From<&Post> for EntryDef {
    fn from(post: &Post) -> Self {
        Self {
            id: post.into(),
            visibility: post.into(),
            crdt_type: post.into(),
            required_validations: post.into(),
        }
    }
}

impl TryFrom<&Post> for Entry {
    type Error = SerializedBytesError;
    fn try_from(post: &Post) -> Result<Self, Self::Error> {
        Ok(Entry::App(post.try_into()?))
    }
}
