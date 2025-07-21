// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! `NetworkConfig`
//! `NetworkConfig` struct for settings used by the network systems.

use std::{net::SocketAddr, path::PathBuf};

/// `NetworkConfig` struct for setting concerning the network systems
#[derive(Debug, serde::Deserialize, serde::Serialize, clap::Args)]
pub struct NetworkConfig {
    /// Socket to bind the server to
    #[arg(long)]
    pub socket: SocketAddr,
    /// Path to the TLS certificates (self- or externally-signed)
    #[arg(long)]
    pub certs: PathBuf,
    /// Path to the TLS private key (self- or externally-signed)
    #[arg(long)]
    pub key: PathBuf,
}

impl Default for NetworkConfig {
    #[expect(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            socket: "0.0.0.0".parse().unwrap(),
            certs: "certs.pem".parse().unwrap(),
            key: "key.pem".parse().unwrap(),
        }
    }
}
