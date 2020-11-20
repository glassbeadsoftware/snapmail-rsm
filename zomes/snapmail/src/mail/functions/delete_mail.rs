use hdk3::prelude::*;
use crate::{
    utils::*,
    mail::functions::get_mail::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct DeleteMailOutput(pub Option<HeaderHash>);


#[hdk_extern]
pub fn delete_mail(address: HeaderHash) -> ExternResult<DeleteMailOutput> {
    /// Make sure address is a valid mail
    let maybe_mail = try_into_mail(address.clone())?;
    debug!("delete_mail(): maybe_mail = {:?}", maybe_mail).ok();
    if maybe_mail.is_none() {
        return Ok(DeleteMailOutput(None));
    }
    ///
    let hh = delete_entry(address)?;
    Ok(DeleteMailOutput(Some(hh)))
}
