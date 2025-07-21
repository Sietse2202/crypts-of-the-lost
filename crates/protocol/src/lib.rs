// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # Protocol
//! Defines the communication between the server and client.

#![expect(clippy::multiple_crate_versions)]

use bevy::app::Plugin;

pub mod command;
pub mod event;
pub mod target;

/// bevy plugin for the protocol
///
/// adds all events and commands as bevy events
#[derive(Debug)]
pub struct Protocol;

impl Plugin for Protocol {
    fn build(&self, app: &mut bevy::app::App) {
        // Command events
        app.add_event::<command::join::Join>();

        // Event events
        app.add_event::<event::join_accept::JoinAccept>()
            .add_event::<event::player_joined::PlayerJoined>();
    }
}
