use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
};
use crate::{
    file::FileChunk,
};

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
pub fn get_chunk(chunk_address: Address) -> ZomeApiResult<String> {
    hdk::debug(format!("get_chunk(): {}", chunk_address)).ok();
    let maybe_entry = hdk::get_entry(&chunk_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Err(ZomeApiError::Internal("No chunk found at given address".into()))
    }
    let chunk = crate::into_typed::<FileChunk>(maybe_entry.unwrap())?;
    // Ok((chunk.chunk_index, chunk.chunk))
    Ok(chunk.chunk)
}
