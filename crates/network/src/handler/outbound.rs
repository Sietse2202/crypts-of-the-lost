// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

#![expect(unused_variables)]
#![expect(clippy::unused_async)]

use super::NetworkHandler;
use crate::envelope::OutboundMessage;
use tokio::sync::broadcast::Receiver;

impl NetworkHandler {
    pub(super) async fn process_outbound(
        dispatcher_rx: Receiver<OutboundMessage>,
        conn_tx: quinn::SendStream,
    ) {
    }
}
