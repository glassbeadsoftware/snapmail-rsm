# Design
#### Synchronous send

```mermaid
sequenceDiagram
    participant Sender
    participant Receiver
    Sender->>Sender: Write OutMessage
    Sender->>Receiver: Send OutMessage        
    Receiver->>Receiver: Write InMessage
    Note right of Receiver: Opens message
    Receiver->>Receiver: Write AckReceipt (private)
    Receiver->>Sender: Send AckReceipt
    Sender->>Sender: Add 'receipts' link to OutMessage
```

#### Asynchronous send
```mermaid
sequenceDiagram
    participant Sender
    participant DHT
    participant Receiver
    Sender->>Sender: Write OutMessage
    loop receipient
        Sender->>Sender: Write PendingMessage
        Sender->>DHT: Share PendingMessage
    end
        
    Note right of Receiver: Checks mail
    Receiver-->>+DHT: Looks up 'message_inbox' links at agentId address
    DHT-->>-Receiver:  [links]
    loop PendingMessage link
        Receiver->>+DHT: Get PendingMessage
        DHT->>-Receiver: [PendingMessage]
        Receiver->>Receiver: Write InMessage
        Receiver->>DHT: Delete PendingMessage
    end
    Note right of Receiver: Opens message
    Receiver->>Receiver: Write AckReceipt
    Receiver->>DHT: Share AckReceipt
    Note left of Sender: Checks mail
    Sender-->>+DHT: Looks up 'ack_inbox' links at agentId address
    DHT-->>-Sender:  [links]
    loop receipt link
        Sender->>+DHT: Get AckReceipt
        DHT->>-Sender: [AckReceipt]    
        Sender->>Sender: Add 'receipts' link to OutMessage
        Sender->>DHT: Delete AckReceipt
    end
```

#### Check if Message has been read

```
bool has_been_read = 'pendings' link count == 'receipts' link count
```

When adding receipt link, must validate that there is a pending link or that receipt is authored by one of the receipients and that there are no other valid receipt link from that author.