use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address, hash::HashString,
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use crate::{
    entry_kind,
    file::FileChunk,
};

/// Zome function
/// Write base64 file as string to source chain
pub fn write_chunk(
    data_hash: HashString,
    chunk_index: usize,
    chunk: String,
) -> ZomeApiResult<Address> {
    let initial_file = FileChunk::new(data_hash.clone(), chunk_index, chunk);
    let file_entry = Entry::App(entry_kind::FileChunk.into(), initial_file.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    maybe_file_address
}