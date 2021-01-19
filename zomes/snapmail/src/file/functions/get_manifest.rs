use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address,
    },
};
use crate::{
    file::FileManifest,
};

/// Zome function
/// Get manifest entry at given address
pub fn get_manifest(manifest_address: Address) -> ZomeApiResult<FileManifest> {
    hdk::debug(format!("get_manifest(): {}", manifest_address)).ok();
    let maybe_entry = hdk::get_entry(&manifest_address)
        .expect("No reason for get_entry() to crash");
    if maybe_entry.is_none() {
        return Err(ZomeApiError::Internal("No entry found at given address".into()))
    }
    let manifest = crate::into_typed::<FileManifest>(maybe_entry.unwrap())?;
    Ok(manifest)
}
