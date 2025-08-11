// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Setup
//! This module handles setting up the `NetworkHandler` and start listening
//! new connections.

use crate::{
    Certs, NetworkHandler,
    bridge::{CommandReceiver, EventSender},
};
use bevy::ecs::system::{Commands, Res};
use config::Config;
use protocol::{command::CommandKind, event::EventKind};
use tracing::{error, info};

#[expect(clippy::expect_used)]
pub fn setup(mut commands: Commands, config: Res<Config>) {
    info!("Setting up network");

    let (inbound_tx, inbound_rx) = tokio::sync::mpsc::unbounded_channel::<CommandKind>();
    let (outbound_tx, outbound_rx) = tokio::sync::mpsc::unbounded_channel::<EventKind>();

    let certs = Certs::read_from_file(&config.network.certs, &config.network.key)
        .expect("A TLS certificate and private key (self- or externally-signed) are required to start a server.");

    let server_config = certs
        .create_server_config()
        .expect("Wasn't able to create the ServerConfig");

    let mut handler = NetworkHandler::new(
        config.network.socket,
        server_config,
        outbound_rx,
        inbound_tx,
    );

    tokio::spawn(async move {
        if let Err(e) = handler.start().await {
            error!("Failed to start network handler: {e}");
            panic!("Server cannot continue without a network handler")
        }
    });

    commands.insert_resource(CommandReceiver { rx: inbound_rx });
    commands.insert_resource(EventSender { tx: outbound_tx });
}
