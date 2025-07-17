# Command

The `Command` struct represents a message sent **from the client to the server**.
It is used by the client to send specific commands or requests to the server,
starting with `Connect` and continuing with actions such as `OpenInventory`

```rust
pub struct Command {
    inner: CommandInner,
}
```

| Field   | Type                                 | Description                          |
| ------- | ------------------------------------ | ------------------------------------ |
| `inner` | [`CommandInner`](./command/inner.md) | The command type and its information |
