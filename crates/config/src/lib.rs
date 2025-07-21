// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Config
//! Defines the config struct used for the game server. Will be read from
//! a TOML file and the CLI can override those settings.

mod config;
pub use config::Config;

mod cli;
pub use cli::Cli;
