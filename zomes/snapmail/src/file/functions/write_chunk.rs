use hdk::prelude::*;

use crate::{
    file::FileChunk,
};


/// Zome function
/// Write base64 file as string to source chain
#[hdk_extern]
#[snapmail_api]
pub fn write_chunk(input_chunk: FileChunk) -> ExternResult<EntryHash> {
    trace!(" write_chunk() {:?}", input_chunk);
    let eh = hash_entry(input_chunk.clone());
    let _hh = create_entry(&input_chunk)?;
    return eh;
}
