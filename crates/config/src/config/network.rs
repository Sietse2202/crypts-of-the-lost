// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! `NetworkConfig`
//! `NetworkConfig` struct for settings used by the network systems.

use std::{net::SocketAddr, path::PathBuf};

/// `NetworkConfig` struct for setting concerning the network systems
#[derive(Debug, Clone, serde::Deserialize)]
pub struct NetworkConfig {
    /// Socket to bind the server to
    pub socket: SocketAddr,
    /// Path to the TLS certificates (self- or externally-signed)
    pub certs: PathBuf,
    /// Path to the TLS private key (self- or externally-signed)
    pub key: PathBuf,
}

impl Default for NetworkConfig {
    #[expect(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            socket: "0.0.0.0:1234".parse().unwrap(),
            certs: "certs.pem".parse().unwrap(),
            key: "key.pem".parse().unwrap(),
        }
    }
}
