// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;

impl NetworkHandler {
    /// Shutdowns the network handler closing all connections and channels.
    pub async fn shutdown(self) {
        let connections = self.connections.clone();
        let clients = self.get_clients().await;
        for client in clients {
            Self::remove_client(connections.clone(), client, 0x100, b"shutting down").await;
        }
    }
}
