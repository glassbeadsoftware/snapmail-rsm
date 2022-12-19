use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::{
    mail::functions::get_mail::*,
};


#[derive(Shrinkwrap, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteMailOutput(pub Option<ActionHash>);

#[hdk_extern]
//#[snapmail_api]
pub fn delete_mail(ah: ActionHash) -> ExternResult<DeleteMailOutput> {
    /// Make sure ActionHash points to a Mail
    let maybe_mail = try_into_mail(ah.clone())?;
    trace!("delete_mail(): maybe_mail = {:?}", maybe_mail);
    if maybe_mail.is_none() {
        return Ok(DeleteMailOutput(None));
    }
    ///
    let deletion_ah = delete_entry(ah)?;
    Ok(DeleteMailOutput(Some(deletion_ah)))
}
