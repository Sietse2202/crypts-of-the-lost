// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Network Handler
//! Handles the actual network communication, message serialization/deserialization,
//! and connection management for the game server

mod client;

use crate::{
    cert::Certs,
    envelope::{InboundMessage, OutboundMessage},
};
use quinn::{Connection, Endpoint, ServerConfig};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

/// The network handler manages actual network connections and message processing
#[derive(Debug)]
#[expect(dead_code)]
pub struct NetworkHandler {
    /// The QUIC endpoint for handling connections
    enpoint: Option<Endpoint>,
    /// Active connections mapped by client Address
    connections: Arc<RwLock<HashMap<SocketAddr, Connection>>>,
    /// Channel for receiving outbound messages from the dispatcher
    outbound_rx: mpsc::UnboundedReceiver<OutboundMessage>,
    /// Channel for sending inbound message to the dispatcher
    inbound_tx: mpsc::UnboundedSender<InboundMessage>,
    /// Server configuration for QUIC
    server_config: ServerConfig,
    /// TLS certificates and key
    certs: Certs,
    /// Socket address to bind to
    socket: SocketAddr,
}

impl NetworkHandler {
    /// Creates a new network handler instance
    ///
    /// Sets up the handler with the necessary channels, configuration, and certificates
    /// for managing network connections and message processing.
    #[must_use]
    pub fn new(
        socket: SocketAddr,
        server_config: ServerConfig,
        certs: Certs,
        outbound_rx: mpsc::UnboundedReceiver<OutboundMessage>,
        inbound_tx: mpsc::UnboundedSender<InboundMessage>,
    ) -> Self {
        Self {
            enpoint: None,
            connections: Arc::new(RwLock::new(HashMap::new())),
            outbound_rx,
            inbound_tx,
            server_config,
            certs,
            socket,
        }
    }
}
