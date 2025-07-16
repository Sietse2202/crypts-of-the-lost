# Event

The `Event` struct represents a message sent **from the server to the client**.
It is used by the server to send game or system events to the client, ranging
from `ConnectionAccepted` and `PositionUpdate` to `ChatMessage`.

```rust
pub struct Event {}
```

| Field | Type | Description |
