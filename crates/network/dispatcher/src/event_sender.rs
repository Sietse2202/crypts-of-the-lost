// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

//! # `EventSender`
//! Stores the tx to the networkhandler

use bevy::ecs::{event::EventReader, resource::Resource, system::Res};
use protocol::event::{EventKind, JoinAccept, PlayerJoined};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug, Resource)]
pub struct EventSender {
    pub tx: UnboundedSender<EventKind>,
}

macro_rules! send_all_events {
    ($sender:expr, $($reader:expr),+ $(,)?) => {
        $(send_events($sender, $reader);)+
    };
}

#[expect(clippy::needless_pass_by_value)]
pub fn process_outbound_events(
    sender: Res<EventSender>,
    mut join_accept: EventReader<JoinAccept>,
    mut player_joined: EventReader<PlayerJoined>,
) {
    send_all_events!(&sender.tx, &mut join_accept, &mut player_joined);
}

fn send_events<T>(sender: &UnboundedSender<EventKind>, reader: &mut EventReader<T>)
where
    T: Clone + Into<EventKind> + bevy::prelude::Event + protocol::Event,
{
    for event in reader.read() {
        let _ = sender.send(event.clone().into());
    }
}
