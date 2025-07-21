// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `Config`
//! Defines the `Config` struct used for the game server. Will be read from
//! a TOML file and be used as a resource in bevy.

/// The main `Config` struct used to configure the server.
#[derive(Debug, Resource)]
pub struct Config {
    /// Maximum amount of players on the server
    pub max_players: u32,
}
