// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use quinn::VarInt;
use tracing::info;

impl NetworkHandler {
    /// Shutdowns the network handler closing all connections and channels.
    pub fn shutdown(&mut self) {
        info!("shutting down network handler");
        self.connections
            .iter()
            .for_each(|item| item.close(VarInt::from_u32(0x100), b"shutting down"));
        if let Some(endpoint) = &self.endpoint {
            endpoint.close(VarInt::from_u32(0x100), b"shutting down");
        }
        self.endpoint = None;
    }
}

impl Drop for NetworkHandler {
    fn drop(&mut self) {
        self.shutdown();
    }
}
