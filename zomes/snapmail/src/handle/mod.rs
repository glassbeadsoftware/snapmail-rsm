mod functions;
mod utils;
mod validation;

use hdk3::prelude::*;

pub use functions::*;
pub use validation::*;

/// Entry representing the username of an Agent
#[hdk_entry(id = "Handle", visibility = "public")]
#[derive(Debug, Clone, PartialEq)]
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
