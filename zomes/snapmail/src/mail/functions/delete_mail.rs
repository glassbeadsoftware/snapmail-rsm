use hdk::prelude::*;


/// TODO: check if address is of correct entry type?
pub fn delete_mail(address: HeaderHash) -> ExternResult<HeaderHash> {
    hdk::remove_entry(&address)
}
