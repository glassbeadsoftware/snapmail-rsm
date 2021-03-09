use hdk::prelude::*;

/// Entry for an Acknowledgement Receipt of a Mail authored by this agent
#[hdk_entry(id = "outack")]
#[derive(Clone, Debug, PartialEq)]
pub struct OutAck {
    // n/a
    //pub name: String,
}

impl OutAck {
    pub fn new() -> Self {
        Self {
            //name: "empty".to_string(),
        }
    }
}
