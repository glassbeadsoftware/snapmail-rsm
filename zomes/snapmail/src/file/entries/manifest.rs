use hdk::prelude::*;

use crate::{
    FILE_MAX_SIZE,
};

/// Entry representing a file in chunks.
/// All chunks must be committed beforehand.
#[hdk_entry(id = "file_manifest")]
#[derive(Clone, PartialEq)]
pub struct FileManifest {
    pub data_hash: String,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<EntryHash>,
}

///
/// TODO: Check if data_hash not already stored in source chain
pub(crate) fn validate_file(manifest: FileManifest, _maybe_validation_package: Option<ValidationPackage>)
    -> ExternResult<ValidateCallbackResult>
{
    /// Check size
    if manifest.orig_filesize > FILE_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A file can't be bigger than {} MiB", FILE_MAX_SIZE / (1024 * 1024))));
    }
    if manifest.orig_filesize < 1 {
        return Ok(ValidateCallbackResult::Invalid("A file cannot be empty".into()));
    }
    if manifest.chunks.len() < 1 {
        return Ok(ValidateCallbackResult::Invalid("A file must have at least one chunk".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}




