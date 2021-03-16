use hdk::prelude::*;

#[hdk_extern]
fn validation_package(_input: AppEntryType) -> ExternResult<ValidationPackageCallbackResult> {
   trace!("*** validation_package() callback called!");
   let dummy = ValidationPackage(vec![]);
   Ok(ValidationPackageCallbackResult::Success(dummy))
}
