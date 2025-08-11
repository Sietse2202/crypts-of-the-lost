// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Config
//! Defines the config struct used for the game server. Will be read from
//! a TOML file and the CLI can override those settings.

#![expect(clippy::multiple_crate_versions)]

pub mod config;
pub use config::Config;

mod cli;
use cli::Cli;

mod parse;
pub use parse::parse_config;
