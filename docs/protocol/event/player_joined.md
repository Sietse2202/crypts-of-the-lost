# PlayerJoined

Send by the server to everyone when a new player joins.

```rust
pub struct PlayerJoined {
    inner: EventInner,
}
```

| Field   | Type                       | Description           |
| ------- | -------------------------- | --------------------- |
| `inner` | [`EventInner`](./inner.md) | Extra associated data |
