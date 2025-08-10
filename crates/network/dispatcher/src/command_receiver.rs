// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `CommandReceiver`
//! Stores the rx from the networkhandler

use bevy::ecs::{event::EventWriter, resource::Resource, system::ResMut};
use protocol::command::{CommandKind, join::Join};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(Debug, Resource)]
pub struct CommandReceiver {
    pub rx: UnboundedReceiver<CommandKind>,
}

pub fn process_incoming_commands(mut recv: ResMut<CommandReceiver>, mut join: EventWriter<Join>) {
    while let Ok(cmd) = recv.rx.try_recv() {
        #[expect(clippy::single_match)]
        match cmd {
            CommandKind::Join(data) => {
                join.write(data);
            }
            _ => {}
        }
    }
}
