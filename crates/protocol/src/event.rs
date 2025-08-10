// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Event
//! This module contains all types used for the communication from the server
//! to the client.

mod inner;
pub mod join_accept;
pub mod player_joined;

use crate::target::Target;
pub use inner::EventInner;

/// Type used inside the server to specify the targets
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Event {
    /// Targets the event is sent to
    pub target: Target,
    /// The actual event
    pub event: EventKind,
}

impl Event {
    /// Creates a new `Event` struct
    #[must_use]
    pub const fn new(event: EventKind, target: Target) -> Self {
        Self { target, event }
    }
}

/// Message from the server, to the client
#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone)]
#[non_exhaustive]
pub enum EventKind {
    /// Gets send when a new player joins
    JoinAccept(join_accept::JoinAccept),
    /// A new player joined
    PlayerJoined(player_joined::PlayerJoined),
}
