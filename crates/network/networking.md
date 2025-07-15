# Game Server Networking Design (Rust + Tokio + ECS)

ChatGPT generated this file, because I'm bad at writing. I also told him to give some
code examples of how it would look like, but we would obviously need to adapt
it to our game.

## Overview

- ✅ Support many concurrent connections (`tokio::spawn`)
- ✅ Bidirectional communication
- ✅ ECS systems need to send & receive messages
- ✅ Most messages sent to all clients, some to specific ones

---

## Dispatcher Design

### Event Enum

```rust
#[derive(Debug)]
pub enum NetworkEvent {
    Connected(SocketAddr),
    Disconnected(SocketAddr),
    Message(SocketAddr, String),
}
```

### Target Enum

```rust
#[derive(Debug, Clone)]
pub enum NetworkTarget {
    One(SocketAddr),
    All,
    AllExcept(SocketAddr),
    Group(Vec<SocketAddr>),
}
```

### Dispatcher Struct

```rust
use std::collections::{HashMap, VecDeque};

pub struct NetworkDispatcher {
    pub events: VecDeque<NetworkEvent>,
    pub outgoing: VecDeque<(NetworkTarget, String)>,
}
```

### Shared Dispatcher

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedDispatcher = Arc<Mutex<NetworkDispatcher>>;
```

---

## `network` Crate: Handling a Client

```rust
pub async fn handle_client(stream: TcpStream, dispatcher: SharedDispatcher) {
    let addr = stream.peer_addr().unwrap();
    {
        let mut d = dispatcher.lock().await;
        d.events.push_back(NetworkEvent::Connected(addr));
    }

    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader).lines();

    let read_task = tokio::spawn({
        let dispatcher = dispatcher.clone();
        async move {
            while let Ok(Some(line)) = reader.next_line().await {
                let mut d = dispatcher.lock().await;
                d.events.push_back(NetworkEvent::Message(addr, line));
            }
            let mut d = dispatcher.lock().await;
            d.events.push_back(NetworkEvent::Disconnected(addr));
        }
    });

    let write_task = tokio::spawn({
        let dispatcher = dispatcher.clone();
        async move {
            loop {
                let mut d = dispatcher.lock().await;
                let mut send_buf = Vec::new();
                let mut i = 0;

                while i < d.outgoing.len() {
                    let (target, msg) = &d.outgoing[i];
                    let should_send = match target {
                        NetworkTarget::One(a) => *a == addr,
                        NetworkTarget::All => true,
                        NetworkTarget::AllExcept(a) => *a != addr,
                        NetworkTarget::Group(addrs) => addrs.contains(&addr),
                    };

                    if should_send {
                        send_buf.push(msg.clone());
                    }

                    i += 1;
                }

                drop(d);

                for msg in send_buf {
                    writer.write_all(msg.as_bytes()).await.unwrap();
                    writer.write_all(b"\n").await.unwrap();
                }

                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        }
    });

    let _ = tokio::join!(read_task, write_task);
}
```

---

## Server Main Loop

```rust
#[tokio::main]
async fn main() {
    let dispatcher = Arc::new(Mutex::new(NetworkDispatcher {
        events: VecDeque::new(),
        outgoing: VecDeque::new(),
    }));

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let dispatcher = dispatcher.clone();
        tokio::spawn(async move {
            handle_client(stream, dispatcher).await;
        });
    }
}
```

---

## ECS System Example

```rust
pub fn network_system(dispatcher: SharedDispatcher) {
    let mut dispatcher = dispatcher.blocking_lock();

    while let Some(event) = dispatcher.events.pop_front() {
        match event {
            NetworkEvent::Message(addr, msg) => {
                println!("{} says: {}", addr, msg);

                dispatcher.outgoing.push_back((
                    NetworkTarget::One(addr),
                    "pong".into(),
                ));
            }
            NetworkEvent::Connected(addr) => {
                println!("{} connected", addr);
            }
            NetworkEvent::Disconnected(addr) => {
                println!("{} disconnected", addr);
            }
        }
    }
}
```

---

## Sending Messages (Selective & Broadcast)

### Send to all clients

```rust
dispatcher.outgoing.push_back((
    NetworkTarget::All,
    "global_tick".into(),
));
```

### Send to one client

```rust
dispatcher.outgoing.push_back((
    NetworkTarget::One(player_addr),
    "you_got_hit".into(),
));
```

### Send to all except one

```rust
dispatcher.outgoing.push_back((
    NetworkTarget::AllExcept(shooter_addr),
    "player_shot".into(),
));
```

### Send to a group

```rust
dispatcher.outgoing.push_back((
    NetworkTarget::Group(vec![a1, a2, a3]),
    "team_message".into(),
));
```

---

## Summary

| Target Type        | Use case                     |
| ------------------ | ---------------------------- |
| `One(addr)`        | Private message              |
| `All`              | Broadcast                    |
| `AllExcept(addr)`  | Broadcast with exclusion     |
| `Group(Vec<addr>)` | Team or zone-based messaging |
