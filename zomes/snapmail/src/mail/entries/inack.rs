use hdk::prelude::*;

/// Entry for a received Acknowledgement Receipt
#[hdk_entry(id = "inack")]
#[derive(Clone, PartialEq)]
pub struct InAck {
    // n/a
}

impl InAck {
    pub fn new() -> Self {
        Self {
        }
    }
}
