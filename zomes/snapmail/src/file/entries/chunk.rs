use hdk::prelude::*;

use crate::{
    CHUNK_MAX_SIZE,
};

/// Entry representing a file chunk.
#[hdk_entry(id = "file_chunk", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct FileChunk {
    pub data_hash: String,
    pub chunk_index: usize,
    pub chunk: String,
}

impl FileChunk {
    pub fn new(data_hash: String, chunk_index: usize, chunk: String) -> Self {
        Self {
            data_hash,
            chunk_index,
            chunk,
        }
    }
}

///
pub(crate) fn validate_chunk(chunk: FileChunk, _maybe_validation_package: Option<ValidationPackage>)
    -> ExternResult<ValidateCallbackResult>
{
    /// Check size
    if chunk.chunk.len() > CHUNK_MAX_SIZE {
        return Ok(ValidateCallbackResult::Invalid(
            format!("A file chunk can't be bigger than {} KiB", CHUNK_MAX_SIZE / 1024)));
    }
    Ok(ValidateCallbackResult::Valid)
}