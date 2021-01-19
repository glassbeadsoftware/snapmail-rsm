use hdk::{
    error::{ZomeApiResult, ZomeApiError},
    holochain_persistence_api::{
        cas::content::Address
    },
    holochain_core_types::{
        entry::Entry,
        time::Timeout,
    },
};

use crate::{entry_kind, file::{FileManifest, FileChunk}, DirectMessageProtocol, DirectMessageProtocol::*, AgentAddress};


///
pub(crate) fn request_chunk_by_dm(destination: AgentAddress, chunk_address: Address) -> ZomeApiResult<Option<FileChunk>> {
    hdk::debug(format!("request_manifest_by_dm(): {}", chunk_address)).ok();
    //   Create DM
    let payload = serde_json::to_string(&DirectMessageProtocol::RequestChunk(chunk_address)).unwrap();
    //   Send DM
    let result = hdk::send(
        destination,
        payload,
        Timeout::new(crate::DIRECT_SEND_CHUNK_TIMEOUT_MS),
    );
    hdk::debug(format!("RequestChunk result = {:?}", result)).ok();
    //   Check Response
    if let Err(e) = result {
        return Err(ZomeApiError::Internal(format!("hdk::send() of RequestChunk failed: {}", e)));
    }
    let response = result.unwrap();
    hdk::debug(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(_e) = maybe_msg {
        return Err(ZomeApiError::Internal("hdk::send() of RequestChunk failed 2".into()))
    }
    match maybe_msg.unwrap() {
        DirectMessageProtocol::Chunk(chunk) => {
            // Commit FileChunk
            let chunk_entry = Entry::App(entry_kind::FileChunk.into(), chunk.clone().into());
            let maybe_address = hdk::commit_entry(&chunk_entry);
            if let Err(err) = maybe_address {
                let response_str = "Failed committing RequestChunk";
                hdk::debug(format!("{}: {}", response_str, err)).ok();
                return Err(err);
            }
            let chunk_address = maybe_address.unwrap();
            hdk::debug(format!("received chunk_address: {}", chunk_address)).ok();
            Ok(Some(chunk))
        },
        UnknownEntry => Ok(None),
        _ => Err(ZomeApiError::Internal("hdk::send() of RequestChunk failed 3".into())),
    }
}


///
pub(crate) fn request_manifest_by_dm(destination: AgentAddress, manifest_address: Address) -> ZomeApiResult<Option<FileManifest>> {
    hdk::debug(format!("request_manifest_by_dm(): {}", manifest_address)).ok();
    //   Create DM
    let payload = serde_json::to_string(&DirectMessageProtocol::RequestManifest(manifest_address)).unwrap();
    //   Send DM
    let result = hdk::send(
        destination,
        payload,
        Timeout::new(crate::DIRECT_SEND_CHUNK_TIMEOUT_MS),
    );
    hdk::debug(format!("RequestManifest result = {:?}", result)).ok();
    //   Check Response
    if let Err(e) = result {
        return Err(ZomeApiError::Internal(format!("hdk::send() of RequestManifest failed: {}", e)));
    }
    let response = result.unwrap();
    hdk::debug(format!("Received response: {:?}", response)).ok();
    let maybe_msg: Result<DirectMessageProtocol, _> = serde_json::from_str(&response);
    if let Err(_e) = maybe_msg {
        return Err(ZomeApiError::Internal("hdk::send() of RequestManifest failed 2".into()))
    }
    match maybe_msg.unwrap() {
        DirectMessageProtocol::FileManifest(manifest) => {
            // Commit FileManifest
            let manifest_entry = Entry::App(entry_kind::FileManifest.into(), manifest.clone().into());
            let maybe_address = hdk::commit_entry(&manifest_entry);
            if let Err(err) = maybe_address {
                let response_str = "Failed committing FileManifest";
                hdk::debug(format!("{}: {}", response_str, err)).ok();
                return Err(err);
            }
            let manifest_address = maybe_address.unwrap();
            hdk::debug(format!("received manifest_address: {}",  manifest_address)).ok();
            Ok(Some(manifest))
        },
        UnknownEntry => Ok(None),
        _ => Err(ZomeApiError::Internal("hdk::send() of FileManifest failed 3".into())),
    }
}
