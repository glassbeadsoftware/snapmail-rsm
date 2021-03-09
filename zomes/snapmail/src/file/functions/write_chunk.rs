use hdk::prelude::*;

use crate::{
    file::FileChunk,
    utils::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct WriteChunkInput(pub FileChunk);

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
pub fn write_chunk(input_chunk: WriteChunkInput) -> ExternResult<EntryHash> {
    debug!(" write_chunk() {:?}", input_chunk);
    let chunk_hh = create_entry(&input_chunk.0)?;
    return hh_to_eh(chunk_hh);
}
