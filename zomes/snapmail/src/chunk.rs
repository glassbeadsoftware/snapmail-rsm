use hdk3::prelude::*;

use crate::ZomeString;
use crate::utils::*;

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file chunk.
// #[hdk_entry(
// id = "file_chunk",
// required_validations = 5,
// required_validation_type = "full",
// visibility = "private"
// )]
#[hdk_entry(id = "file_chunk")]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FileChunk {
    pub data_hash: String,
    pub chunk_index: usize,
    pub chunk: String,
}

// pub(crate) fn validate_chunk(validation_data: hdk::EntryValidationData<FileChunk>) -> Result<(), String> {
//     match validation_data {
//         EntryValidationData::Create{entry: file, validation_data: _} => {
//             // Check size
//             if file.chunk.len() > CHUNK_MAX_SIZE {
//                 return Err(format!("A file chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024));
//             }
//             return Ok(());
//         },
//         EntryValidationData::Modify{new_entry: _, old_entry: _, old_entry_header:_, validation_data: _} => {
//             return Err("Update chunk not allowed".into());
//         },
//         EntryValidationData::Delete{old_entry: _, old_entry_header: _, validation_data:_} => {
//             return Ok(());
//         }
//     }

// }

impl FileChunk {
    pub fn new(data_hash: String, chunk_index: usize, chunk: String) -> Self {
        Self {
            data_hash,
            chunk_index,
            chunk,
        }
    }
}

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
pub fn write_chunk(
    file_chunk: FileChunk
) -> ExternResult<HeaderHash> {
    debug!("fileChunk: {:?}", file_chunk);
    let res = create_entry(&file_chunk)?;
    debug!("commit_result: {:?}", res);
    Ok(res)
}


/// Zome function
#[hdk_extern]
pub fn get_chunk_hash(
    file_chunk: FileChunk
) -> ExternResult<EntryHash> {
    debug!("fileChunk: {:?}", file_chunk);
    let res = hash_entry(&file_chunk)?;
    debug!("entry_hash_result: {:?}", res);
    Ok(res)
}


/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
#[hdk_extern]
pub fn get_chunk(chunk_address: EntryHash) -> ExternResult<ZomeString> {
//pub fn _get_chunk(chunk_address: EntryHash) -> Result<MyString, WasmError> {
        //debug!(format!("chunk_address_raw: {:?}", chunk_address_raw));
    //let chunk_address = HoloHash::<hash_type::Entry>::from_raw_bytes_and_type(chunk_address_raw.to_vec(), hash_type::Entry::Content);
    debug!("chunk_address: {:?}", chunk_address);
    let maybe_element = get(chunk_address, GetOptions::latest())
        .expect("No reason for get() to crash");
    if maybe_element.is_none() {
        return Ok(ZomeString(String::new().into()));
    }
    let chunk_element = maybe_element.unwrap();
    let maybe_chunk: Option<FileChunk> = chunk_element.entry().to_app_option()?;
    if maybe_chunk.is_none() {
        return Ok(ZomeString(String::new().into()));
    }
    let chunk = maybe_chunk.unwrap();
    Ok(ZomeString(chunk.chunk.into()))
}


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct SendChunkInput {
    pub agent_pubkey: AgentPubKey,
    pub file_chunk: FileChunk,
}

/// Zome function
#[hdk_extern]
fn send_chunk(input: SendChunkInput) -> ExternResult<HeaderHash> {
    debug!("to_agent: {:?}", input.agent_pubkey);
    let chunk = input.file_chunk;
    debug!("dbg chunk: {:?}", chunk);
    let response: ZomeCallResponse = call_remote(
        input.agent_pubkey,
        zome_info()?.zome_name,
        "write_chunk".to_string().into(),
        None,
        &chunk,
    )?;
    debug!("response2: {:?}", response);
    match response {
        ZomeCallResponse::Ok(guest_output) => {
            debug!("guest_output: {:?}", guest_output);
            let hash: HeaderHash = guest_output.into_inner().try_into()?;
            debug!("hash_output: {:?}", hash);
            Ok(hash)
        },
        ZomeCallResponse::NetworkError(_msg) => unreachable!(), // FIXME
        // we're just panicking here because our simple tests can always call set_access before
        // calling whoami, but in a real app you'd want to handle this by returning an `Ok` with
        // something meaningful to the extern's client
        //ZomeCallResponse::Unauthorized => unreachable!(),
        ZomeCallResponse::Unauthorized(_,_,_,_) => error(
            "{\"code\": \"000\", \"message\": \"[Unauthorized] write_chunk\"}"),
    }
    //Ok(result.try_into()?)
}
