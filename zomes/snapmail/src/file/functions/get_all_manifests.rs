use hdk::prelude::*;

use crate::{
    entry_kind::*,
    file::FileManifest,
    utils::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ZomeManifestVec(Vec<FileManifest>);

/// Zome function
/// Get all manifests stored in our source chain
#[hdk_extern]
pub fn get_all_manifests(_: ()) -> ExternResult<ZomeManifestVec> {
    debug!("get_all_manifests()");
    /// Get all FileManifest on local chain with query
    let query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(EntryKind::FileManifest.as_type());
    let query_result = query(query_args);
    if let Err(err) = query_result {
        debug!("find_manifest() query_result failed: {:?}", err);
        //return Err(hdk::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let manifest_elements: Vec<Element> = query_result.unwrap().0;
    /// For each File Manifest element, get its entry
    let mut manifest_list = Vec::new();
    for manifest_el in &manifest_elements {
        let manifest: FileManifest = get_typed_from_el(manifest_el.clone())?;
        manifest_list.push(manifest);
    }
    /// Done
    Ok(ZomeManifestVec(manifest_list))
}
