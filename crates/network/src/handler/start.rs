// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use crate::error::HandlerError;
use quinn::Endpoint;
use tracing::{error, info};

impl NetworkHandler {
    /// Starts the network handler and begins to listen for new connections
    ///
    /// This method initializes the QUIC endpoint, starts accepting connections,
    /// and spawns tasks for handling outbound messages and connection management.
    ///
    /// # Errors
    /// Returns an error if the endpoint cannot be created or bound to the socket.
    #[tracing::instrument]
    pub async fn start(&mut self) -> Result<(), HandlerError> {
        info!("starting the networkhandler and listening to connections");

        let endpoint = Endpoint::server(self.server_config.clone(), self.socket)?;
        self.endpoint = Some(endpoint.clone());

        while let Some(incoming) = endpoint.accept().await {
            let Ok(connection) = incoming.await else {
                error!("Error accepting incoming connection");
                continue;
            };
            let addr = connection.remote_address();
            info!("new connection with {addr}");
            self.add_client(addr, connection.clone());

            let tx = self.inbound_tx.clone();
            let rx = self.broadcast.subscribe();
            let connections = self.connections.clone();

            tokio::spawn(async move {
                Self::handle_connection(connection, connections, tx, rx).await;
            });
        }

        Ok(())
    }
}
