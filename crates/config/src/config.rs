// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `Config`
//! Defines the `Config` struct used for the game server. Will be read from
//! a TOML file and be used as a resource in bevy.

pub mod logging;
pub mod network;

use crate::config::{logging::LoggingConfig, network::NetworkConfig};
use bevy::ecs::resource::Resource;

/// The main `Config` struct used to configure the server.
#[derive(Debug, Resource, serde::Deserialize, konfik::Config)]
pub struct Config {
    /// Maximum amount of players on the server
    pub max_players: u32,
    /// Network settings
    pub network: NetworkConfig,
    /// Logging config
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_players: 100,
            network: NetworkConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}
