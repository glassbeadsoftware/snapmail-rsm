use hdi::prelude::*;

/// Entry representing the username of an Agent
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Handle {
    pub username: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            username: name,
        }
    }

    pub fn empty() -> Self {
        Self {
            username: String::new(),
        }
    }

    /// DEBUG
    pub fn dummy() -> Self {
        Self {
            username: "dummy".to_string(),
        }
    }
}
