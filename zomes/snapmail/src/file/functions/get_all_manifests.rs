use hdk::prelude::*;
use snapmail_model::*;
use zome_utils::*;

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZomeManifestVec(Vec<FileManifest>);

/// Zome function
/// Get all manifests stored in our source chain
#[hdk_extern]
//#[snapmail_api]
pub fn get_all_manifests(_: ()) -> ExternResult<ZomeManifestVec> {
    trace!("get_all_manifests()");
    /// Get all FileManifest on local chain with query
    let query_args = ChainQueryFilter::default()
       .include_entries(true)
       .entry_type(UnitEntryTypes::FileManifest.try_into().unwrap());
    let query_result = query(query_args);
    if let Err(err) = query_result {
        error!("find_manifest() query_result failed: {:?}", err);
        //return Err(hdk::error::HdkError::SerializedBytes(err));
        return Err(err);
    }
    let manifest_elements: Vec<Record> = query_result.unwrap();
    /// For each File Manifest record, get its entry
    let mut manifest_list = Vec::new();
    for manifest_el in &manifest_elements {
        let manifest: FileManifest = get_typed_from_record(manifest_el.clone())?;
        manifest_list.push(manifest);
    }
    /// Done
    Ok(ZomeManifestVec(manifest_list))
}
