// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Network Handler
//! Handles the actual network communication, message serialization/deserialization,
//! and connection management for the game server

mod client;
mod deserialize;
mod handle_connection;
mod inbound;
mod outbound;
mod serialize;
mod shutdown;
mod start;

use crate::envelope::{InboundMessage, OutboundMessage};
use dashmap::DashMap;
use quinn::{Connection, Endpoint, ServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{
    broadcast::{self, Sender},
    mpsc::{UnboundedReceiver, UnboundedSender},
};

/// The network handler manages actual network connections and message processing
#[derive(Debug)]
pub struct NetworkHandler {
    /// The QUIC endpoint for handling connections
    endpoint: Option<Endpoint>,
    /// Active connections mapped by client Address
    connections: Arc<DashMap<SocketAddr, Connection>>,
    /// Channel for sending inbound message to the dispatcher
    inbound_tx: UnboundedSender<InboundMessage>,
    /// Fan out of the `outbound_rx`
    broadcast: Sender<OutboundMessage>,
    /// Server configuration for QUIC
    server_config: ServerConfig,
    /// Socket address to bind to
    socket: SocketAddr,
}

impl NetworkHandler {
    /// Creates a new network handler instance
    ///
    /// Sets up the handler with the necessary channels, configuration, and certificates
    /// for managing network connections and message processing.
    #[must_use]
    #[inline]
    pub fn new(
        socket: SocketAddr,
        server_config: ServerConfig,
        outbound_rx: UnboundedReceiver<OutboundMessage>,
        inbound_tx: UnboundedSender<InboundMessage>,
    ) -> Self {
        let broadcast = Self::start_fan_out(outbound_rx);
        Self {
            endpoint: None,
            connections: Arc::new(DashMap::new()),
            inbound_tx,
            broadcast,
            server_config,
            socket,
        }
    }

    fn start_fan_out(
        mut outbound_rx: UnboundedReceiver<OutboundMessage>,
    ) -> Sender<OutboundMessage> {
        let (broadcast_tx, _) = broadcast::channel::<OutboundMessage>(1024);

        let broadcast_tx_clone = broadcast_tx.clone();
        tokio::spawn(async move {
            while let Some(msg) = outbound_rx.recv().await {
                let _ = broadcast_tx_clone.send(msg);
            }
        });

        broadcast_tx
    }
}
