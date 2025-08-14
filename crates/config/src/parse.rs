// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Parse

use crate::Config;
use konfik::{ConfigLoader, Error};

/// Parses the `Config` struct.
///
/// Parses the config struct by using the default values as the base
/// and then merging the TOML file and the cli args on top of it.
///
/// # Errors
///
/// Returns an error when deserialization to the `Config` struct failed.
pub fn parse_config() -> Result<Config, Error> {
    ConfigLoader::default()
        .with_config_file("config.toml")
        .with_env_prefix("COTL")
        .with_cli()
        .load()
}
