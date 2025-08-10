// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Event
//! This module contains all types used for the communication from the server
//! to the client.

mod join_accept;
mod player_joined;

pub use join_accept::JoinAccept;
pub use player_joined::PlayerJoined;

use crate::Targetable;

/// Message from the server, to the client
#[derive(
    serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq, Clone, derive_more::TryFrom,
)]
#[enum_dispatch::enum_dispatch]
#[non_exhaustive]
pub enum EventKind {
    /// Gets send when a new player joins
    JoinAccept(JoinAccept),
    /// A new player joined
    PlayerJoined(PlayerJoined),
}

/// Each event needs to have this trait
#[enum_dispatch::enum_dispatch(EventKind)]
pub trait Event: Targetable {}
