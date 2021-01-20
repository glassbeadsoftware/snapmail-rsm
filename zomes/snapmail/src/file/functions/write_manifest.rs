use hdk3::prelude::*;

use crate::{
    file::FileManifest,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct WriteManifestInput {
    pub data_hash: String,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<HeaderHash>,
}

/// Zome function
/// Write file manifest to source chain
#[hdk_extern]
pub fn write_manifest(input: WriteManifestInput) -> ExternResult<HeaderHash> {
    let manifest = FileManifest {
        data_hash: input.data_hash,
        filename: input.filename,
        filetype: input.filetype,
        orig_filesize: input.orig_filesize,
        chunks: input.chunks,
    };
    let hh = create_entry(&manifest)?;
    hh
}
