use hdk3::prelude::*;

use crate::{
    mail::functions::get_mail::*,
};

#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct DeleteMailOutput(pub Option<HeaderHash>);

#[hdk_extern]
pub fn delete_mail(hh: HeaderHash) -> ExternResult<DeleteMailOutput> {
    /// Make sure HeaderHash points to a Mail
    let maybe_mail = try_into_mail(hh.clone())?;
    debug!("delete_mail(): maybe_mail = {:?}", maybe_mail);
    if maybe_mail.is_none() {
        return Ok(DeleteMailOutput(None));
    }
    ///
    let deletion_hh = delete_entry(hh)?;
    Ok(DeleteMailOutput(Some(deletion_hh)))
}
