// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Event
//! This module contains all types used for the communication from the server
//! to the client.

mod inner;
pub mod join_accept;
pub mod player_joined;

use bincode::{Decode, Encode};
pub use inner::EventInner;

/// Message from the server, to the client
#[derive(Encode, Decode, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
#[non_exhaustive]
pub enum Event {
    /// Gets send when a new player joins
    JoinAccept(join_accept::JoinAccept),
    /// A new player joined
    PlayerJoined(player_joined::PlayerJoined),
}
