use hdk::prelude::*;

use hdk::{
    holochain_persistence_api::{
        cas::content::Address, hash::HashString,
    },
    holochain_core_types::{
        entry::Entry,
    },
};
use crate::{
    entry_kind,
    file::FileManifest,
};

/// Zome function
/// Write file manifest to source chain
pub fn write_manifest(
    data_hash: HashString,
    filename: String,
    filetype: String,
    orig_filesize: usize,
    chunks: Vec<Address>,
) -> ZomeApiResult<Address> {
    let manifest = FileManifest {
        data_hash, filename, filetype, orig_filesize, chunks
    };
    let file_entry = Entry::App(entry_kind::FileManifest.into(), manifest.into());
    let maybe_file_address = hdk::commit_entry(&file_entry);
    maybe_file_address
}
