use hdi::prelude::*;

use ts_rs::TS;


/// Entry representing the username of an Agent
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
#[derive(TS)]
#[ts(export)]
pub struct Handle {
    pub name: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: String::new(),
        }
    }

    /// DEBUG
    pub fn dummy() -> Self {
        Self {
            name: "dummy".to_string(),
        }
    }
}
