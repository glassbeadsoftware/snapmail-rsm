use hdk::prelude::*;

/// Entry for a received Acknowledgement Receipt
#[hdk_entry(id = "delivery_confirmation", visibility = "private")]
#[derive(Clone, PartialEq)]
pub struct DeliveryConfirmation {
    /// EntryHash to OutMail or OutAck on same chain
    pub package_eh: EntryHash,
    pub recipient: AgentPubKey,
}

impl DeliveryConfirmation {
    pub fn new(package_eh: EntryHash, recipient: AgentPubKey) -> Self {
        Self {
            package_eh,
            recipient,
        }
    }
}
