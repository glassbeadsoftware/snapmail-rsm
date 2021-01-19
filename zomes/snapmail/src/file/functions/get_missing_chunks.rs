use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
};
use crate::{
    file::{FileManifest},
    AgentAddress,
};
use crate::file::dm::request_chunk_by_dm;


/// Zome Function
/// Request missing chunks for an attachment
/// Returns number of remaining missing chunks
pub fn get_missing_chunks(from: AgentAddress, manifest_address: Address) -> ZomeApiResult<u32> {
    let manifest = hdk::utils::get_as_type::<FileManifest>(manifest_address.clone())?;
    let chunk_count = manifest.chunks.len();
    let mut missing = 0;
    let mut i = -1;
    for chunk_address in manifest.chunks {
        i += 1;
        let chunk_str = format!("Chunk {}/{}", i, chunk_count);
        // Skip if chunk already held
        let maybe_entry = hdk::get_entry(&chunk_address)?;
        if let Some(_) = maybe_entry {
            hdk::debug(format!("{} already held", chunk_str)).ok();
            continue;
        }
        // Request chunk
        let maybe_maybe_chunk = request_chunk_by_dm(from.clone(), chunk_address);
        // Notify failure
        if let Err(err) = maybe_maybe_chunk {
            let response_str = format!("{} failed", chunk_str);
            hdk::debug(format!("{}: {}", response_str, err)).ok();
            missing += 1;
            continue;
        }
        if let None = maybe_maybe_chunk.unwrap() {
            hdk::debug(format!("{} unknown from source agent", chunk_str)).ok();
            missing += 1;
            continue;
        }
    }
    Ok(missing)
}
