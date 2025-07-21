// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Parse

use crate::{Cli, Config};
use clap::Parser;
use figment::{
    Figment, Profile,
    providers::{Format, Serialized, Toml},
};

/// Parses the `Config` struct.
///
/// Parses the config struct by using the default values as the base
/// and then merging the TOML file and the cli args on top of it.
///
/// # Errors
///
/// Returns an error when deserialization to the `Config` struct failed.
#[expect(clippy::result_large_err)]
pub fn parse_config() -> Result<Config, figment::Error> {
    let cli = Cli::parse();

    let mut figment = Figment::new().merge(Serialized::defaults(Config::default()));

    if let Some(path) = &cli.config {
        figment = figment.merge(Toml::file(path));
    }

    figment = figment.merge(Serialized::from(cli, Profile::Default));

    figment.extract()
}
