use std::convert::TryFrom;

use hdk::prelude::*;

pub type TypedEntryAndHash<T> = (T, HeaderHash, EntryHash);
pub type OptionTypedEntryAndHash<T> = Option<TypedEntryAndHash<T>>;

pub fn error<T>(reason: &str) -> ExternResult<T> {
    //Err(HdkError::Wasm(WasmError::Zome(String::from(reason))))
    Err(WasmError::Guest(String::from(reason)))
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

/// Get Element at address using query()
pub fn get_entry_type(eh: EntryHash) -> ExternResult<EntryType> {
    let maybe_element = get(eh, GetOptions::latest())?;
    if maybe_element.is_none() {
        return error("no element found for entry_hash");
    }
    let element = maybe_element.unwrap();
    let entry_type = element.header().entry_type().expect("should have entry type").clone();
    Ok(entry_type)
}

/// Get Element at address using query()
pub fn get_local_from_hh(hh: HeaderHash) -> ExternResult<Element> {
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true);
    let maybe_vec = query(inmail_query_args);
    if let Err(err) = maybe_vec {
        return error(&format!("{:?}",err));
    }
    let vec = maybe_vec.unwrap();
    for element in vec {
        if element.header_address() == &hh {
            return Ok(element.clone());
        }
    }
    return error("Element not found at given HeaderHash");
}

/// Get Element at address using query()
pub fn get_local_from_eh(eh: EntryHash) -> ExternResult<Element> {
    let inmail_query_args = ChainQueryFilter::default()
       .include_entries(true);
    let maybe_vec = query(inmail_query_args);
    if let Err(err) = maybe_vec {
        return error(&format!("{:?}",err));
    }
    let vec = maybe_vec.unwrap();
    for element in vec {
        if element.header().entry_hash() == Some(&eh) {
            return Ok(element.clone());
        }
    }
    return error("Element not found at given EntryHash");
}


/// Get EntryHash for Element
pub fn get_eh(element: &Element) -> ExternResult<EntryHash> {
    let maybe_eh = element.header().entry_hash();
    if let None = maybe_eh {
        warn!("get_eh(): entry_hash not found");
        return error("get_eh(): entry_hash not found");
    }
    Ok(maybe_eh.unwrap().clone())
}

/// Call get() to obtain EntryHash from a HeaderHash
pub fn hh_to_eh(hh: HeaderHash) -> ExternResult<EntryHash> {
    trace!("hh_to_eh(): START - get...");
    let maybe_element = get(hh, GetOptions::content())?;
    trace!("hh_to_eh(): START - get DONE");
    if let None = maybe_element {
        warn!("hh_to_eh(): Element not found");
        return error("hh_to_eh(): Element not found");
    }
    return get_eh(&maybe_element.unwrap());
}


/// Call get() to obtain EntryHash and AppEntry from a HeaderHash
pub fn get_typed_from_hh<T: TryFrom<SerializedBytes>>(hash: HeaderHash)
    -> ExternResult<(EntryHash, T)>
{
    match get(hash.clone(), GetOptions::content())? {
        Some(element) => {
            let eh = element.header().entry_hash().expect("Converting HeaderHash which does not have an Entry");
            Ok((eh.clone(), get_typed_from_el(element)?))
        },
        None => crate::error("Entry not found"),
    }
}


/// Call get() to obtain EntryHash and AppEntry from an EntryHash
pub fn get_latest_typed_from_eh<T: TryFrom<SerializedBytes>>(entry_hash: EntryHash)
    -> ExternResult<(EntryHash, T)>
{
    match get(entry_hash.clone(), GetOptions::latest())? {
        Some(element) => Ok((entry_hash, get_typed_from_el(element)?)),
        None => crate::error("Entry not found"),
    }
}

/// Obtain AppEntry from Element
pub fn get_typed_from_el<T: TryFrom<SerializedBytes>>(element: Element) -> ExternResult<T> {
    match element.entry() {
        element::ElementEntry::Present(entry) => get_typed_from_entry::<T>(entry.clone()),
        _ => crate::error("Could not convert element"),
    }
}

/// Obtain AppEntry from Entry
pub fn get_typed_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => crate::error("Could not convert entry"),
        },
        _ => crate::error("Could not convert entry"),
    }
}

/// Obtain latest AppEntry at EntryHash and get its author
/// Conditions: Must be a single author entry type
pub(crate) fn get_typed_and_author<T: TryFrom<SerializedBytes>>(eh: &EntryHash)
    -> ExternResult<(AgentPubKey, T)>
{
    let maybe_maybe_element = get(eh.clone(), GetOptions::latest());
    if let Err(err) = maybe_maybe_element {
        warn!("Failed getting element: {}", err);
        return Err(err);
    }
    let maybe_element = maybe_maybe_element.unwrap();
    if maybe_element.is_none() {
        return error("no element found at address");
    }
    let element = maybe_element.unwrap();
    //assert!(entry_item.headers.len() > 0);
    //assert!(entry_item.headers[0].provenances().len() > 0);
    let author = element.header().author();
    let app_entry = get_typed_from_el::<T>(element.clone())?;
    Ok((author.clone(), app_entry))
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

pub fn get_latest_entry_from_eh<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_hash: EntryHash,
) -> ExternResult<OptionTypedEntryAndHash<T>> {
    // First, make sure we DO have the latest header_hash address
    let maybe_latest_header_hash = match get_details(entry_hash, GetOptions::latest())? {
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
        Some(latest_header_hash) => match get(latest_header_hash, GetOptions::latest())? {
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


///
pub fn get_latest_element_from_eh<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_hash: EntryHash,
) -> ExternResult<Option<Element>> {
    let maybe_entry_and_hash = get_latest_entry_from_eh::<T>(entry_hash.clone())?;
    let entry_and_hash = match maybe_entry_and_hash {
        Some(e) => e,
        None => return Ok(None),
    };
    debug!("get_latest entry_and_hash:\n - hh: {:?}\n - eh: {:?}", entry_and_hash.1, entry_and_hash.2);
    let maybe_maybe_element = get(entry_and_hash.2, GetOptions::latest());
    let element = match maybe_maybe_element {
        Ok(Some(e)) => e,
        _ => return Ok(None),
    };
    debug!("get_latest: element({}): {:?}", element.header().header_seq(), element.header().entry_hash());
    Ok(Some(element))
}


///
pub fn get_typed_from_eh<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_hash: EntryHash,
    get_options: GetOptions,
) -> ExternResult<OptionTypedEntryAndHash<T>> {
    /// First, make sure we DO have the latest header_hash address
    let maybe_latest_header_hash = match get_details(entry_hash.clone(), get_options.clone())? {
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
    let latest_header_hash = match maybe_latest_header_hash {
        None => return Ok(None),
        Some(hh) => hh,
    };
    /// Second, go and get that element, and return its entry and header_address
    let maybe_latest_element = get(latest_header_hash, get_options)?;
    let element = match maybe_latest_element {
        None => return Ok(None),
        Some(el) => el,
    };
    let maybe_typed_entry = element.entry().to_app_option::<T>()?;
    let entry = match maybe_typed_entry {
        None => return Ok(None),
        Some(e) => e,
    };
    let hh = match element.header() {
        /// we DO want to return the header for the original instead of the updated
        Header::Update(update) => update.original_header_address.clone(),
        Header::Create(_) => element.header_address().clone(),
        _ => unreachable!("Can't have returned a header for a nonexistent entry"),
    };
    let eh =  element.header().entry_hash().unwrap().to_owned();
    /// Done
    Ok(Some((entry, hh, eh)))
}
