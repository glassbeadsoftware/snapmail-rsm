use hdk::prelude::*;
#[allow(unused_imports)]
use snapmail_model::*;

use crate::{
    mail::functions::get_mail::*,
};


#[hdk_extern]
//#[snapmail_api]
pub fn delete_mail(ah: ActionHash) -> ExternResult<Option<ActionHash>> {
    /// Make sure ActionHash points to a Mail
    let maybe_mail = try_into_mail(ah.clone())?;
    trace!("delete_mail(): maybe_mail = {:?}", maybe_mail);
    if maybe_mail.is_none() {
        return Ok(None);
    }
    ///
    let deletion_ah = delete_entry(ah)?;
    Ok(Some(deletion_ah))
}
