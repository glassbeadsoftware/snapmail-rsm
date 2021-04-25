use hdk::prelude::*;

use crate::{
    file::FileChunk,
    utils::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WriteChunkInput(pub FileChunk);

/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
#[cfg_attr(not(target_arch = "wasm32"), snapmail_api)]
pub fn write_chunk(input_chunk: WriteChunkInput) -> ExternResult<EntryHash> {
    trace!(" write_chunk() {:?}", input_chunk);
    let chunk_hh = create_entry(&input_chunk.0)?;
    return hh_to_eh(chunk_hh);
}
