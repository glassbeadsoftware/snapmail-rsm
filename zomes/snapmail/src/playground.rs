use hdk3::prelude::*;

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
pub struct Post(String);


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
