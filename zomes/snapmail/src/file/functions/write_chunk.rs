use hdk3::prelude::*;

use crate::{
    file::FileChunk,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct WriteChunkInput(pub FileChunk);

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
pub fn write_chunk(input_chunk: WriteChunkInput) -> ExternResult<HeaderHash> {
    debug!(" write_chunk() {:?}", input_chunk);
    return create_entry(&input_chunk.0);
}
