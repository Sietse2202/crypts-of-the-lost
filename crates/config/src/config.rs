// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `Config`
//! Defines the `Config` struct used for the game server. Will be read from
//! a TOML file and be used as a resource in bevy.

use bevy::ecs::resource::Resource;
use std::net::SocketAddr;

/// The main `Config` struct used to configure the server.
#[derive(Debug, Resource, serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// Maximum amount of players on the server
    pub max_players: u32,
    /// Socket to bind the server to
    pub socket: SocketAddr,
}

impl Default for Config {
    #[expect(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            max_players: 100,
            socket: "127.0.0.1:42069".parse().unwrap(),
        }
    }
}
