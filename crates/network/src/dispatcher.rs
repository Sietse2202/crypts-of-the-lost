// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! Defines the [`NetworkDispatcher`] struct that coordinates the communication between
//! the game logic and network logic.

use crate::cert::Certs;
use crate::error::Result;
use crate::protocol::{Command, InboundMessage, OutboundMessage};
use quinn::{Connection, Endpoint, ServerConfig};
use std::collections::{HashSet, VecDeque};
use std::net::SocketAddr;
use tracing::{error, info};

/// The network-dispatcher, this handles all connections between server and clients.
#[derive(Debug)]
pub struct NetworkDispatcher {
    socket: SocketAddr,
    clients: HashSet<SocketAddr>,
    pub(crate) outbound: VecDeque<OutboundMessage>,
    pub(crate) inbound: VecDeque<InboundMessage>,
}

impl NetworkDispatcher {
    /// Create a new dispatcher on the given `addr`
    #[inline]
    #[must_use]
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            outbound: VecDeque::new(),
            inbound: VecDeque::new(),
            clients: HashSet::new(),
            socket: addr,
        }
    }

    /// Get the socket
    #[inline]
    #[must_use]
    pub const fn socket(&self) -> SocketAddr {
        self.socket
    }

    /// Get the clients currently connected
    #[inline]
    #[must_use]
    pub const fn clients(&self) -> &HashSet<SocketAddr> {
        &self.clients
    }

    pub(crate) fn add_client(&mut self, client: SocketAddr) {
        self.clients.insert(client);
    }

    /// Pushes a new message to the queue
    #[inline]
    pub fn push(&mut self, message: OutboundMessage) {
        self.outbound.push_back(message);
    }

    /// Get the first response, and remove it from the queue
    #[inline]
    pub fn pop(&mut self) -> Option<InboundMessage> {
        self.inbound.pop_front()
    }

    /// Listens for connections on `self.socket`, uses `handle_conn` to handle each connection
    /// encountered by the server.
    ///
    /// # Errors
    /// The function may error due to one of
    /// - IO errors
    /// - Connection errors
    pub async fn listen<F>(
        mut self,
        config: ServerConfig,
        handle_conn: F,
        certs: Certs,
    ) -> Result<()>
    where
        F: AsyncFn(Connection) -> Result<()> + Copy + Send + Sync + 'static,
        for<'a> F::CallRefFuture<'a>: Send,
    {
        let endpoint = Endpoint::server(config, self.socket)?;
        let (_certs, _key) = (certs.certs(), certs.key());

        while let Some(conn) = endpoint.accept().await {
            let connection = conn.await?;

            let addr = connection.remote_address();
            self.add_client(addr);

            tokio::spawn(async move {
                if let Err(e) = handle_conn(connection).await {
                    error!("Error while handling connection {addr}: {e}");
                }
            });
        }

        Ok(())
    }
}

/// Default connection handler
///
/// # Errors
/// TODO: fix the errors section
pub async fn default_handler(conn: Connection) -> Result<()> {
    info!("New connection at {}", conn.remote_address());

    while let Ok((_send, mut recv)) = conn.accept_bi().await {
        let mut len_buf = [0u8; 4];
        recv.read(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf);

        let mut buf = vec![0u8; len as usize];
        recv.read(&mut buf).await?;

        let (_response, _byte_count): (Command, _) =
            bincode::serde::decode_from_slice(&buf, bincode::config::standard())?;
    }

    Ok(())
}
