use hdk3::prelude::*;
use test_wasm_common::*;
use holochain_wasmer_guest::*;
use holochain_zome_types::*;
use holo_hash::HoloHash;
use holo_hash::hash_type;

holochain_wasmer_guest::holochain_externs!();

// use hdk::{
//     entry_definition::ValidatingEntryType,
//     holochain_persistence_api::{
//         cas::content::Address, hash::HashString,
//     },
// };

pub const CHUNK_MAX_SIZE: usize = 200 * 1024;

map_extern!(write_chunk, _write_chunk);
map_extern!(get_chunk, _get_chunk);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct MyString(String);

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct MyRaw(Vec<u8>);

//-------------------------------------------------------------------------------------------------
// Definition
//-------------------------------------------------------------------------------------------------

/// Entry representing a file chunk.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, SerializedBytes)]
pub struct FileChunk {
    pub data_hash: String,
    pub chunk_index: usize,
    pub chunk: String,
}

impl FileChunk {
    pub fn entry_def() -> EntryDef {
        EntryDef {
            id: "file_chunk".into(),
            visibility: EntryVisibility::Private,
            crdt_type: CrdtType,
            required_validations: 8.into(),
        }
    }
}

impl From<&FileChunk> for EntryDefId {
    fn from(_: &FileChunk) -> Self {
        "file_chunk".into()
    }
}

impl From<&FileChunk> for EntryVisibility {
    fn from(_: &FileChunk) -> Self {
        Self::Private
    }
}

impl From<&FileChunk> for CrdtType {
    fn from(_: &FileChunk) -> Self {
        Self
    }
}

impl From<&FileChunk> for RequiredValidations {
    fn from(_: &FileChunk) -> Self {
        8.into()
    }
}

impl From<&FileChunk> for EntryDef {
    fn from(post: &FileChunk) -> Self {
        Self {
            id: post.into(),
            visibility: post.into(),
            crdt_type: post.into(),
            required_validations: post.into(),
        }
    }
}

impl TryFrom<&FileChunk> for Entry {
    type Error = SerializedBytesError;
    fn try_from(post: &FileChunk) -> Result<Self, Self::Error> {
        Ok(Entry::App(post.try_into()?))
    }
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
pub fn _write_chunk(
    //data_hash: String,
    //chunk_index: usize,
    chunk: MyString,
) -> Result<EntryHash, WasmError> {
    let data_hash = "QM324rdx".to_string();
    let chunk_index = 0;
    let initial_file = FileChunk::new(data_hash.clone(), chunk_index, chunk.0);
    // let file_entry = Entry::App(entry_kind::FileChunk.into(), initial_file.into());
    // let maybe_file_address = hdk::commit_entry(&file_entry);

    debug!(format!("data_hash: {}", data_hash)).ok();

    let res: EntryHash = host_call!(
        __commit_entry,
        CommitEntryInput::new(((&initial_file).into(), (&initial_file).try_into()?))
    )?;
    debug!(format!("commit_result: {:?}", res)).ok();
    debug!(format!("commit_result: {:?}", res.hash_type())).ok();
    debug!(format!("commit_result: {:?}", res.get_raw())).ok();
    Ok(res)
}

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
pub fn _get_chunk(chunk_address_str: /*EntryHash*/ MyRaw) -> Result<MyString, WasmError> {
    debug!(format!("!!! getChunk : {:?}", chunk_address_str)).ok();
    //let hardcoded = [71, 198, 64, 23, 140, 236, 238, 52, 45, 24, 23, 49, 174, 76, 245, 96, 159, 177, 79, 237, 236, 216, 152, 112, 146, 158, 213, 243, 212, 178, 164, 145, 204, 155, 174, 205];
    let chunk_address = HoloHash::<hash_type::Entry>::from_raw_bytes_and_type(chunk_address_str.to_vec(), hash_type::Entry::Content);
    //let chunk_address = EntryHash::new();
        debug!(format!("chunk_address: {:?}", chunk_address)).ok();
    //hdk::debug(format!("get_chunk(): {}", chunk_address)).ok();
    let maybe_entry = get_entry!(chunk_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Ok(MyString("".into()));
    }

    let chunk = match maybe_entry.unwrap() {
        Entry::App(entry_value) => FileChunk::try_from(entry_value).expect("should convert"),
        _ =>  return Ok(MyString(String::new().into())),
    };

    // Ok((chunk.chunk_index, chunk.chunk))
    Ok(MyString(chunk.chunk.into()))
}

//
// #[no_mangle]
// pub extern "C" fn _get_chunk(ptr: GuestPtr) -> GuestPtr {
//     // Convert input
//     let input: HostInput = host_args!(ptr);
//     let sb: SerializedBytes = input.into_inner();
//     let chunk_address: EntryHash = match EntryHash::try_from(input.into_inner()) {
//
//     };
//
//     // Do actual processing
//     let maybe_chunk = __get_chunk(chunk_address);
//
//     // Convert ouput
//     return match maybe_chunk {
//         Ok(chunk) => {
//             let response_sb: SerializedBytes = try_result!(chunk.try_into(), "failed to serialize chunk");
//             ret!(GuestOutput::new(response_sb));
//         },
//         Err(msg) => {
//             ret_err!(msg);
//         }
//     };
// }