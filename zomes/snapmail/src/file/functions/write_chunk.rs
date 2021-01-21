use hdk3::prelude::*;

use crate::{
    file::FileChunk,
};

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
pub fn write_chunk(input_chunk: FileChunk) -> ExternResult<HeaderHash> {
    return create_entry(&input_chunk);
}