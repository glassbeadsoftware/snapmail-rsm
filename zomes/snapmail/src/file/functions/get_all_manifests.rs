use hdk::prelude::*;

use crate::{
    entry_kind,
    file::FileManifest,
};

/// Zome function
/// Get all manifests stored in our source chain
pub fn get_all_manifests() -> ZomeApiResult<Vec<FileManifest>> {
    hdk::debug(format!("get_all_manifests()")).ok();
    let query_result = hdk::query(entry_kind::FileManifest.into(), 0, 0)?;
    // For each File chunk
    let mut manifest_list = Vec::new();
    for manifest_address in &query_result {
        // Get entry
        let entry = hdk::get_entry(manifest_address)
            .expect("No reason for get_entry() to crash")
            .expect("Should have it");
        let manifest = crate::into_typed::<FileManifest>(entry).expect("Should be a FileManifest");
        // Add to list
        manifest_list.push(manifest);
    }
    Ok(manifest_list)
}
