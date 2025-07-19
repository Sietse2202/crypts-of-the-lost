# Event

The `Event` struct represents a message sent **from the server to the client**.
It is used by the server to send game or system events to the client, ranging
from `ConnectionAccepted` and `PositionUpdate` to `ChatMessage`.

```rust
pub enum Event {
    JoinAccept(join_accept::JoinAccept),
    PlayerJoined(player_joined::PlayerJoined),
}
```

| Variant        | Description                       | Data                                           |
| -------------- | --------------------------------- | ---------------------------------------------- |
| `JoinAccept`   | Gets send when a new player joins | Holds [JoinAccept](./event/join_accept.md)     |
| `PlayerJoined` | A new player joined               | Holds [PlayerJoined](./event/player_joined.md) |
