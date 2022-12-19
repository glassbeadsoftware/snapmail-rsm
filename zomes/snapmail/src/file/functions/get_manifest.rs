use hdk::prelude::*;
use snapmail_model::*;
use zome_utils::*;


/// Zome function
/// Get manifest entry at given address
/// Must be a valid address
#[hdk_extern]
//#[snapmail_api]
pub fn get_manifest(manifest_address: AnyDhtHash) -> ExternResult<FileManifest> {
    trace!("get_manifest(): {}", manifest_address);
    /// Look for record
    let record = match get(manifest_address, GetOptions::content())? {
        Some(record) => record,
        None => return error("No record found at given address"),
    };
    /// Check if record is a Manifest
    let maybe_FileManifest: ExternResult<FileManifest> = get_typed_from_record(record.clone());
    if let Ok(manifest) = maybe_FileManifest {
        return Ok(manifest);
    }
    /// Done
    return error("Record at given address is not a FileManifest");
}
