use hdk::prelude::*;

/// Entry for a received Acknowledgement Receipt
#[hdk_entry(id = "inack", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct InAck {
    /// Signed outmail_eh
    pub from_signature: Signature,
}

impl InAck {
    pub fn new(from_signature: Signature) -> Self {
        Self {
            from_signature,
        }
    }
}
