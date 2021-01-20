use hdk3::prelude::*;

use crate::{
    file::{
        FileManifest,
        get_all_manifests,
    },
};

/// Zome function
/// Get manifest from file content hash
#[hdk_extern]
pub fn find_manifest(data_hash: String) -> ExternResult<Option<FileManifest>> {
    debug!("find_manifest(): {}", data_hash);
    /// Get all FileManifest on local chain with query
    let manifest_list = get_all_manifests()?;
    /// Check each Manifest
    for manifest in &manifest_list {
        if manifest.data_hash == data_hash {
            return Ok(Some(manifest));
        }
    }
    /// Done
    Ok(None)
}
