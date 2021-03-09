use hdk::prelude::*;

use crate::{
    ZomeString,
    file::{
        FileManifest,
        get_all_manifests,
    },
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindManifestOutput(pub Option<FileManifest>);


/// Zome function
/// Get manifest from file content hash
#[hdk_extern]
pub fn find_manifest(data_hash: ZomeString) -> ExternResult<FindManifestOutput> {
    debug!("find_manifest(): {}", data_hash.0);
    /// Get all FileManifest on local chain with query
    let manifest_list = get_all_manifests(())?;
    /// Check each Manifest
    for manifest in manifest_list.iter() {
        if manifest.data_hash == data_hash.0 {
            return Ok(FindManifestOutput(Some(manifest.clone())));
        }
    }
    /// Done
    Ok(FindManifestOutput(None))
}
