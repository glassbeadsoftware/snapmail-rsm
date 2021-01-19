use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        hash::HashString,
    },
};
use crate::{
    entry_kind,
    file::FileManifest,
};

/// Zome function
/// Get manifest entry at given address
pub fn find_manifest(data_hash: HashString) -> ZomeApiResult<Option<FileManifest>> {
    hdk::debug(format!("get_manifest(): {}", data_hash)).ok();
    let query_result = hdk::query(entry_kind::FileManifest.into(), 0, 0)?;
    // For each File chunk
    for manifest_address in &query_result {
        // Get entry
        let entry = hdk::get_entry(manifest_address)
            .expect("No reason for get_entry() to crash")
            .expect("Should have it");
        let manifest = crate::into_typed::<FileManifest>(entry).expect("Should be a FileManifest");
        if manifest.data_hash == data_hash {
            return Ok(Some(manifest));
        }
    }
    Ok(None)
}
