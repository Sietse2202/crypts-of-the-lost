// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use protocol::event::Event;
use tokio::sync::broadcast::Receiver;
use tracing::{error, warn};

impl NetworkHandler {
    pub(super) async fn process_outbound(
        mut dispatcher_rx: Receiver<Event>,
        mut conn_tx: quinn::SendStream,
    ) {
        let id = conn_tx.id();
        while let Ok(event) = dispatcher_rx.recv().await {
            /*if !target.is_recipient(&addr) {
                continue;
            }*/

            let Ok(data) = Self::serialize_event(event) else {
                warn!("wasn't able to serialize event");
                continue;
            };

            #[expect(clippy::cast_possible_truncation)]
            let len = u32::to_be_bytes(data.len() as u32); // event data will never be 4 GiB big
            let mut buf = Vec::with_capacity(len.len() + data.len());
            buf.extend_from_slice(&len);
            buf.extend_from_slice(&data);

            if let Err(e) = conn_tx.write_all(&buf).await {
                error!("[Stream {id}] error writing to the stream: {e}");
                return;
            }
        }
    }
}
