use hdk3::prelude::*;

/// Entry representing an AcknowldegmentReceipt on the DHT waiting to be received
#[hdk_entry(id = "pending_ack")]
#[derive(Clone, Debug, PartialEq)]
pub struct PendingAck {
    pub outmail_address: HeaderHash,
}

impl PendingAck {
    pub fn new(outmail_address: HeaderHash) -> Self {
        Self {
            outmail_address,
        }
    }
}
