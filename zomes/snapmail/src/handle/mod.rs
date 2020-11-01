mod functions;
mod utils;
mod validation;

use hdk3::prelude::*;

pub use functions::*;
pub use validation::*;

/// Entry representing the username of an Agent
#[hdk_entry(id = "Handle", visibility = "public")]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Handle {
    pub name: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}
