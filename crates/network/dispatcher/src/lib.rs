// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! This crate handles all networking code between the client and the server, this also
//! specifies the protocol they use to communicate.

#![expect(clippy::multiple_crate_versions)]

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::system::{Commands, Res},
};

mod command_receiver;

use command_receiver::{CommandReceiver, process_incoming_commands};
use config::Config;
use handler::{Certs, NetworkHandler};
use protocol::{command::CommandKind, event::EventKind};
use tracing::info;

/// Network plugin which starts the `NetworkHandler` and
/// the dispatchers.
#[derive(Debug)]
pub struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, process_incoming_commands);
    }
}

#[expect(unused)]
#[expect(clippy::expect_used)]
#[expect(clippy::needless_pass_by_value)]
fn setup(mut commands: Commands, config: Res<Config>) {
    info!("Setting up network");

    let (inbound_tx, inbound_rx) = tokio::sync::mpsc::unbounded_channel::<CommandKind>();
    let (outbound_tx, outbound_rx) = tokio::sync::mpsc::unbounded_channel::<EventKind>();

    let certs = Certs::read_from_file(&config.network.certs, &config.network.key)
        .expect("A TLS certificate and private key (self- or externally-signed) are required to start a server.");

    let server_config = certs
        .create_server_config()
        .expect("Wasn't able to create the ServerConfig");

    let handler = NetworkHandler::new(
        config.network.socket,
        server_config,
        outbound_rx,
        inbound_tx,
    );

    commands.insert_resource(CommandReceiver { rx: inbound_rx });
}
