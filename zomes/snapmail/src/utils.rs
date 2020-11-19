use std::convert::TryFrom;

use hdk3::prelude::*;

pub type EntryAndHash<T> = (T, HeaderHash, EntryHash);
pub type OptionEntryAndHash<T> = Option<EntryAndHash<T>>;

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
pub fn snapmail_now() -> u64 {
    let now = sys_time().expect("sys_time() should always work");
    now.as_secs()
}

///
pub fn hh_to_eh(hh: HeaderHash) -> ExternResult<EntryHash> {
    let element = get(hh, GetOptions)?.expect("Converting non existing HeaderHash");
    let eh = element.header().entry_hash().expect("Converting HeaderHash which does not have an Entry");
    Ok(eh.clone())
}


///
pub fn get_typed_entry<T: TryFrom<SerializedBytes>>(
    hash: HeaderHash,
) -> ExternResult<(EntryHash, T)> {
    match get(hash.clone(), GetOptions)? {
        Some(element) => {
            let eh = element.header().entry_hash().expect("Converting HeaderHash which does not have an Entry");
            Ok((eh.clone(), try_from_element(element)?))
        },
        None => crate::error("Entry not found"),
    }
}


///
pub fn try_get_and_convert<T: TryFrom<SerializedBytes>>(
    entry_hash: EntryHash,
) -> ExternResult<(EntryHash, T)> {
    match get(entry_hash.clone(), GetOptions)? {
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

// #[derive(Serialize, Deserialize, SerializedBytes)]
// struct StringLinkTag(String);
// pub fn link_tag(tag: &str) -> LinkTag {
//     let sb: SerializedBytes = StringLinkTag(tag.into())
//        .try_into()
//        .expect("StringLinkTag should convert to SerializedBytes");
//     LinkTag(sb.bytes().clone())
// }

/// From Connor @acorn ///

pub fn get_header_hash(shh: element::SignedHeaderHashed) -> HeaderHash {
    shh.header_hashed().as_hash().to_owned()
}

pub fn get_latest_for_entry<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_hash: EntryHash,
) -> ExternResult<OptionEntryAndHash<T>> {
    // First, make sure we DO have the latest header_hash address
    let maybe_latest_header_hash = match get_details(entry_hash, GetOptions)? {
        Some(Details::Entry(details)) => match details.entry_dht_status {
            metadata::EntryDhtStatus::Live => match details.updates.len() {
                // pass out the header associated with this entry
                0 => Some(get_header_hash(details.headers.first().unwrap().to_owned())),
                _ => {
                    let mut sortlist = details.updates.to_vec();
                    // unix timestamp should work for sorting
                    sortlist.sort_by_key(|update| update.header().timestamp().0);
                    // sorts in ascending order, so take the last element
                    let last = sortlist.last().unwrap().to_owned();
                    Some(get_header_hash(last))
                }
            },
            metadata::EntryDhtStatus::Dead => None,
            _ => None,
        },
        _ => None,
    };

    // Second, go and get that element, and return it and its header_address
    match maybe_latest_header_hash {
        Some(latest_header_hash) => match get(latest_header_hash, GetOptions)? {
            Some(element) => match element.entry().to_app_option::<T>()? {
                Some(entry) => Ok(Some((
                    entry,
                    match element.header() {
                        // we DO want to return the header for the original
                        // instead of the updated, in our case
                        Header::Update(update) => update.original_header_address.clone(),
                        Header::Create(_) => element.header_address().clone(),
                        _ => unreachable!("Can't have returned a header for a nonexistent entry"),
                    },
                    element.header().entry_hash().unwrap().to_owned(),
                ))),
                None => Ok(None),
            },
            None => Ok(None),
        },
        None => Ok(None),
    }
}
