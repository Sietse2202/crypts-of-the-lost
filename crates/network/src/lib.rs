// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Network
//! This crate handles all networking code between the client and the server.

#![expect(clippy::multiple_crate_versions)]

mod bridge;
mod cert;
mod error;
mod handler;
mod setup;

pub use cert::Certs;
pub use error::{CertsError, HandlerError};
pub use handler::NetworkHandler;

use bevy::app::{Plugin, Startup, Update};
use bridge::{process_incoming_commands, process_outbound_events};
use setup::setup;

/// Network plugin which starts the `NetworkHandler` and
/// the dispatchers.
#[derive(Debug)]
pub struct Network;

impl Plugin for Network {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (process_incoming_commands, process_outbound_events));
    }
}
