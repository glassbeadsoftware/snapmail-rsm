use hdk3::prelude::*;

use crate::{
    entry_kind,
    file::FileManifest,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeManifestVec(Vec<FileManifest>);

/// Zome function
/// Get all manifests stored in our source chain
#[hdk_extern]
pub fn get_all_manifests() -> ExternResult<ZomeManifestVec> {
    debug!("get_all_manifests()");
    /// Get all FileManifest on local chain with query
    let query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(EntryKind::FileManifest.as_type());
    let query_result = query(query_args);
    if let Err(err) = query_result {
        debug!("find_manifest() query_result failed: {:?}", err);
        //return Err(hdk3::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let manifest_elements: Vec<Element> = query_result.unwrap().0;
    /// For each File Manifest element, get its entry
    let mut manifest_list = Vec::new();
    for manifest_el in &manifest_elements {
        let manifest: FileManifest = try_from_element(manifest_el)?;
        manifest_list.push(manifest);
    }
    /// Done
    Ok(ZomeManifestVec(manifest_list))
}
