// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use tracing::error;

use crate::envelope::InboundMessage;

use super::NetworkHandler;

impl NetworkHandler {
    pub(super) async fn process_inbound(
        dispatcher_tx: flume::Sender<InboundMessage>,
        mut conn_rx: quinn::RecvStream,
    ) {
        while let Ok(data) = conn_rx.read_to_end(1024 * 1024).await {
            let Ok(command) = Self::deserialize_command(&data) else {
                error!("error deserializing command with data: {data:?}");
                continue;
            };
            let msg = InboundMessage::new(command);
            let Ok(()) = dispatcher_tx.send_async(msg).await else {
                error!("coudln't send `InboundMessage` to the dispatcher");
                continue;
            };
        }
    }
}
