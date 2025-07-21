// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `CLI`
//! Defines the `CLI` struct used to provided the TOML config file path
//! and to overwrite settings from it. Will be read from

use crate::config::network::NetworkConfig;
use std::path::PathBuf;

/// CLI struct to provide the config file and overwrite settings from it.
#[derive(Debug, clap::Parser, serde::Serialize)]
#[command(author = "Crypts of the Lost")]
pub struct Cli {
    /// The path to the config file
    #[arg(short, long)]
    pub config: Option<PathBuf>, // ignored by figment
    /// Network config
    #[command(flatten)]
    pub network: NetworkConfig,
}
