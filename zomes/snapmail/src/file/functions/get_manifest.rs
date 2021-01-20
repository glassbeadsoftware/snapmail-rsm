use hdk3::prelude::*;

use crate::{
    file::FileManifest,
    utils::*,
};

/// Zome function
/// Get manifest entry at given address
/// Must be a valid address
#[hdk_extern]
pub fn get_manifest(manifest_hh: HeaderHash) -> ExternResult<FileManifest> {
    debug!("get_manifest(): {}", manifest_hh);
    /// Look for element
    let element = match get(manifest_hh, GetOptions::content())? {
        Some(element) => element,
        None => return error("No element found at given address"),
    };
    /// Check if element is a Manifest
    let maybe_FileManifest: ExternResult<FileManifest> = try_from_element(element.clone());
    if let Ok(manifest) = maybe_FileManifest {
        return Ok(manifest);
    }
    /// Done
    return error("Element at given address is not a FileManifest");
}
