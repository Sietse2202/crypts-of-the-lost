// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Dispatcher
//! This crate handles all networking code between the client and the server, this also
//! specifies the protocol they use to communicate.

#![expect(clippy::multiple_crate_versions)]

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::system::Commands,
};

mod command_receiver;
use command_receiver::process_incoming_commands;
use protocol::{command::Command, event::Event};
use tracing::info;

use crate::command_receiver::CommandReceiver;

/// Dispatcher plugin for ease of use
#[derive(Debug)]
pub struct Dispatcher;

impl Plugin for Dispatcher {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, process_incoming_commands);
    }
}

#[expect(unused)]
fn setup(mut commands: Commands) {
    info!("Setting up network");

    let (inbound_tx, inbound_rx) = tokio::sync::mpsc::unbounded_channel::<Command>();
    let (outbound_tx, outbound_rx) = tokio::sync::mpsc::unbounded_channel::<Event>();

    commands.insert_resource(CommandReceiver { rx: inbound_rx });
}
