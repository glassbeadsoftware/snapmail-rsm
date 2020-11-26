use hdk3::prelude::*;

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[hdk_entry(id = "pending_ack")]
#[derive(Clone, Debug, PartialEq)]
pub struct PendingAck {
    pub outmail_eh: EntryHash,
}

impl PendingAck {
    pub fn new(outmail_eh: EntryHash) -> Self {
        Self {
            outmail_eh,
        }
    }
}
