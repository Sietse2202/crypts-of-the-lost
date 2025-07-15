// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! This module defines types that are used to communicate between client and server.

#![allow(dead_code)]

use crate::cert::Certs;
use bevy_ecs::prelude::Resource;
use quinn::{Connection, Endpoint, ServerConfig};
use rustls_pki_types::pem;
use std::collections::VecDeque;
use std::fmt::Formatter;
use std::net::SocketAddr;
use thiserror::Error;

/// Errors the dispatcher can encounter
#[derive(Error, Debug)]
pub enum DispatcherError {
    /// Io error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Quinn connection error
    #[error("connection error: {0}")]
    Connection(#[from] quinn::ConnectionError),
    /// Pem error from trying to get the certificates and key
    #[error("pem error: {0}")]
    Pem(#[from] pem::Error),
}

pub(crate) type Result<T> = std::result::Result<T, DispatcherError>;

/// Simple type alias for binary data
pub type Data = Box<[u8]>;

/// A network event from the server point of view
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub enum NetworkEvent {
    /// Event that a new client has connected
    NewConnection(SocketAddr),
    /// Event that a client has disconnected
    ConnectionClosed(SocketAddr),
    /// Response from a client
    Message(Response),
}

/// Response from a client
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Response {
    client: SocketAddr,
    data: Data,
}

/// Abstraction over a network target, this trait is used to select the clients a message should be
/// sent to
pub trait NetworkTarget {
    /// This method should return the clients to send a message to.
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr>;
}

/// Target to send a message to a single client
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Single(SocketAddr);

impl Single {
    /// Create a new self from the address
    #[inline]
    #[must_use]
    pub const fn new(client: SocketAddr) -> Self {
        Self(client)
    }
}

impl NetworkTarget for Single {
    fn receivers(&self, _: Vec<SocketAddr>) -> Vec<SocketAddr> {
        vec![self.0]
    }
}

/// Network target, to send a message to a group of people
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct Group(Vec<SocketAddr>);

impl Group {
    /// Create a new group from an iterator
    #[inline]
    #[must_use]
    pub fn new<I, Item>(ground: I) -> Self
    where
        I: IntoIterator<Item = Item>,
        Item: Into<SocketAddr>,
    {
        Self(ground.into_iter().map(Into::into).collect())
    }
}

impl NetworkTarget for Group {
    fn receivers(&self, _: Vec<SocketAddr>) -> Vec<SocketAddr> {
        self.0.clone()
    }
}

/// Target to send a message to everyone
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    Ord,
    PartialOrd,
    Default,
)]
pub struct All;

impl NetworkTarget for All {
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr> {
        clients
    }
}

/// Sends a message to everyone but one client
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Ord, PartialOrd,
)]
pub struct AllBut(pub(crate) SocketAddr);

impl AllBut {
    /// Creates a new [`AllBut`] instance from an address
    #[inline]
    #[must_use]
    pub const fn new(addr: SocketAddr) -> Self {
        Self(addr)
    }
}

impl NetworkTarget for AllBut {
    fn receivers(&self, clients: Vec<SocketAddr>) -> Vec<SocketAddr> {
        clients.into_iter().filter(|c| c != &self.0).collect()
    }
}

/// This struct contains a
pub struct MessageData {
    target: Box<dyn NetworkTarget + Send + Sync>,
    data: Data,
}

impl std::fmt::Debug for MessageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageData")
            .field("target", &"Box<dyn NetworkTarget>")
            .field("data", &self.data)
            .finish()
    }
}

/// The network-dispatcher, this handles all connections between server and clients.
#[derive(Debug, Resource)]
pub struct NetworkDispatcher {
    socket: SocketAddr,
    clients: Vec<SocketAddr>,
    pub(crate) message_queue: VecDeque<MessageData>,
    pub(crate) event_queue: VecDeque<NetworkEvent>,
}

impl NetworkDispatcher {
    /// Create a new dispatcher on the given `addr`
    #[inline]
    #[must_use]
    pub const fn new(addr: SocketAddr) -> Self {
        Self {
            message_queue: VecDeque::new(),
            event_queue: VecDeque::new(),
            clients: Vec::new(),
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
    pub fn clients(&self) -> &[SocketAddr] {
        &self.clients
    }

    #[inline]
    pub(crate) fn queue(&mut self, message: MessageData) {
        self.message_queue.push_back(message);
    }

    /// Listens for connections on `self.socket`, uses `handle_conn` to handle each connection
    /// encountered by the server.
    ///
    /// # Errors
    /// The function may error due to one of
    /// - IO errors
    /// - Connection errors
    pub async fn listen<F>(self, config: ServerConfig, handle_conn: F, certs: Certs) -> Result<()>
    where
        F: AsyncFn(&Self, Connection) -> Result<()> + Send + Sync + 'static,
    {
        let endpoint = Endpoint::server(config, self.socket)?;
        let (_certs, _key) = (certs.certs(), certs.key());

        while let Some(conn) = endpoint.accept().await {
            let connection = conn.await?;

            handle_conn(&self, connection).await?;
        }

        Ok(())
    }
}
