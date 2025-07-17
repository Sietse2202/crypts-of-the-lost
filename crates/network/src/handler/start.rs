// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use super::NetworkHandler;
use crate::error::HandlerError;
use quinn::Endpoint;

impl NetworkHandler {
    /// Starts the network handler and begins to listen for new connections
    ///
    /// This method initializes the QUIC endpoint, starts accepting connections,
    /// and spawns tasks for handling outbound messages and connection management.
    ///
    /// # Errors
    /// Returns an error if the endpoint cannot be created or bound to the socket.
    pub fn start(&mut self) -> Result<(), HandlerError> {
        let endpoint = Endpoint::server(self.server_config.clone(), self.socket)?;
        self.endpoint = Some(endpoint);

        Ok(())
    }
}
