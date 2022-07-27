use hdk::prelude::*;

use crate::{
    file::FileManifest,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WriteManifestInput {
    pub data_hash: String,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<EntryHash>,
}

/// Zome function
/// Write file manifest to source chain
#[hdk_extern]
#[snapmail_api]
pub fn write_manifest(input: WriteManifestInput) -> ExternResult<ActionHash> {
    let manifest = FileManifest {
        data_hash: input.data_hash,
        filename: input.filename,
        filetype: input.filetype,
        orig_filesize: input.orig_filesize,
        chunks: input.chunks,
    };
    return create_entry(&manifest);
}
