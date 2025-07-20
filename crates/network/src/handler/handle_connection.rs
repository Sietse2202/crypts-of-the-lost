// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use crate::envelope::{InboundMessage, OutboundMessage};
use dashmap::DashMap;
use quinn::Connection;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast::Receiver, mpsc::UnboundedSender};
use tracing::{error, info};

impl NetworkHandler {
    pub(super) async fn handle_connection(
        connection: Connection,
        connections: Arc<DashMap<SocketAddr, Connection>>,
        handler_tx: UnboundedSender<InboundMessage>,
        handler_rx: Receiver<OutboundMessage>,
    ) {
        let addr = connection.remote_address();
        let Ok((tx, rx)) = connection.open_bi().await else {
            error!("error opening bidirectional stream for client {addr}");
            return;
        };

        let inbound =
            tokio::spawn(async move { Self::process_inbound(handler_tx, rx, addr).await });

        let outbound =
            tokio::spawn(async move { Self::process_outbound(handler_rx, tx, addr).await });

        let result = tokio::select! {
            _ = inbound => "inbound",
            _ = outbound => "outbound",
        };

        info!("cleaning up connections for {addr} (reason: {result} ended");
        Self::remove_client(&connections, addr, 0, b"Connection handler ended");
    }
}
