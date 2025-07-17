// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use quinn::Connection;
use std::{collections::HashSet, net::SocketAddr};

impl NetworkHandler {
    /// Adds a new client connection to the handler
    pub(super) async fn add_client(&self, addr: SocketAddr, conn: Connection) {
        let mut connections = self.connections.write().await;
        connections.insert(addr, conn);
    }

    /// Removes a client connection from the handler
    pub(super) async fn remove_client(&self, addr: SocketAddr) {
        let mut connections = self.connections.write().await;
        connections.remove(&addr);
    }

    /// Gets all currently connected client addresses
    pub async fn get_clients(&self) -> HashSet<SocketAddr> {
        let connections = self.connections.read().await;
        connections.keys().copied().collect()
    }
}
