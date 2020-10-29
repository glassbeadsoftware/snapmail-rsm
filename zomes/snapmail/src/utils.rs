use std::convert::TryFrom;

use hdk3::prelude::*;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
}

/*
use timestamp::Timestamp;

pub(crate) fn to_timestamp(duration: Duration) -> Timestamp {
    Timestamp(duration.as_secs() as i64, duration.subsec_nanos())
}
*/

/// Returns number of seconds since UNIX_EPOCH
/// TODO: Time not available in WASM
pub fn snapmail_now() -> u64 {
    42
    // hdk::now()
// let duration_since_epoch = SystemTime::now();
//        .duration_since(SystemTime::UNIX_EPOCH)
//        .expect("System time must not be before UNIX EPOCH");
//    duration_since_epoch.as_secs()
}

///
pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(
    entry_hash: EntryHash,
) -> ExternResult<(EntryHash, T)> {
    match get!(entry_hash.clone())? {
        Some(element) => Ok((entry_hash, try_from_element(element)?)),
        None => crate::error("Entry not found"),
    }
}

///
pub fn try_from_element<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => try_from_entry::<T>(entry.clone()),
        _ => crate::error("Could not convert element"),
    }
}

///
pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Could not convert entry"),
        },
        _ => crate::error("Could not convert entry"),
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}