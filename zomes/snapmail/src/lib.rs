#[macro_use] extern crate shrinkwraprs;

mod chunk;

//mod handle;

/*
mod file;
mod mail;
mod utils;
mod protocol;
mod signal_protocol;
mod globals;
mod link_kind;
mod entry_kind;
*/


use hdk3::prelude::*;
//use test_wasm_common::*;
use chunk::*;

/*
pub use signal_protocol::*;
pub use protocol::*;
pub use utils::*;
pub use globals::*;
pub use entry_kind::*;

use mail::entries::*;
*/

holochain_externs!();
//holochain_wasmer_guest::host_externs!(__call_remote);

/*
const POST_ID: &str = "post";
const POST_VALIDATIONS: u8 = 8;
#[derive(Default, SerializedBytes, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
struct Post(String);
*/

#[hdk_entry(
id = "post",
required_validations = 5,
required_validation_type = "full"
)]
struct Post(String);

entry_defs![Post::entry_def(), FileChunk::entry_def()];


// returns the current agent info
#[hdk_extern]
fn whoami(_: ()) -> Result<AgentInfo, WasmError> {
    Ok(agent_info!()?)
}


#[hdk_extern]
fn set_access(_: ()) -> ExternResult<()> {
    let mut functions: GrantedFunctions = HashSet::new();
    //functions.insert((zome_info!()?.zome_name, "whoami".into()));
    functions.insert((zome_info!()?.zome_name, "write_chunk".into()));
    create_cap_grant!(
        CapGrantEntry {
            tag: "".into(),
            // empty access converts to unrestricted
            access: ().into(),
            functions,
        }
    )?;
    Ok(())
}

#[hdk_extern]
fn whoarethey(agent_pubkey: AgentPubKey) -> ExternResult<AgentInfo> {
    let response: ZomeCallResponse = call_remote!(
        agent_pubkey,
        zome_info!()?.zome_name,
        "whoami".to_string().into(),
        None,
        ().try_into()?
    )?;

    match response {
        ZomeCallResponse::Ok(guest_output) => Ok(guest_output.into_inner().try_into()?),
        // we're just panicking here because our simple tests can always call set_access before
        // calling whoami, but in a real app you'd want to handle this by returning an `Ok` with
        // something meaningful to the extern's client
        ZomeCallResponse::Unauthorized => unreachable!(),
    }
}


//
//
// fn _commit_post() -> Result<holo_hash_core::EntryHash, WasmError> {
//     let post = Post("foo".into());
//     Ok(host_call!(
//         __commit_entry,
//         CommitEntryInput::new(((&post).into(), (&post).try_into()?))
//     )?)
// }

// #[no_mangle]
// /// always returns "foo" in a TestString
// pub extern "C" fn foo(_: GuestPtr) -> GuestPtr {
//     // this is whatever the dev wants we don't know
//     let response = TestString::from(String::from("foo"));
//
//     // imagine this is inside the hdk
//     let response_sb: SerializedBytes = try_result!(response.try_into(), "failed to serialize TestString");
//     ret!(GuestOutput::new(response_sb));
// }

/*
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
*/