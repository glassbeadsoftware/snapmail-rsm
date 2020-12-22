use hdk3::prelude::*;

#[hdk_extern]
fn validation_package(_input: AppEntryType) -> ExternResult<ValidationPackageCallbackResult> {
   debug!("*** validation_package() callback called!");
   let dummy = ValidationPackage(vec![]);
   Ok(ValidationPackageCallbackResult::Success(dummy))
}
