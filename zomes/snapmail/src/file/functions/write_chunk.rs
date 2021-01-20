use hdk3::prelude::*;

use crate::{
    file::FileChunk,
};

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
pub fn write_chunk(input_chunk: FileChunk) -> ExternResult<HeaderHash> {
    let hh = create_entry(&input_chunk);
    hh
}