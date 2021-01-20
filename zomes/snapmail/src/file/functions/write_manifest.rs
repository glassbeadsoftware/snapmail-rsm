use hdk3::prelude::*;

use crate::{
    entry_kind,
    file::FileManifest,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct WriteManifestInput {
    pub data_hash: HashString,
    pub filename: String,
    pub filetype: String,
    pub orig_filesize: usize,
    pub chunks: Vec<Address>,
}

/// Zome function
/// Write file manifest to source chain
#[hdk_extern]
pub fn write_manifest(input: WriteManifestInput) -> ExternResult<HeaderHash> {
    let manifest = FileManifest {
        data_hash: input.data_hash,
        filename: input.filename,
        filetype: inpuit.filetype,
        orig_filesize: input.orig_filesize,
        chunks: input.chunks,
    };
    let hh = create_entry(&manifest)?;
    hh
}
