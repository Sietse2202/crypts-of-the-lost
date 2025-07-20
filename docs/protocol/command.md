# Command

The `Command` struct represents a message sent **from the client to the server**.
It is used by the client to send specific commands or requests to the server,
starting with `Connect` and continuing with actions such as `OpenInventory`

```rust
pub enum Command {
    Join(join::Join),
}
```

| Variant | Description                  | Data                            |
| ------- | ---------------------------- | ------------------------------- |
| `Join`  | A request to join the server | Holds [Join](./command/join.md) |
