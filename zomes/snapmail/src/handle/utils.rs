use hdk3::prelude::*;
use hdk3::prelude::link::Link;

use crate::{
    ZomeString,
    utils::*,
    link_kind, path_kind,
    handle::Handle,
};

///
pub(crate) fn get_handle_string(maybe_handle_element: Option<Element>) -> ExternResult<ZomeString> {
    if let Some(current_handle_element) = maybe_handle_element {
        let current_handle: Handle = try_from_element(current_handle_element)
            .expect("Should be a Handle entry");
        return Ok(ZomeString(current_handle.name.into()));
    }
    return Ok(ZomeString("<noname>".into()));
}


/// Get 'Members' links on the DNA entry
pub(crate) fn get_members() -> ExternResult<Vec<Link>> {
    let path_hash = Path::from(path_kind::Directory).hash()?;
    let entry_results = get_links(path_hash, link_tag(link_kind::Members))?;
    Ok(entry_results.into_inner())
}
