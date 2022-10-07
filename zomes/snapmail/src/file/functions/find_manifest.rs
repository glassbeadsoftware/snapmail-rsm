use hdk::prelude::*;
use snapmail_model::*;

use crate::file::get_all_manifests;


#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FindManifestOutput(pub Option<FileManifest>);


/// Zome function
/// Get manifest from file content hash
#[hdk_extern]
#[snapmail_api]
pub fn find_manifest(data_hash: String) -> ExternResult<FindManifestOutput> {
    debug!("find_manifest(): {}", data_hash);
    /// Get all FileManifest on local chain with query
    let manifest_list = get_all_manifests(())?;
    /* Check each Manifest */
    for manifest in manifest_list.iter() {
        if manifest.data_hash == data_hash {
            return Ok(FindManifestOutput(Some(manifest.clone())));
        }
    }
    /// Done
    Ok(FindManifestOutput(None))
}
