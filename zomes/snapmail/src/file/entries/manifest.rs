use hdk::prelude::*;

use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_persistence_api::{
        cas::content::Address
    },
};
use crate::{
    entry_kind,
    FILE_MAX_SIZE,
};
use holochain_wasm_utils::{
    holochain_persistence_api::hash::HashString,
};

/// Entry representing a file in chunks.
/// All chunks must be committed beforehand.
#[hdk_entry(id = "file_manifest")]
#[derive(Debug, Clone)]
pub struct FileManifest {
    pub data_hash: HashString,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<Address>,
}

///
/// FIXME: Check if data_hash not already stored in source chain
pub(crate) fn validate_file(manifest: FileManifest, _maybe_validation_package: Option<ValidationPackage>)
    -> ExternResult<ValidateCallbackResult>
{
    /// Check size
    if file.orig_filesize > FILE_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A file can't be bigger than {} MiB", FILE_MAX_SIZE / (1024 * 1024))));
    }
    if file.orig_filesize < 1 {
        return Ok(ValidateCallbackResult::Invalid("A file cannot be empty".into()));
    }
    if file.chunks.len() < 1 {
        return Ok(ValidateCallbackResult::Invalid("A file must have at least one chunk".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}




