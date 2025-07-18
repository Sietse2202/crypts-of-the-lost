// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! This crate provides an abstraction over whom to send a message to. This is in the form of a
//! trait

use std::collections::HashSet;
use std::net::SocketAddr;

/// The target to send the Event to
#[derive(Debug, Clone)]
pub enum Target {
    /// Sends the event to everyone
    All,
    /// Sends the event to only one connection
    One(SocketAddr),
    /// Sends the event to a group of connections
    Group(HashSet<SocketAddr>),
    /// Sends the event to all but one
    AllButOne(SocketAddr),
    /// Sends the event to all but a group of connections
    AllBut(HashSet<SocketAddr>),
}

impl Target {
    /// If the provided address is specified as a target
    #[must_use]
    pub fn is_recipient(&self, address: &SocketAddr) -> bool {
        match self {
            Self::All => true,
            Self::One(addr) => addr == address,
            Self::Group(group) => group.contains(address),
            Self::AllBut(group) => !group.contains(address),
            Self::AllButOne(addr) => addr != address,
        }
    }
}
