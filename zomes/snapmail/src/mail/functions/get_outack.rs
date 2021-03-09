use hdk::prelude::*;

//
///// Get InMail our OutMail struct in local source chain at address
//pub fn get_outack(mail_address: &Address, from: &Address) -> Option<Result<InMail, OutMail>> {
//    let maybe_InMail = hdk::utils::get_as_type::<InMail>(address.clone());
//    if let Ok(inmail) = maybe_InMail {
//        return Some(Ok(inmail));
//    }
//    let maybe_OutMail = hdk::utils::get_as_type::<OutMail>(address);
//    if let Ok(outmail) = maybe_OutMail {
//        return Some(Err(outmail));
//    }
//    None
//}
