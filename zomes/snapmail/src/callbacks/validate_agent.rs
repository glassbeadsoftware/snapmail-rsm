use hdk::prelude::*;

#[hdk_extern]
fn validate_agent(_: Element) -> ExternResult<ValidateCallbackResult> {
   debug!("*** validate_agent() callback called!");
   Ok(ValidateCallbackResult::Valid)
}
