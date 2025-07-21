// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use dashmap::DashMap;
use quinn::Connection;
use std::{collections::HashSet, net::SocketAddr, sync::Arc};

impl NetworkHandler {
    /// Adds a new client connection to the handler
    pub(super) fn add_client(&self, addr: SocketAddr, conn: Connection) {
        self.connections.insert(addr, conn);
    }

    /// Removes a client connection from the handler
    pub(super) fn remove_client(
        connections: &Arc<DashMap<SocketAddr, Connection>>,
        addr: SocketAddr,
        error_code: u32,
        reason: &[u8],
    ) {
        let Some((_, connection)) = connections.remove(&addr) else {
            return;
        };
        connection.close(error_code.into(), reason);
    }

    /// Gets all currently connected client addresses
    #[must_use]
    pub fn get_clients(&self) -> HashSet<SocketAddr> {
        self.connections.iter().map(|item| *item.key()).collect()
    }
}
