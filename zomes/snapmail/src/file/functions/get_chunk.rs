use hdk3::prelude::*;

use crate::{
    file::FileChunk,
    utils::*,
};

/// Zome function
/// Get chunk index and chunk as base64 string in local source chain at given address
/// Must be a valid address
#[hdk_extern]
pub fn get_chunk(chunk_hh: HeaderHash) -> ExternResult<String> {
    debug!("get_chunk(): {}", chunk_hh);
    /// Look for element
    let element = match get(chunk_hh, GetOptions::content())? {
        Some(element) => element,
        None => return error("No element found at given address"),
    };
    /// Check if element is a Manifest
    let maybe_FileChunk: ExternResult<FileChunk> = try_from_element(element.clone());
    if let Ok(chunk) = maybe_FileChunk {
        return Ok(chunk.chunk);
    }
    /// Done
    return error("Element at given address is not a FileChunk");
}
