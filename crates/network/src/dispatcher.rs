// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! Defines the [`NetworkDispatcher`] struct that coordinates the communication between
//! the game logic and network logic.

use crate::envelope::{InboundMessage, OutboundMessage};
use crate::target::NetworkTarget;
use protocol::command::Command;
use protocol::event::Event;
use std::collections::{HashSet, VecDeque};
use std::net::SocketAddr;

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

    pub(crate) fn _add_client(&mut self, client: SocketAddr) {
        self.clients.insert(client);
    }

    /// Pushes a new event to the queue
    #[inline]
    pub fn push(&mut self, event: Event, target: Box<dyn NetworkTarget + Send + Sync>) {
        let msg = OutboundMessage::new(target, event);
        self.outbound.push_back(msg);
    }

    /// Get the first response, and remove it from the queue
    #[inline]
    pub fn pop(&mut self) -> Option<Command> {
        let msg = self.inbound.pop_front()?;
        Some(msg.command)
    }
}
